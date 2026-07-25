//! End-to-end tests for the BC1–BC7 decoders against Compressonator fixtures.
//!
//! The `.bcn`/`.rgba` pairs under `tests/data/compressonator/` were produced by
//! `tools/gen_bcn_golden.cpp` (see `tools/gen_bcn_golden.sh`), which round-trips
//! known inputs through the adjacent Compressonator block codecs and writes the
//! raw compressed payload plus the tightly packed RGBA8 reference output. The
//! tests are hermetic: they never invoke Compressonator at run time.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use texture_decoder_rs::{
    DecodeError,
    bcn::{
        decode_bc1, decode_bc1_block, decode_bc2, decode_bc2_block, decode_bc3, decode_bc3_block,
        decode_bc4, decode_bc4_block, decode_bc5, decode_bc5_block, decode_bc6h_sf16,
        decode_bc6h_sf16_block, decode_bc6h_uf16, decode_bc6h_uf16_block, decode_bc7,
        decode_bc7_block,
    },
};

const DATA_DIR: &str = "tests/data/compressonator";
const BCDEC_DIR: &str = "tests/data/bcdec";

fn fixture(name: &str) -> (Vec<u8>, Vec<u8>) {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let base = Path::new(manifest).join(DATA_DIR);
    let cmp = fs::read(base.join(format!("{name}.bcn"))).unwrap();
    let rgba = fs::read(base.join(format!("{name}.rgba"))).unwrap();
    assert_eq!(rgba.len(), 64, "{name} golden size");
    (cmp, rgba)
}

fn bcdec_fixture(name: &str) -> (Vec<u8>, Vec<u8>) {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let base = Path::new(manifest).join(BCDEC_DIR);
    let cmp = fs::read(base.join(format!("{name}.bcn"))).unwrap();
    let rgba = fs::read(base.join(format!("{name}.rgba"))).unwrap();
    assert_eq!(cmp.len(), 16, "{name} block size");
    assert_eq!(rgba.len(), 64, "{name} golden size");
    (cmp, rgba)
}

/// Decode every fixture block and compare byte-for-byte against the Compressonator
/// reference output. Covers BC1–BC5, all 14 BC6H modes (UF16 + SF16, including
/// the `-nz` non-zero-data variants), and all 8 BC7 modes.
///
/// One fixture is exempted from the blanket comparison: `bc6h-mode5-nz-uf16`,
/// whose golden output reflects a known Compressonator bug. In BC6H mode 5 the
/// `by[4]` high bit (block bit 40) is read but the result is discarded before
/// assignment (`bc6_encode_kernel.cpp` drops the `(getvalue(40,1)<<4)` term), so
/// a block with bit 40 set decodes differently from the spec. The Rust decoder
/// keeps that bit, so it is checked separately below.
#[test]
fn decodes_all_compressed_blocks_byte_exactly() {
    let mut names: Vec<String> = fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join(DATA_DIR))
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().into_string().unwrap())
        .filter(|n| n.ends_with(".bcn"))
        .map(|n| n.trim_end_matches(".bcn").to_string())
        .collect();
    names.sort();

    // Pin the exact fixture set: a blanket `>= N` threshold would still pass if
    // whole BC6H/BC7 modes went missing from the tree. Listing them by mode makes
    // a dropped `zero`/`-nz`/`uf16`/`sf16` sample fail loudly instead.
    let mut expected: BTreeSet<String> = BTreeSet::new();
    expected.extend(
        [
            "bc1-block",
            "bc2-block",
            "bc3-block",
            "bc4-block",
            "bc5-block",
        ]
        .map(str::to_string),
    );
    for m in 1..=14_u8 {
        for &nz in &[false, true] {
            for &signed in &[false, true] {
                let mut n = format!("bc6h-mode{m}");
                if nz {
                    n.push_str("-nz");
                }
                n.push_str(if signed { "-sf16" } else { "-uf16" });
                expected.insert(n);
            }
        }
    }
    // bc6h-1/bc6h-2 are mode-independent smoke fixtures that predate the
    // per-mode sweep.
    expected
        .extend(["bc6h-1-uf16", "bc6h-1-sf16", "bc6h-2-uf16", "bc6h-2-sf16"].map(str::to_string));
    for m in 0..=7_u8 {
        for &nz in &[false, true] {
            expected.insert(format!("bc7-mode{m}{}", if nz { "-nz" } else { "" }));
        }
    }
    // bc7-block-1/bc7-block-2 are untyped smoke fixtures.
    expected.extend(["bc7-block-1", "bc7-block-2"].map(str::to_string));

    let present: BTreeSet<String> = names.iter().cloned().collect();
    let missing: Vec<&String> = expected.iter().filter(|e| !present.contains(*e)).collect();
    assert!(missing.is_empty(), "missing fixtures: {missing:?}");

    for name in &names {
        // bc6h-mode5-nz-uf16 carries Compressonator's mode-5 bit-40 bug; it is
        // verified by `bc6h_mode5_nonzero_uf16_differs_from_buggy_reference`.
        if name == "bc6h-mode5-nz-uf16" {
            continue;
        }
        let (cmp, golden) = fixture(name);
        let mut out = [0_u8; 64];
        let result = dispatch_block(name, &cmp, &mut out);
        result.unwrap_or_else(|e| panic!("fixture {name}: {e:?}"));
        assert_eq!(out[..], golden[..], "fixture {name}");
    }
}

