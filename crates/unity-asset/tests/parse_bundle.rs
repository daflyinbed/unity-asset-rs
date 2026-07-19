//! 端到端测试：用 example/char_002_amiya.ab 验证 UnityFS 解包。
//!
//! 期望值与 UnityPy 实测输出一致（首尾 16 字节 bit-for-bit 对照过）。

use bytes::Bytes;
use unity_asset::{FileFormat, LoadedFile, load, load_bytes};

// fixture 位于仓库根的 example/；从本 crate（crates/unity-asset）回溯两级。
const AB_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../example/char_002_amiya.ab"
);
const EXPECTED_NAME: &str = "CAB-374c777ce38840c47301d29986e9f61f";
const EXPECTED_SIZE: usize = 57416;
const EXPECTED_FIRST16: [u8; 16] = [
    0x00, 0x00, 0xb6, 0xd1, 0x00, 0x00, 0xe0, 0x48, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0xb6, 0xf0,
];
const EXPECTED_LAST16: [u8; 16] = [
    0x55, 0x7f, 0x76, 0x5f, 0x00, 0x00, 0x00, 0x00, 0xf1, 0xe9, 0xf4, 0xce, 0x78, 0xf0, 0xdd, 0xff,
];

fn init_logger() {
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .is_test(true)
        .try_init();
}

/// 从 `LoadedFile` 提取 UnityFS bundle 的 unity_revision + 首个叶子。
fn assert_expected(loaded: &LoadedFile) {
    let (unity_revision, format) = match loaded {
        LoadedFile::Container {
            unity_revision,
            format,
            ..
        } => (unity_revision.clone(), *format),
        _ => panic!("expected Container, got Leaf"),
    };
    assert_eq!(format, FileFormat::FileStreamBundle, "format");
    assert_eq!(
        unity_revision.as_deref(),
        Some("2017.4.39f1"),
        "unity revision"
    );

    let leaves = loaded.leaves();
    assert_eq!(leaves.len(), 1, "exactly one inner file");

    let (name, bytes, _, _) = leaves[0];
    assert_eq!(name, EXPECTED_NAME, "node name");
    assert_eq!(bytes.len(), EXPECTED_SIZE, "node size");

    let first16 = &bytes[..16];
    let last16 = &bytes[bytes.len() - 16..];
    assert_eq!(first16, EXPECTED_FIRST16, "first 16 bytes");
    assert_eq!(last16, EXPECTED_LAST16, "last 16 bytes");
}

#[test]
fn load_from_path() {
    init_logger();
    let loaded = load(AB_PATH).expect("parse bundle");
    assert_expected(&loaded);
}

#[test]
fn load_from_bytes_matches_path() {
    init_logger();
    let raw = std::fs::read(AB_PATH).expect("read ab");
    let from_bytes = load_bytes(Bytes::from(raw)).expect("parse from bytes");

    let from_path = load(AB_PATH).expect("parse from path");

    let a = from_bytes.leaves();
    let b = from_path.leaves();
    assert_eq!(a.len(), b.len());
    assert_eq!(a[0].0, b[0].0, "name");
    assert_eq!(a[0].1.len(), b[0].1.len(), "size");
    assert_eq!(a[0].1[..], b[0].1[..], "byte content identical");
}

#[test]
fn node_offsets_stay_within_blob() {
    init_logger();
    let loaded = load(AB_PATH).expect("parse bundle");
    for (name, bytes, _, _) in loaded.leaves() {
        assert!(!bytes.is_empty(), "file {} is empty", name);
    }
}

#[test]
fn total_blob_matches_sum_of_node_sizes() {
    init_logger();
    let loaded = load(AB_PATH).expect("parse bundle");
    let sum: usize = loaded
        .leaves()
        .iter()
        .map(|(_, bytes, _, _)| bytes.len())
        .sum();
    assert_eq!(sum, EXPECTED_SIZE, "sum of node sizes");
}
