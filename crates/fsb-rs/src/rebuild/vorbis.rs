//! FMOD Vorbis packet reconstruction and Ogg repackaging.

use std::collections::HashMap;
use std::sync::OnceLock;

use base64::Engine as _;
use ogg::{PacketWriteEndInfo, PacketWriter};

use super::error::RebuildError;
use crate::{ChunkData, ChunkType, Sample};

const SERIAL: u32 = 1;
const SETUP_JSON: &str = include_str!("vorbis_headers_converted.json");

#[derive(Debug)]
struct SetupRecord {
    header: Vec<u8>,
    seek_bit: usize,
}

static SETUPS: OnceLock<HashMap<u32, SetupRecord>> = OnceLock::new();

fn setups() -> &'static HashMap<u32, SetupRecord> {
    SETUPS.get_or_init(|| {
        let root: serde_json::Value =
            serde_json::from_str(SETUP_JSON).expect("embedded Vorbis setup JSON is valid");
        root.as_object()
            .expect("embedded Vorbis setup JSON is an object")
            .iter()
            .map(|(crc, value)| {
                let crc = crc.parse::<u32>().expect("Vorbis setup CRC is a u32");
                let header = value["headerBytes"]
                    .as_str()
                    .and_then(|encoded| {
                        base64::engine::general_purpose::STANDARD
                            .decode(encoded)
                            .ok()
                    })
                    .expect("Vorbis setup header is base64");
                let seek_bit = value["seekBit"]
                    .as_u64()
                    .and_then(|value| usize::try_from(value).ok())
                    .expect("Vorbis setup seekBit fits usize");
                (crc, SetupRecord { header, seek_bit })
            })
            .collect()
    })
}

struct BitReader<'a> {
    bytes: &'a [u8],
    bit: usize,
}

impl<'a> BitReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, bit: 0 }
    }

    fn seek(&mut self, bit: usize) -> Result<(), RebuildError> {
        if bit > self.bytes.len().saturating_mul(8) {
            return Err(RebuildError::InvalidVorbisData(
                "setup bit offset is out of bounds",
            ));
        }
        self.bit = bit;
        Ok(())
    }

    fn read_bits(&mut self, count: usize) -> Result<u32, RebuildError> {
        if count > 32
            || self
                .bit
                .checked_add(count)
                .is_none_or(|end| end > self.bytes.len().saturating_mul(8))
        {
            return Err(RebuildError::InvalidVorbisData("truncated bit stream"));
        }

        let mut value = 0u32;
        for shift in 0..count {
            let byte = self.bytes[self.bit / 8];
            value |= u32::from((byte >> (self.bit % 8)) & 1) << shift;
            self.bit += 1;
        }
        Ok(value)
    }
}

fn mode_block_flags(setup: &SetupRecord) -> Result<Vec<bool>, RebuildError> {
    if setup.header.get(..7) != Some(b"\x05vorbis") {
        return Err(RebuildError::InvalidVorbisData(
            "invalid setup packet signature",
        ));
    }

    let mut bits = BitReader::new(&setup.header);
    bits.seek(setup.seek_bit)?;
    let mode_count = bits.read_bits(6)? as usize + 1;
    let mut flags = Vec::with_capacity(mode_count);
    for _ in 0..mode_count {
        flags.push(bits.read_bits(1)? != 0);
        bits.read_bits(16)?;
        bits.read_bits(16)?;
        bits.read_bits(8)?;
    }
    Ok(flags)
}

fn packet_block_size(packet: &[u8], flags: &[bool]) -> Result<u32, RebuildError> {
    if flags.is_empty() {
        return Err(RebuildError::InvalidVorbisData("setup has no modes"));
    }
    let mut bits = BitReader::new(packet);
    if bits.read_bits(1)? != 0 {
        return Err(RebuildError::InvalidVorbisData(
            "non-audio packet in sample data",
        ));
    }
    let mode_bits = usize::try_from(usize::BITS - (flags.len() - 1).leading_zeros())
        .map_err(|_| RebuildError::InvalidVorbisData("invalid mode count"))?;
    let mode = bits.read_bits(mode_bits)? as usize;
    match flags.get(mode) {
        Some(true) => Ok(2048),
        Some(false) => Ok(256),
        None => Err(RebuildError::InvalidVorbisData(
            "audio packet mode is out of bounds",
        )),
    }
}