/// BC6H mode 5 with non-zero data exposes a Compressonator decoder bug: the
/// `by[4]` high bit (block bit 40) is computed but never assigned, so the
/// reference drops it. With the `-nz` block every data bit — including bit 40 —
/// is set, so the omission changes pixel 2's blue channel from the spec's 255 to
/// the reference's 76. This test pins the Rust decoder to the spec-correct value
/// and documents the single-byte divergence, rather than mirroring the bug.
#[test]
fn bc6h_mode5_nonzero_uf16_differs_from_buggy_reference() {
    let (cmp, golden) = fixture("bc6h-mode5-nz-uf16");
    let mut out = [0_u8; 64];
    decode_bc6h_uf16_block(cmp.as_slice().try_into().unwrap(), &mut out).unwrap();
    // Every byte matches the reference except the one affected by the dropped bit.
    for i in 0..64 {
        if i == 10 {
            // Pixel 2, blue channel: Rust keeps bit 40 (255), reference drops it (76).
            assert_eq!(out[i], 255, "pixel 2 blue should keep BC6H mode-5 bit 40");
            assert_eq!(
                golden[i], 76,
                "Compressonator golden must reflect its bit-40 bug"
            );
        } else {
            assert_eq!(out[i], golden[i], "byte {i}");
        }
    }
}

fn dispatch_block(name: &str, cmp: &[u8], out: &mut [u8]) -> Result<(), DecodeError> {
    if name.starts_with("bc1") {
        let block: &[u8; 8] = cmp.try_into().unwrap();
        decode_bc1_block(block, out)
    } else if name.starts_with("bc4") {
        let block: &[u8; 8] = cmp.try_into().unwrap();
        decode_bc4_block(block, out)
    } else if name.starts_with("bc2") {
        decode_bc2_block(cmp.try_into().unwrap(), out)
    } else if name.starts_with("bc3") {
        decode_bc3_block(cmp.try_into().unwrap(), out)
    } else if name.starts_with("bc5") {
        decode_bc5_block(cmp.try_into().unwrap(), out)
    } else if name.contains("uf16") {
        decode_bc6h_uf16_block(cmp.try_into().unwrap(), out)
    } else if name.contains("sf16") {
        decode_bc6h_sf16_block(cmp.try_into().unwrap(), out)
    } else if name.starts_with("bc7") {
        decode_bc7_block(cmp.try_into().unwrap(), out)
    } else {
        unreachable!("unknown fixture {name}")
    }
}

/// Multi-block image decode must match the concatenation of the single-block
/// outputs and clip the right/bottom edges for non-4-multiple dimensions.
#[test]
fn image_decode_matches_block_decode_and_clips_edges() {
    // Tile the bc1-block fixture into a 7x5 image (2x2 blocks with clipping).
    let (cmp, block_rgba) = fixture("bc1-block");
    let mut tiled = Vec::new();
    for _ in 0..4 {
        tiled.extend_from_slice(&cmp);
    }
    let width = 7;
    let height = 5;
    let mut output = vec![0_u8; width * height * 4];
    decode_bc1(&tiled, width, height, &mut output).unwrap();

    for y in 0..height {
        for x in 0..width {
            let local_x = x % 4;
            let local_y = y % 4;
            let src = (local_y * 4 + local_x) * 4;
            let dst = (y * width + x) * 4;
            assert_eq!(
                &output[dst..dst + 4],
                &block_rgba[src..src + 4],
                "pixel ({x},{y})"
            );
        }
    }
}

#[test]
fn image_decode_preserves_extra_output() {
    let (cmp, _) = fixture("bc1-block");
    let mut tiled = Vec::new();
    for _ in 0..4 {
        tiled.extend_from_slice(&cmp);
    }
    let mut output = vec![0xcd_u8; 4 * 4 * 4 + 7];
    decode_bc1(&tiled, 4, 4, &mut output).unwrap();
    assert_eq!(&output[4 * 4 * 4..], &[0xcd; 7]);
}

#[test]
fn image_decode_handles_zero_dimensions() {
    let (cmp, _) = fixture("bc1-block");
    let mut output = vec![];
    decode_bc1(&cmp, 0, 4, &mut output).unwrap();
    decode_bc1(&cmp, 4, 0, &mut output).unwrap();
    // Zero width with an enormous height must return immediately rather than
    // spinning through ~`usize::MAX/4` empty outer-loop iterations.
    decode_bc1(&[], 0, usize::MAX, &mut output).unwrap();
    decode_bc1(&[], usize::MAX, 0, &mut output).unwrap();
}

