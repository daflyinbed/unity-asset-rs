//! Structural golden-value tests for the FSB5 container parser.
//!
//! These fixtures are copied from Fmod5Sharp's TestResources and cover every
//! codec the original library supports at the container level. Expected values
//! were derived by running this parser against the fixtures and cross-checking
//! the FSB5 header bytes, then frozen.

use fsb_rs::{AudioType, ChunkData, ChunkType, FsbError};

/// Load a fixture from `tests/fixtures/`.
fn fixture(name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    std::fs::read(&path).unwrap_or_else(|e| panic!("failed to read fixture {name}: {e}"))
}

// ---------- Shared invariants ----------

/// Invariants that hold for every well-formed bank, regardless of codec.
fn assert_common_invariants(bank: &fsb_rs::SoundBank<'_>, bytes: &[u8]) {
    assert_eq!(
        bank.samples.len(),
        bank.header.num_samples as usize,
        "num_samples should match the sample count"
    );

    let total_sample_bytes: usize = bank.samples.iter().map(|s| s.sample_bytes.len()).sum();
    assert_eq!(
        total_sample_bytes, bank.header.size_of_data as usize,
        "summed sample_bytes should equal the bank's declared data size"
    );

    // Every sample's bytes must live inside the input buffer.
    let base = bytes.as_ptr();
    let end = unsafe { base.add(bytes.len()) };
    for s in &bank.samples {
        let p = s.sample_bytes.as_ptr();
        assert!(p >= base && unsafe { p.add(s.sample_bytes.len()) } <= end);
    }
}

// ---------- Per-fixture golden values ----------

#[test]
fn parses_short_vorbis() {
    let bytes = fixture("short_vorbis.fsb");
    let bank = fsb_rs::parse(&bytes).expect("short_vorbis should parse");
    assert_common_invariants(&bank, &bytes);

    assert_eq!(bank.header.version, 1);
    assert_eq!(bank.header.num_samples, 1);
    assert_eq!(bank.header.audio_type, AudioType::Vorbis);

    let s = &bank.samples[0];
    assert_eq!(s.metadata.channels, 1);
    assert_eq!(s.metadata.frequency(), 44_100);
    assert_eq!(s.metadata.sample_count, 24_806);
    assert_eq!(s.sample_bytes.len(), 5_632);
    // This fixture has no name table.
    assert_eq!(s.name, None);

    // Must have a VORBISDATA chunk carrying the crc32 lookup key.
    let vorbis = s
        .metadata
        .chunks
        .iter()
        .find(|c| c.chunk_type == ChunkType::VorbisData)
        .expect("vorbis sample should have a VORBISDATA chunk");
    match &vorbis.data {
        ChunkData::Vorbis { crc32 } => assert_eq!(*crc32, 0xd6e0bbd4),
        other => panic!("expected Vorbis chunk data, got {other:?}"),
    }
}

#[test]
fn parses_long_vorbis() {
    let bytes = fixture("long_vorbis.fsb");
    let bank = fsb_rs::parse(&bytes).expect("long_vorbis should parse");
    assert_common_invariants(&bank, &bytes);
    assert_eq!(bank.header.audio_type, AudioType::Vorbis);
    assert_eq!(bank.samples.len(), 1);
    let s = &bank.samples[0];
    assert_eq!(s.metadata.channels, 1);
    assert_eq!(s.metadata.frequency(), 44_100);
    assert_eq!(s.metadata.sample_count, 725_038);
    // Same setup header as short_vorbis (same crc32).
    let crc = vorbis_crc32(&bank.samples[0]);
    assert_eq!(crc, Some(0xd6e0bbd4));
}

#[test]
fn parses_vorbis_with_blockflag_exception() {
    let bytes = fixture("vorbis_with_blockflag_exception.fsb");
    let bank = fsb_rs::parse(&bytes).expect("blockflag fixture should parse");
    assert_common_invariants(&bank, &bytes);
    assert_eq!(bank.header.audio_type, AudioType::Vorbis);
    let s = &bank.samples[0];
    assert_eq!(s.metadata.frequency(), 8_000);
    assert_eq!(vorbis_crc32(s), Some(0x9f62e5dd));
}