fn audio_packets(mut data: &[u8]) -> Result<Vec<&[u8]>, RebuildError> {
    let mut packets = Vec::new();
    while data.len() >= 2 {
        let size = u16::from_le_bytes([data[0], data[1]]) as usize;
        data = &data[2..];
        if size == 0 || size == u16::MAX as usize {
            break;
        }
        if size > data.len() {
            return Err(RebuildError::InvalidVorbisData("truncated audio packet"));
        }
        packets.push(&data[..size]);
        data = &data[size..];
    }
    if packets.is_empty() {
        return Err(RebuildError::InvalidVorbisData(
            "sample has no audio packets",
        ));
    }
    Ok(packets)
}

fn identification_packet(sample: &Sample<'_>) -> Result<Vec<u8>, RebuildError> {
    let channels = u8::try_from(sample.metadata.channels)
        .ok()
        .filter(|channels| *channels != 0)
        .ok_or(RebuildError::InvalidChannelCount(sample.metadata.channels))?;
    let mut packet = Vec::with_capacity(30);
    packet.extend_from_slice(b"\x01vorbis");
    packet.extend_from_slice(&0u32.to_le_bytes());
    packet.push(channels);
    packet.extend_from_slice(&sample.metadata.frequency().to_le_bytes());
    packet.extend_from_slice(&[0; 12]);
    packet.push(0xb8);
    packet.push(1);
    Ok(packet)
}

fn comment_packet() -> Vec<u8> {
    const VENDOR: &[u8] = b"Fmod5Sharp (Samboy063)";
    let mut packet = Vec::with_capacity(16 + VENDOR.len());
    packet.extend_from_slice(b"\x03vorbis");
    packet.extend_from_slice(&(VENDOR.len() as u32).to_le_bytes());
    packet.extend_from_slice(VENDOR);
    packet.extend_from_slice(&0u32.to_le_bytes());
    packet.push(1);
    packet
}

pub(crate) fn rebuild(sample: &Sample<'_>) -> Result<Vec<u8>, RebuildError> {
    let crc = sample
        .metadata
        .chunks
        .iter()
        .find(|chunk| chunk.chunk_type == ChunkType::VorbisData)
        .and_then(|chunk| match chunk.data {
            ChunkData::Vorbis { crc32 } => Some(crc32),
            _ => None,
        })
        .ok_or(RebuildError::MissingVorbisData)?;
    let setup = setups()
        .get(&crc)
        .ok_or(RebuildError::UnknownVorbisSetup(crc))?;
    let flags = mode_block_flags(setup)?;
    let packets = audio_packets(sample.sample_bytes)?;

    let mut writer = PacketWriter::new(Vec::new());
    writer
        .write_packet(
            identification_packet(sample)?,
            SERIAL,
            PacketWriteEndInfo::EndPage,
            0,
        )
        .map_err(|_| RebuildError::OggWrite)?;
    writer
        .write_packet(
            comment_packet(),
            SERIAL,
            PacketWriteEndInfo::NormalPacket,
            0,
        )
        .map_err(|_| RebuildError::OggWrite)?;
    writer
        .write_packet(
            setup.header.as_slice(),
            SERIAL,
            PacketWriteEndInfo::EndPage,
            0,
        )
        .map_err(|_| RebuildError::OggWrite)?;

    let mut previous_block_size = 0u32;
    let mut granule = 0u64;
    let sample_count = u64::from(sample.metadata.sample_count);
    for (index, packet) in packets.iter().enumerate() {
        let block_size = packet_block_size(packet, &flags)?;
        if previous_block_size != 0 {
            granule = granule
                .checked_add(u64::from(block_size + previous_block_size) / 4)
                .ok_or(RebuildError::OutputTooLarge)?
                .min(sample_count);
        }
        previous_block_size = block_size;

        let is_end = index + 1 == packets.len() || granule == sample_count;
        writer
            .write_packet(
                *packet,
                SERIAL,
                if is_end {
                    PacketWriteEndInfo::EndStream
                } else {
                    PacketWriteEndInfo::NormalPacket
                },
                granule,
            )
            .map_err(|_| RebuildError::OggWrite)?;
        if is_end {
            break;
        }
    }

    Ok(writer.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_reader_is_lsb_first() {
        let mut reader = BitReader::new(&[0b1011_0010]);
        assert_eq!(reader.read_bits(4).unwrap(), 0b0010);
        assert_eq!(reader.read_bits(4).unwrap(), 0b1011);
    }

    #[test]
    fn packet_parser_rejects_truncation() {
        assert!(matches!(
            audio_packets(&[3, 0, 1, 2]),
            Err(RebuildError::InvalidVorbisData(_))
        ));
    }
}
