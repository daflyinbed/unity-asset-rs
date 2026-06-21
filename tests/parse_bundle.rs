//! 端到端测试：用 example/char_002_amiya.ab 验证 UnityFS 解包。
//!
//! 期望值与 UnityPy 实测输出一致（首尾 16 字节 bit-for-bit 对照过）。

use bytes::Bytes;
use unity_asset_rs::{load, load_bytes, Bundle};

const AB_PATH: &str = "example/char_002_amiya.ab";
const EXPECTED_NAME: &str = "CAB-374c777ce38840c47301d29986e9f61f";
const EXPECTED_SIZE: usize = 57416;
const EXPECTED_FLAGS: u32 = 4;
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

fn assert_expected(b: &Bundle) {
    assert_eq!(b.format_version, 6, "format version");
    assert_eq!(b.unity_revision, "2017.4.39f1", "unity revision");
    assert_eq!(b.files.len(), 1, "exactly one inner file");

    let f = &b.files[0];
    assert_eq!(f.name, EXPECTED_NAME, "node name");
    assert_eq!(f.bytes.len(), EXPECTED_SIZE, "node size");
    assert_eq!(f.flags, EXPECTED_FLAGS, "node flags");

    let first16 = &f.bytes[..16];
    let last16 = &f.bytes[f.bytes.len() - 16..];
    assert_eq!(first16, EXPECTED_FIRST16, "first 16 bytes");
    assert_eq!(last16, EXPECTED_LAST16, "last 16 bytes");
}

#[test]
fn load_from_path() {
    init_logger();
    let bundle = load(AB_PATH).expect("parse bundle");
    assert_expected(&bundle);
}

#[test]
fn load_from_bytes_matches_path() {
    init_logger();
    let raw = std::fs::read(AB_PATH).expect("read ab");
    let from_bytes = load_bytes(Bytes::from(raw)).expect("parse from bytes");

    let from_path = load(AB_PATH).expect("parse from path");

    assert_eq!(from_bytes.files.len(), from_path.files.len());
    let a = &from_bytes.files[0];
    let b = &from_path.files[0];
    assert_eq!(a.name, b.name);
    assert_eq!(a.bytes.len(), b.bytes.len());
    assert_eq!(&a.bytes[..], &b.bytes[..], "byte content identical");
}

#[test]
fn node_offsets_stay_within_blob() {
    init_logger();
    let bundle = load(AB_PATH).expect("parse bundle");
    for f in &bundle.files {
        assert!(!f.bytes.is_empty(), "file {} is empty", f.name);
    }
}

#[test]
fn total_blob_matches_sum_of_node_sizes() {
    init_logger();
    let bundle = load(AB_PATH).expect("parse bundle");
    let sum: usize = bundle.files.iter().map(|f| f.bytes.len()).sum();
    assert_eq!(sum, EXPECTED_SIZE, "sum of node sizes");
}