#[test]
fn parses_previously_unrecoverable_vorbis() {
    let bytes = fixture("previously_unrecoverable_vorbis.fsb");
    let bank = fsb_rs::parse(&bytes).expect("previously_unrecoverable should parse");
    assert_common_invariants(&bank, &bytes);
    assert_eq!(bank.header.audio_type, AudioType::Vorbis);
    let s = &bank.samples[0];
    assert_eq!(s.metadata.frequency(), 22_050);
    assert_eq!(vorbis_crc32(s), Some(0x8d00698d));
}

#[test]
fn parses_pcm16() {
    let bytes = fixture("pcm16.fsb");
    let bank = fsb_rs::parse(&bytes).expect("pcm16 should parse");
    assert_common_invariants(&bank, &bytes);
    assert_eq!(bank.header.audio_type, AudioType::Pcm16);
    assert_eq!(bank.samples.len(), 1);
    let s = &bank.samples[0];
    assert_eq!(s.metadata.channels, 1);
    assert_eq!(s.metadata.frequency(), 96_000); // freq_id 10 -> 96 kHz
    assert_eq!(s.metadata.sample_count, 29_482);
    assert!(s.metadata.chunks.is_empty(), "pcm16 carries no chunks");
}

#[test]
fn parses_gcadpcm() {
    let bytes = fixture("gcadpcm.fsb");
    let bank = fsb_rs::parse(&bytes).expect("gcadpcm should parse");
    assert_common_invariants(&bank, &bytes);
    assert_eq!(bank.header.audio_type, AudioType::Gcadpcm);
    assert_eq!(bank.samples.len(), 1);

    let s = &bank.samples[0];
    assert_eq!(s.metadata.channels, 1);
    // Frequency chunk overrides the packed id; here 11025 Hz (id outside table).
    assert_eq!(s.metadata.frequency(), 11_025);
    assert_eq!(s.metadata.frequency_id, 11_025);

    // Two chunks: Frequency (more=true) then DspCoeff (more=false).
    assert_eq!(s.metadata.chunks.len(), 2);
    assert!(matches!(
        s.metadata.chunks[0].chunk_type,
        ChunkType::Frequency
    ));
    assert!(
        s.metadata.chunks[0].more,
        "Frequency chunk signals more follow"
    );
    match &s.metadata.chunks[0].data {
        ChunkData::Frequency { frequency_id } => assert_eq!(*frequency_id, 11_025),
        other => panic!("expected Frequency, got {other:?}"),
    }
    assert!(matches!(
        s.metadata.chunks[1].chunk_type,
        ChunkType::DspCoeff
    ));
    assert!(!s.metadata.chunks[1].more);
    match &s.metadata.chunks[1].data {
        ChunkData::DspCoefficients { channel_data } => {
            assert_eq!(channel_data.len(), 1, "one coefficient set per channel");
            assert_eq!(channel_data[0].len(), 16, "16 coefficients per channel");
        }
        other => panic!("expected DspCoefficients, got {other:?}"),
    }
}

#[test]
fn parses_imaadpcm_short() {
    let bytes = fixture("imaadpcm_short.fsb");
    let bank = fsb_rs::parse(&bytes).expect("imaadpcm_short should parse");
    assert_common_invariants(&bank, &bytes);
    assert_eq!(bank.header.audio_type, AudioType::ImaAdpcm);
    let s = &bank.samples[0];
    assert_eq!(s.metadata.channels, 2);
    assert_eq!(s.metadata.frequency(), 44_100);
    // Exercises the Unknown-chunk fallback path (a Comment chunk).
    assert!(matches!(
        s.metadata.chunks[0].chunk_type,
        ChunkType::Comment
    ));
    match &s.metadata.chunks[0].data {
        ChunkData::Unknown(v) => assert_eq!(v.len(), 260),
        other => panic!("expected Unknown, got {other:?}"),
    }
}

