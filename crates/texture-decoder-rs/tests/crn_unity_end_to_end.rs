//! End-to-end tests for the Unity-fork Crunch (`crn_unity`) decoder against
//! reference fixtures.
//!
//! The `.crn`/`.bc` pairs under `tests/data/crunch_unity/` were produced by
//! `tools/gen_crn_unity_golden.sh`, which compresses small source images with
//! the `vendor/unity-crunch` CLI (the Unity fork on the `unity` branch)
//! and then transcodes every face and mip level to raw block bytes using the
//! reference `unity-crunch/inc/crn_decomp.h`. The tests are hermetic: they
//! never invoke the reference project at run time.

#![cfg(feature = "crn-unity")]

use std::fs;
use std::path::Path;

use texture_decoder_rs::crn_unity::{CrnUnityFormat, decode};

const DATA_DIR: &str = "tests/data/crunch_unity";

fn fixture(name: &str) -> (Vec<u8>, Vec<u8>) {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let base = Path::new(manifest).join(DATA_DIR);
    let crn = fs::read(base.join(format!("{name}.crn"))).unwrap();
    let bc = fs::read(base.join(format!("{name}.bc"))).unwrap();
    (crn, bc)
}

/// Decodes `name`, then asserts the produced block bytes — concatenated face by
/// face, mip by mip — equal the golden `.bc` exactly.
fn assert_fixture(name: &str, expected_format: CrnUnityFormat, block_bytes: usize) {
    let (crn, golden) = fixture(name);

    let texture = decode(&crn).expect("decode");
    assert_eq!(texture.format, expected_format);

    let mut produced = Vec::new();
    for (face_index, face) in texture.faces.iter().enumerate() {
        for (level_index, level) in face.levels.iter().enumerate() {
            // Sanity: each mip level is a whole number of blocks.
            assert_eq!(
                level.data.len() % block_bytes,
                0,
                "{name} face {face_index} mip {level_index} not a whole block count"
            );
            produced.extend_from_slice(&level.data);
        }
    }
    assert_eq!(
        produced.len(),
        golden.len(),
        "{name}: total produced bytes must match golden"
    );
    assert_eq!(
        produced, golden,
        "{name}: decoded blocks must match the reference transcoder"
    );
}

/// Asserts a three-level, six-face cubemap fixture against the golden bytes,
/// also checking the per-face/per-level dimensions.
fn assert_cubemap_fixture(name: &str, expected_format: CrnUnityFormat, block_bytes: usize) {
    let (crn, golden) = fixture(name);
    let expected_levels = [(6, 6), (3, 3), (1, 1)];
    let expected_face_size: usize = expected_levels
        .iter()
        .map(|&(w, h)| (w as usize).div_ceil(4) * (h as usize).div_ceil(4) * block_bytes)
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
    assert_eq!(texture.format, CrnUnityFormat::Bc1);
    assert_eq!((texture.width, texture.height), (6, 6));
    assert_eq!(texture.faces.len(), 1);
    assert_eq!(texture.faces[0].levels.len(), 1);
    assert_eq!(
        texture.faces[0].levels[0].data, golden,
        "decoded BC1 blocks must match the reference transcoder"
    );
}

#[test]
fn decodes_dxt5_fixture_to_reference_blocks() {
    let (crn, golden) = fixture("dxt5-6x6");
    assert_eq!(golden.len(), 2 * 2 * 16);
    let texture = decode(&crn).expect("DXT5 decode");
    assert_eq!(texture.format, CrnUnityFormat::Bc3);
    assert_eq!((texture.width, texture.height), (6, 6));
    assert_eq!(texture.faces.len(), 1);
    assert_eq!(texture.faces[0].levels.len(), 1);
    assert_eq!(
        texture.faces[0].levels[0].data, golden,
        "decoded BC3 blocks must match the reference transcoder"
    );
}

#[test]
fn decodes_etc1_fixture_to_reference_blocks() {
    // ETC_RGB4 / ETC1, 8 bytes per block.
    assert_fixture("etc1-6x6", CrnUnityFormat::Etc1Rgb4, 8);
}

#[test]
fn decodes_etc2a_fixture_to_reference_blocks() {
    // ETC2_RGBA8, 16 bytes per block.
    assert_fixture("etc2a-6x6", CrnUnityFormat::Etc2Rgba8, 16);
}

#[test]
fn decodes_etc2_fixture_to_reference_blocks() {
    assert_fixture("etc2-6x6", CrnUnityFormat::Etc2Rgb8, 8);
}

#[test]
fn decodes_etc1s_fixture_to_reference_blocks() {
    assert_fixture("etc1s-6x6", CrnUnityFormat::Etc1Rgb4, 8);
}

#[test]
fn decodes_etc2as_fixture_to_reference_blocks() {
    assert_fixture("etc2as-6x6", CrnUnityFormat::Etc2Rgba8, 16);
}

#[test]
fn decodes_all_dxt1_cubemap_faces_and_mips() {
    assert_cubemap_fixture("dxt1-cubemap-6x6", CrnUnityFormat::Bc1, 8);
}

#[test]
fn decodes_all_dxt5_cubemap_faces_and_mips() {
    assert_cubemap_fixture("dxt5-cubemap-6x6", CrnUnityFormat::Bc3, 16);
}

#[test]
fn decodes_all_etc1_cubemap_faces_and_mips() {
    assert_cubemap_fixture("etc1-cubemap-6x6", CrnUnityFormat::Etc1Rgb4, 8);
}

#[test]
fn decodes_all_etc2a_cubemap_faces_and_mips() {
    assert_cubemap_fixture("etc2a-cubemap-6x6", CrnUnityFormat::Etc2Rgba8, 16);
}

#[test]
fn decodes_all_etc2_cubemap_faces_and_mips() {
    assert_cubemap_fixture("etc2-cubemap-6x6", CrnUnityFormat::Etc2Rgb8, 8);
}

#[test]
fn decodes_all_etc1s_cubemap_faces_and_mips() {
    assert_cubemap_fixture("etc1s-cubemap-6x6", CrnUnityFormat::Etc1Rgb4, 8);
}

#[test]
fn decodes_all_etc2as_cubemap_faces_and_mips() {
    assert_cubemap_fixture("etc2as-cubemap-6x6", CrnUnityFormat::Etc2Rgba8, 16);
}

#[test]
fn rejects_bad_signature() {
    let (mut crn, _golden) = fixture("dxt1-6x6");
    crn[0] = 0x00;
    crn[1] = 0x00;
    let err = decode(&crn).unwrap_err();
    assert!(matches!(
        err,
        texture_decoder_rs::crn_unity::CrnUnityDecodeError::InvalidSignature
    ));
}

#[test]
fn rejects_truncated_input() {
    let err = decode(&[0x48, 0x78]).unwrap_err();
    assert!(matches!(
        err,
        texture_decoder_rs::crn_unity::CrnUnityDecodeError::InputTooShort
    ));
}

#[test]
fn rejects_unsupported_format() {
    // DXT3 (format byte 1) is not supported by the Unity decoder here.
    let (mut crn, _golden) = fixture("dxt1-6x6");
    crn[18] = 1;
    let err = decode(&crn).unwrap_err();
    assert!(matches!(
        err,
        texture_decoder_rs::crn_unity::CrnUnityDecodeError::UnsupportedFormat
    ));
}

#[test]
fn clamps_zero_dimensions_to_one_like_reference_unpack() {
    let (mut crn, golden) = fixture("dxt1-6x6");
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
    // 1x1 => one block of 8 bytes.
    assert_eq!(texture.faces[0].levels[0].data, golden[..8]);
}
