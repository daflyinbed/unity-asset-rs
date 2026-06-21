use std::path::PathBuf;

use unity_asset_rs::{load, Error};

fn main() -> Result<(), Error> {
    env_logger::init();

    let path = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("example/char_002_amiya.ab"));

    let bundle = load(&path)?;

    println!(
        "bundle: format v{}, engine {}",
        bundle.format_version, bundle.unity_revision
    );
    println!("files ({}):", bundle.files.len());
    let mut total = 0u64;
    for f in &bundle.files {
        total += f.bytes.len() as u64;
        println!(
            "  {:>10} bytes  flags=0x{:08x}  {}",
            f.bytes.len(),
            f.flags,
            f.name
        );
    }
    println!("total uncompressed: {} bytes", total);
    Ok(())
}