#[test]
fn parses_imaadpcm_long() {
    let bytes = fixture("imaadpcm_long.fsb");
    let bank = fsb_rs::parse(&bytes).expect("imaadpcm_long should parse");
    assert_common_invariants(&bank, &bytes);
    assert_eq!(bank.header.audio_type, AudioType::ImaAdpcm);
    let s = &bank.samples[0];
    assert_eq!(s.metadata.channels, 2);
    assert_eq!(s.metadata.frequency(), 48_000);
    assert_eq!(s.metadata.sample_count, 121_088);
    assert!(s.metadata.chunks.is_empty(), "no chunks declared");
}

#[test]
fn parses_xbox_imaad() {
    let bytes = fixture("xbox_imaad.fsb");
    let bank = fsb_rs::parse(&bytes).expect("xbox_imaad should parse");
    assert_common_invariants(&bank, &bytes);
    assert_eq!(bank.header.audio_type, AudioType::ImaAdpcm);
    // Xbox IMA is the mono variant in the original library.
    for s in &bank.samples {
        assert_eq!(s.metadata.channels, 1, "xbox_imaad is mono");
    }
}

fn vorbis_crc32(s: &fsb_rs::Sample<'_>) -> Option<u32> {
    s.metadata
        .chunks
        .iter()
        .find(|c| c.chunk_type == ChunkType::VorbisData)
        .and_then(|c| match &c.data {
            ChunkData::Vorbis { crc32 } => Some(*crc32),
            _ => None,
        })
}

// ---------- Name-table coverage (synthetic) ----------
//
// None of the upstream fixtures ship a name table, so build a minimal FSB5
// stream by hand to exercise that code path.

#[test]
fn reads_sample_names_from_name_table() {
    let bytes = build_minimal_bank_with_names();
    let bank = fsb_rs::parse(&bytes).expect("synthetic bank parses");
    assert_eq!(bank.samples.len(), 2);
    assert_eq!(bank.samples[0].name.as_deref(), Some("alpha"));
    assert_eq!(bank.samples[1].name.as_deref(), Some("beta"));
    // name_table region was non-empty.
    assert!(bank.header.size_of_name_table > 0);
}

/// Hand-build a 2-sample version-1 FSB5 bank with a name table and tiny payloads.
fn build_minimal_bank_with_names() -> Vec<u8> {
    let mut out = Vec::new();

    // --- Header (0x3C for version 1) ---
    out.extend_from_slice(b"FSB5");
    out.extend_from_slice(&1u32.to_le_bytes()); // version
    out.extend_from_slice(&2u32.to_le_bytes()); // num_samples
    // size_of_sample_headers: 2 samples * 8 bytes metadata, no chunks.
    let size_of_sample_headers: u32 = 2 * 8;
    out.extend_from_slice(&size_of_sample_headers.to_le_bytes());

    // Name table layout: [u32 offset_alpha][u32 offset_beta] then the names.
    // Offsets are relative to the base = header_size + size_of_sample_headers.
    let name_offsets_base = 2 * 4; // two u32 entries
    let alpha_off = name_offsets_base;
    let beta_off = alpha_off + "alpha".len() as u32 + 1; // +1 for NUL
    let name_table: Vec<u8> = {
        let mut v = Vec::new();
        v.extend_from_slice(&alpha_off.to_le_bytes());
        v.extend_from_slice(&beta_off.to_le_bytes());
        v.extend_from_slice(b"alpha\0");
        v.extend_from_slice(b"beta\0");
        v
    };
    let size_of_name_table = name_table.len() as u32;
    out.extend_from_slice(&size_of_name_table.to_le_bytes());

    // Two zero-byte payloads -> size_of_data = 0.
    let size_of_data: u32 = 0;
    out.extend_from_slice(&size_of_data.to_le_bytes());
    out.extend_from_slice(&2u32.to_le_bytes()); // audio_type = PCM16 (discriminant 2)
    out.extend_from_slice(&0u32.to_le_bytes()); // 0x1C always 0
    out.extend_from_slice(&0u32.to_le_bytes()); // flags
    out.extend_from_slice(&0u64.to_le_bytes()); // hash lower
    out.extend_from_slice(&0u64.to_le_bytes()); // hash upper
    out.extend_from_slice(&0u64.to_le_bytes()); // unknown u64
    debug_assert_eq!(out.len(), 0x3C, "version-1 header is 0x3C bytes");

    // --- Sample metadata words (no chunks) ---
    // Both samples share data_offset=0; size_of_data=0 means zero-length slices.
    for _ in 0..2 {
        // has_any_chunks=0, freq_id=8, channels=1, data_offset=0, sample_count=0
        let mut encoded = 0u64;
        encoded |= 8 << 1; // freq_id 8 at bits 1-4
        // channel code 0 (1 ch) at bits 5-6, data_offset 0, sample_count 0.
        out.extend_from_slice(&encoded.to_le_bytes());
    }

    // --- Name table ---
    out.extend_from_slice(&name_table);

    // --- Data region (empty) ---
    out
}