#[test]
fn image_decode_reports_errors() {
    let (cmp, _) = fixture("bc1-block");
    let mut output = vec![0_u8; 64];
    // Input too short: 4x4 needs 1 block (8 bytes) but we supply 0.
    assert_eq!(
        decode_bc1(&[], 4, 4, &mut output),
        Err(DecodeError::InputTooShort)
    );
    // Output too short.
    let mut short_out = vec![0_u8; 3];
    assert_eq!(
        decode_bc1(&cmp, 4, 4, &mut short_out),
        Err(DecodeError::OutputTooShort)
    );
    // Size overflow on width*height.
    assert_eq!(
        decode_bc1(&cmp, usize::MAX, 2, &mut output),
        Err(DecodeError::SizeOverflow)
    );
}

#[test]
fn block_decode_rejects_short_output_before_writing() {
    let (cmp, _) = fixture("bc7-block-1");
    let mut output = [0xa5_u8; 3];
    assert_eq!(
        decode_bc7_block(cmp.as_slice().try_into().unwrap(), &mut output),
        Err(DecodeError::OutputTooShort)
    );
    assert_eq!(output, [0xa5; 3]);
}

/// BC6H reserved mode and BC7 all-zero block must be rejected.
#[test]
fn invalid_blocks_are_rejected() {
    let mut bc6 = [0_u8; 16];
    bc6[0] = 0b0001_0011; // decvalue 0x13 -> invalid mode
    let mut out = [0_u8; 64];
    assert_eq!(
        decode_bc6h_uf16_block(&bc6, &mut out),
        Err(DecodeError::InvalidBlock)
    );
    let bc7_zero = [0_u8; 16];
    assert_eq!(
        decode_bc7_block(&bc7_zero, &mut out),
        Err(DecodeError::InvalidBlock)
    );
}

/// Every image-level entry point round-trips a single-block image without error.
#[test]
fn all_image_entry_points_accept_single_blocks() {
    let (bc1, _) = fixture("bc1-block");
    let (bc2, _) = fixture("bc2-block");
    let (bc3, _) = fixture("bc3-block");
    let (bc4, _) = fixture("bc4-block");
    let (bc5, _) = fixture("bc5-block");
    let (bc6u, _) = fixture("bc6h-1-uf16");
    let (bc6s, _) = fixture("bc6h-1-sf16");
    let (bc7, _) = fixture("bc7-block-1");

    let mut o = vec![0_u8; 4 * 4 * 4];
    decode_bc1(&bc1, 4, 4, &mut o).unwrap();
    decode_bc2(&bc2, 4, 4, &mut o).unwrap();
    decode_bc3(&bc3, 4, 4, &mut o).unwrap();
    decode_bc4(&bc4, 4, 4, &mut o).unwrap();
    decode_bc5(&bc5, 4, 4, &mut o).unwrap();
    decode_bc6h_uf16(&bc6u, 4, 4, &mut o).unwrap();
    decode_bc6h_sf16(&bc6s, 4, 4, &mut o).unwrap();
    decode_bc7(&bc7, 4, 4, &mut o).unwrap();
}

/// BC6H endpoint interpolation must round to nearest (`(… + 32) >> 6`), matching
/// the spec and bcdec. The Compressonator golden suite never lands on a block
/// whose interpolated weight sums across the `>> 6` rounding boundary, so its
/// fixtures cannot tell the rounding form from the truncating `(… ) >> 6` form.
/// These two fixtures are generated independently from bcdec (`bcdec_bc6h_half`,
/// then the crate's half→RGBA8 mapping) and pick blocks that *do* straddle the
/// boundary: pixel 1 blue in the UF16 block and pixel 11 green in the SF16 block
/// each differ by exactly one between the two forms. The Rust decoder must match
/// bcdec's rounded value. Without `+ 32` this test fails.
#[test]
fn bc6h_interpolation_rounds_to_nearest_matching_bcdec() {
    let (cmp, golden) = bcdec_fixture("bc6h-uf16-lerp-round");
    let mut out = [0_u8; 64];
    decode_bc6h_uf16_block(cmp.as_slice().try_into().unwrap(), &mut out).unwrap();
    // Pixel 1 blue (offset 1*4 + 2 = 6): rounding -> 0xd0, truncation -> 0xcf.
    assert_eq!(out[6], 0xd0, "UF16 pixel 1 blue must round to nearest");
    assert_eq!(
        out[..],
        golden[..],
        "UF16 block must match bcdec byte-for-byte"
    );

    let (cmp, golden) = bcdec_fixture("bc6h-sf16-lerp-round");
    let mut out = [0_u8; 64];
    decode_bc6h_sf16_block(cmp.as_slice().try_into().unwrap(), &mut out).unwrap();
    // Pixel 11 green (offset 11*4 + 1 = 45): rounding -> 0x03, truncation -> 0x02.
    assert_eq!(out[45], 0x03, "SF16 pixel 11 green must round to nearest");
    assert_eq!(
        out[..],
        golden[..],
        "SF16 block must match bcdec byte-for-byte"
    );
}
