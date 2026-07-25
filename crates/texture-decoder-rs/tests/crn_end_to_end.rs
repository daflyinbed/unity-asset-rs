//! End-to-end tests for the Crunch (CRN) decoder against reference fixtures.
//!
//! The `.crn`/`.bc` pairs under `tests/data/crunch/` were produced by
//! `tools/gen_crn_golden.cpp` (see `tools/gen_crn_golden.sh`), which compresses
//! small source images with the `vendor/crunch` CLI and then transcodes
//! every face and mip level to raw BC blocks using the reference
//! `crn_decomp.h`. The tests are hermetic: they never invoke the reference
//! project at run time.

#![cfg(feature = "crn")]

use std::fs;
use std::path::Path;

use texture_decoder_rs::crn::{CrnFormat, decode};

const DATA_DIR: &str = "tests/data/crunch";

fn fixture(name: &str) -> (Vec<u8>, Vec<u8>) {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let base = Path::new(manifest).join(DATA_DIR);
    let crn = fs::read(base.join(format!("{name}.crn"))).unwrap();
    let bc = fs::read(base.join(format!("{name}.bc"))).unwrap();
    (crn, bc)
}

fn assert_cubemap_fixture(name: &str, expected_format: CrnFormat, block_bytes: usize) {
    let (crn, golden) = fixture(name);
    let expected_levels = [(6, 6), (3, 3), (1, 1)];
    let expected_face_size: usize = expected_levels
        .iter()
        .map(|&(width, height)| {
            (width as usize).div_ceil(4) * (height as usize).div_ceil(4) * block_bytes
        })
        .sum();
    assert_eq!(golden.len(), 6 * expected_face_size);

    let texture = decode(&crn).expect("cubemap decode");
    assert_eq!(texture.format, expected_format);
    assert_eq!((texture.width, texture.height), (6, 6));
    assert_eq!(texture.faces.len(), 6);

    let mut golden_offset = 0;
    for (face_index, face) in texture.faces.iter().enumerate() {
        assert_eq!(
            face.levels.len(),
            expected_levels.len(),
            "face {face_index} mip count"
        );
        for (level_index, (level, &(width, height))) in
            face.levels.iter().zip(expected_levels.iter()).enumerate()
        {
            let expected_len =
                (width as usize).div_ceil(4) * (height as usize).div_ceil(4) * block_bytes;
            let golden_end = golden_offset + expected_len;
            assert_eq!(
                (level.width, level.height),
                (width, height),
                "face {face_index} mip {level_index} dimensions"
            );
            assert_eq!(
                level.data.len(),
                expected_len,
                "face {face_index} mip {level_index} block byte count"
            );
            assert_eq!(
                level.data,
                golden[golden_offset..golden_end],
                "face {face_index} mip {level_index} must match the reference transcoder"
            );
            golden_offset = golden_end;
        }
    }
    assert_eq!(golden_offset, golden.len());
}

#[test]
fn decodes_dxt1_fixture_to_reference_blocks() {
    let (crn, golden) = fixture("dxt1-6x6");
    // 6x6 image => ceil(6/4) x ceil(6/4) = 2x2 blocks, 8 bytes each.
    assert_eq!(golden.len(), 2 * 2 * 8);

    let texture = decode(&crn).expect("DXT1 decode");
    assert_eq!(texture.format, CrnFormat::Bc1);
    assert_eq!((texture.width, texture.height), (6, 6));
    assert_eq!(texture.faces.len(), 1);
    assert_eq!(texture.faces[0].levels.len(), 1);
    let level = &texture.faces[0].levels[0];
    assert_eq!((level.width, level.height), (6, 6));
    assert_eq!(level.data.len(), golden.len());
    assert_eq!(
        level.data, golden,
        "decoded BC1 blocks must match the reference transcoder"
    );
}

#[test]
fn decodes_dxt5_fixture_to_reference_blocks() {
    let (crn, golden) = fixture("dxt5-6x6");
    // 6x6 image => 2x2 blocks, 16 bytes each.
    assert_eq!(golden.len(), 2 * 2 * 16);

    let texture = decode(&crn).expect("DXT5 decode");
    assert_eq!(texture.format, CrnFormat::Bc3);
    assert_eq!((texture.width, texture.height), (6, 6));
    assert_eq!(texture.faces.len(), 1);
    assert_eq!(texture.faces[0].levels.len(), 1);
    let level = &texture.faces[0].levels[0];
    assert_eq!((level.width, level.height), (6, 6));
    assert_eq!(level.data.len(), golden.len());
    assert_eq!(
        level.data, golden,
        "decoded BC3 blocks must match the reference transcoder"
    );
}

