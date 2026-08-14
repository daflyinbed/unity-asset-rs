//! Dev helper: dump every fixture's parsed structure to capture golden values.
//!
//! `cargo run --example dump_all`

use std::path::Path;

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut names: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| {
            let e = e.unwrap();
            let n = e.file_name().to_string_lossy().into_owned();
            n.ends_with(".fsb").then_some(n)
        })
        .collect();
    names.sort();

    for name in &names {
        let bytes = std::fs::read(dir.join(name)).unwrap();
        match fsb_rs::parse(&bytes) {
            Ok(bank) => {
                println!("\n=== {name} ({} bytes) ===", bytes.len());
                println!(
                    "  header: version={} num_samples={} audio_type={:?} header_size={} sample_headers={} name_table={} data={}",
                    bank.header.version,
                    bank.header.num_samples,
                    bank.header.audio_type,
                    bank.header.header_size,
                    bank.header.size_of_sample_headers,
                    bank.header.size_of_name_table,
                    bank.header.size_of_data,
                );
                for (i, s) in bank.samples.iter().enumerate() {
                    println!(
                        "  sample[{}]: name={:?} sample_count={} channels={} freq_id={} freq={} data_off={} bytes={}",
                        i,
                        s.name,
                        s.metadata.sample_count,
                        s.metadata.channels,
                        s.metadata.frequency_id,
                        s.metadata.frequency(),
                        s.metadata.data_offset,
                        s.sample_bytes.len(),
                    );
                    for c in &s.metadata.chunks {
                        let detail = match &c.data {
                            fsb_rs::ChunkData::Vorbis { crc32 } => format!("crc32={crc32:#010x}"),
                            fsb_rs::ChunkData::Frequency { frequency_id } => {
                                format!("frequency_id={frequency_id}")
                            }
                            fsb_rs::ChunkData::Channels { num_channels } => {
                                format!("num_channels={num_channels}")
                            }
                            fsb_rs::ChunkData::Loop {
                                loop_start,
                                loop_end,
                            } => format!("loop={loop_start}..{loop_end}"),
                            fsb_rs::ChunkData::DspCoefficients { channel_data } => {
                                format!(
                                    "{}ch x {} coeffs",
                                    channel_data.len(),
                                    channel_data.first().map(|c| c.len()).unwrap_or(0)
                                )
                            }
                            fsb_rs::ChunkData::Unknown(v) => format!("{} unknown bytes", v.len()),
                        };
                        println!(
                            "    chunk: type={:?} more={} {}",
                            c.chunk_type, c.more, detail
                        );
                    }
                }
            }
            Err(e) => println!("\n=== {name}: ERROR {e:?} ==="),
        }
    }
}