// ---------- Error cases ----------

#[test]
fn rejects_bad_magic() {
    let bad = b"NOT FSB AT ALL extra bytes here";
    let err = fsb_rs::parse(bad).unwrap_err();
    assert!(matches!(err, FsbError::BadMagic), "got {err:?}");
}

#[test]
fn rejects_truncated_input() {
    let bytes = fixture("pcm16.fsb");
    let truncated = &bytes[..6]; // keep magic, truncate after
    let err = fsb_rs::parse(truncated).unwrap_err();
    assert!(matches!(err, FsbError::UnexpectedEof), "got {err:?}");
}

#[test]
fn empty_input_is_unexpected_eof() {
    let err = fsb_rs::parse(b"").unwrap_err();
    assert!(matches!(err, FsbError::UnexpectedEof), "got {err:?}");
}

#[test]
fn rejects_sample_count_larger_than_header_region() {
    let mut bytes = build_minimal_bank_with_names();
    bytes[8..12].copy_from_slice(&u32::MAX.to_le_bytes());

    let err = fsb_rs::parse(&bytes).unwrap_err();
    assert!(
        matches!(err, FsbError::BadSampleHeaderBounds),
        "got {err:?}"
    );
}

#[test]
fn chunks_cannot_escape_sample_header_region() {
    let mut bytes = build_minimal_bank_with_names();
    // Claim that the first sample has chunks. The following bytes do not contain
    // a complete chunk inside the declared sample-header region.
    bytes[0x3c] |= 1;

    let err = fsb_rs::parse(&bytes).unwrap_err();
    assert!(matches!(err, FsbError::UnexpectedEof), "got {err:?}");
}

#[test]
fn rejects_name_offset_outside_name_table_without_panicking() {
    let mut bytes = build_minimal_bank_with_names();
    let first_name_offset = 0x3c + 16;
    bytes[first_name_offset..first_name_offset + 4].copy_from_slice(&u32::MAX.to_le_bytes());

    let err = fsb_rs::parse(&bytes).unwrap_err();
    assert!(matches!(err, FsbError::BadNameBounds), "got {err:?}");
}

#[test]
fn rejects_sample_offset_outside_declared_data_region() {
    let mut bytes = build_minimal_bank_with_names();
    // The second sample starts 32 bytes into a declared zero-byte data region.
    let second_metadata = 0x3c + 8;
    let mut encoded = u64::from_le_bytes(
        bytes[second_metadata..second_metadata + 8]
            .try_into()
            .unwrap(),
    );
    encoded |= 1 << 7;
    bytes[second_metadata..second_metadata + 8].copy_from_slice(&encoded.to_le_bytes());
    // Appended bytes must not make the invalid declared offset acceptable.
    bytes.extend_from_slice(&[0xaa; 64]);

    let err = fsb_rs::parse(&bytes).unwrap_err();
    assert!(matches!(err, FsbError::BadSampleBounds), "got {err:?}");
}