#[test]
fn decodes_all_dxt1_cubemap_faces_and_mips() {
    assert_cubemap_fixture("dxt1-cubemap-6x6", CrnFormat::Bc1, 8);
}

#[test]
fn decodes_all_dxt5_cubemap_faces_and_mips() {
    assert_cubemap_fixture("dxt5-cubemap-6x6", CrnFormat::Bc3, 16);
}

#[test]
fn rejects_cubemap_when_a_later_mip_range_is_missing() {
    let (mut crn, _golden) = fixture("dxt1-cubemap-6x6");
    let data_size = u32::from_be_bytes(crn[6..10].try_into().unwrap());

    // Keep the complete level-0 range and the preceding offsets intact, but
    // move level 2's start to EOF so that its range is empty. A decoder that
    // silently stopped after level 0 would accept this; full-texture decode
    // must reject the missing later subresources.
    crn[78..82].copy_from_slice(&data_size.to_be_bytes());

    assert_eq!(
        decode(&crn).unwrap_err(),
        texture_decoder_rs::crn::CrnDecodeError::InvalidHeader
    );
}

#[test]
fn accepts_zero_levels_like_unchecked_reference_unpack() {
    let (mut crn, golden) = fixture("dxt1-6x6");
    crn[16] = 0;

    let texture = decode(&crn).expect("zero-level header follows reference level-0 path");
    assert_eq!(texture.faces[0].levels[0].data, golden);
}

#[test]
fn clamps_zero_dimensions_to_one_like_reference_unpack() {
    let (mut crn, golden) = fixture("dxt5-6x6");
    crn[12..16].fill(0);

    let texture = decode(&crn).expect("zero dimensions clamp to one");
    assert_eq!((texture.width, texture.height), (1, 1));
    assert_eq!(
        (
            texture.faces[0].levels[0].width,
            texture.faces[0].levels[0].height
        ),
        (1, 1)
    );
    assert_eq!(texture.faces[0].levels[0].data, golden[..16]);
}

#[test]
fn accepts_in_buffer_level_data_with_segmented_flag() {
    let (mut crn, golden) = fixture("dxt1-6x6");
    crn[20] |= 1;

    let texture = decode(&crn).expect("ordinary reference path ignores segmented flag");
    assert_eq!(texture.faces[0].levels[0].data, golden);
}

#[test]
fn reports_segmented_level_when_data_is_out_of_line() {
    let (mut crn, _golden) = fixture("dxt1-6x6");
    crn[20] |= 1;
    let level_start = u32::from_be_bytes(crn[70..74].try_into().unwrap()) as usize;
    crn[6..10].copy_from_slice(&(level_start as u32).to_be_bytes());
    crn.truncate(level_start);

    let err = decode(&crn).unwrap_err();
    assert_eq!(
        err,
        texture_decoder_rs::crn::CrnDecodeError::UnsupportedSegmented
    );
}

#[test]
fn accepts_color_endpoint_palette_larger_than_8192() {
    let (mut crn, golden) = fixture("dxt1-6x6");
    // m_color_endpoints.m_num is a big-endian u16 at bytes 39..41. Crunch
    // accepts 8193 entries and zero-fills the exhausted palette bitstream.
    crn[39..41].copy_from_slice(&8193u16.to_be_bytes());

    let texture = decode(&crn).expect("8193-entry endpoint palette");
    assert_eq!(texture.faces[0].levels[0].data, golden);
}

#[test]
fn rejects_bad_signature() {
    // A valid DXT1 fixture with the first two bytes corrupted.
    let (mut crn, _golden) = fixture("dxt1-6x6");
    crn[0] = 0x00;
    crn[1] = 0x00;
    let err = decode(&crn).unwrap_err();
    assert!(matches!(
        err,
        texture_decoder_rs::crn::CrnDecodeError::InvalidSignature
    ));
}

#[test]
fn rejects_truncated_input() {
    let err = decode(&[0x48, 0x78]).unwrap_err();
    assert!(matches!(
        err,
        texture_decoder_rs::crn::CrnDecodeError::InputTooShort
    ));
}
