use std::fs;
use std::path::{Path, PathBuf};

use texture_decoder_rs::{
    DecodeError,
    etc::{
        decode_eac_r11, decode_eac_rg11, decode_eac_signed_r11, decode_eac_signed_rg11,
        decode_etc1_rgb8, decode_etc2_rgb8, decode_etc2_rgb8a1, decode_etc2_rgba8,
    },
};

type Decoder = fn(&[u8], usize, usize, &mut [u8]) -> Result<(), DecodeError>;
type Golden = fn(&str) -> Vec<u8>;
type ColorCase = (&'static str, &'static str, Decoder, Golden);

fn etcpack_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/etcpack")
}

fn read_pkm(name: &str) -> (u16, usize, usize, Vec<u8>) {
    let data = fs::read(etcpack_root().join("pkm").join(name)).unwrap();
    assert_eq!(&data[..4], b"PKM ");
    let format = u16::from_be_bytes([data[6], data[7]]);
    let width = usize::from(u16::from_be_bytes([data[12], data[13]]));
    let height = usize::from(u16::from_be_bytes([data[14], data[15]]));
    (format, width, height, data[16..].to_vec())
}

fn pnm_pixels(name: &str) -> (usize, usize, u16, Vec<u8>) {
    let data = fs::read(etcpack_root().join("golden").join(name)).unwrap();
    let mut newlines = data.iter().enumerate().filter(|(_, byte)| **byte == b'\n');
    let first = newlines.next().unwrap().0;
    let second = newlines.next().unwrap().0;
    let third = newlines.next().unwrap().0;
    let dimensions = core::str::from_utf8(&data[first + 1..second]).unwrap();
    let mut dimensions = dimensions.split_ascii_whitespace();
    let width = dimensions.next().unwrap().parse().unwrap();
    let height = dimensions.next().unwrap().parse().unwrap();
    let maximum = core::str::from_utf8(&data[second + 1..third])
        .unwrap()
        .parse()
        .unwrap();
    (width, height, maximum, data[third + 1..].to_vec())
}

fn rgb_golden(name: &str) -> Vec<u8> {
    let (width, height, maximum, pixels) = pnm_pixels(name);
    assert_eq!(maximum, 255);
    assert_eq!(pixels.len(), width * height * 3);
    let mut output = Vec::with_capacity(width * height * 4);
    for pixel in pixels.chunks_exact(3) {
        output.extend_from_slice(&[pixel[0], pixel[1], pixel[2], 255]);
    }
    output
}

fn rgba_golden(name: &str) -> Vec<u8> {
    let data = fs::read(etcpack_root().join("golden").join(name)).unwrap();
    assert_eq!(data[2], 2, "expected an uncompressed true-color TGA");
    let width = usize::from(u16::from_le_bytes([data[12], data[13]]));
    let height = usize::from(u16::from_le_bytes([data[14], data[15]]));
    assert_eq!(data[16], 32);
    let top_origin = data[17] & 0x20 != 0;
    let pixels = &data[18..];
    assert_eq!(pixels.len(), width * height * 4);
    let mut output = vec![0_u8; pixels.len()];
    for y in 0..height {
        let source_y = if top_origin { y } else { height - 1 - y };
        for x in 0..width {
            let source = (source_y * width + x) * 4;
            let destination = (y * width + x) * 4;
            output[destination..destination + 4].copy_from_slice(&[
                pixels[source + 2],
                pixels[source + 1],
                pixels[source],
                pixels[source + 3],
            ]);
        }
    }
    output
}

fn eac_golden(name: &str, channels: usize, signed: bool) -> Vec<u8> {
    let (width, height, maximum, pixels) = pnm_pixels(name);
    assert_eq!(maximum, 65535);
    let stored_channels = if channels == 1 { 1 } else { 3 };
    assert_eq!(pixels.len(), width * height * stored_channels * 2);
    let mut output = Vec::with_capacity(width * height * 4);
    for pixel in pixels.chunks_exact(stored_channels * 2) {
        let map = |channel: usize| {
            let value = i32::from(u16::from_be_bytes([
                pixel[channel * 2],
                pixel[channel * 2 + 1],
            ]));
            if signed {
                let expanded = value - 32768;
                let eleven_bit = expanded.signum() * (expanded.abs() >> 5);
                (((eleven_bit + 1023) * 255 + 1023) / 2046).clamp(0, 255) as u8
            } else {
                let eleven_bit = value >> 5;
                ((eleven_bit * 255 + 1023) / 2047) as u8
            }
        };
        output.extend_from_slice(&[map(0), if channels == 2 { map(1) } else { 0 }, 0, 255]);
    }
    output
}

fn assert_decodes(pkm: &str, golden: &str, decoder: Decoder, expected: fn(&str) -> Vec<u8>) {
    let (_, width, height, payload) = read_pkm(pkm);
    let expected = expected(golden);
    assert_eq!(expected.len(), width * height * 4);
    let mut actual = vec![0_u8; expected.len()];
    decoder(&payload, width, height, &mut actual).unwrap();
    assert_eq!(actual, expected, "fixture {pkm}");
}

#[test]
fn decodes_etcpack_color_vectors_byte_exactly() {
    let cases: [ColorCase; 7] = [
        (
            "elina_etc1.pkm",
            "elina_etc1.ppm",
            decode_etc1_rgb8,
            rgb_golden,
        ),
        ("elina.pkm", "elina.ppm", decode_etc2_rgb8, rgb_golden),
        (
            "elina_sRGB.pkm",
            "elina_sRGB.ppm",
            decode_etc2_rgb8,
            rgb_golden,
        ),
        (
            "elina_alpha1.pkm",
            "elina_alpha1.tga",
            decode_etc2_rgb8a1,
            rgba_golden,
        ),
        (
            "elina_sRGBA1.pkm",
            "elina_sRGBA1.tga",
            decode_etc2_rgb8a1,
            rgba_golden,
        ),
        (
            "elina_alpha.pkm",
            "elina_alpha.tga",
            decode_etc2_rgba8,
            rgba_golden,
        ),
        (
            "elina_alpha_sRGBA.pkm",
            "elina_alpha_sRGBA.tga",
            decode_etc2_rgba8,
            rgba_golden,
        ),
    ];
    for (pkm, golden, decoder, expected) in cases {
        assert_decodes(pkm, golden, decoder, expected);
    }
}

#[test]
fn decodes_etcpack_eac_vectors_byte_exactly() {
    let cases: [(&str, &str, Decoder, usize, bool); 4] = [
        (
            "conftestimage_R11_EAC.pkm",
            "conftestimage_R11_EAC.pgm",
            decode_eac_r11,
            1,
            false,
        ),
        (
            "conftestimage_RG11_EAC.pkm",
            "conftestimage_RG11_EAC.ppm",
            decode_eac_rg11,
            2,
            false,
        ),
        (
            "conftestimage_SIGNED_R11_EAC.pkm",
            "conftestimage_SIGNED_R11_EAC.pgm",
            decode_eac_signed_r11,
            1,
            true,
        ),
        (
            "conftestimage_SIGNED_RG11_EAC.pkm",
            "conftestimage_SIGNED_RG11_EAC.ppm",
            decode_eac_signed_rg11,
            2,
            true,
        ),
    ];
    for (pkm, golden, decoder, channels, signed) in cases {
        let (_, width, height, payload) = read_pkm(pkm);
        let expected = eac_golden(golden, channels, signed);
        let mut actual = vec![0_u8; expected.len()];
        decoder(&payload, width, height, &mut actual).unwrap();
        if actual != expected {
            let index = actual
                .iter()
                .zip(&expected)
                .position(|(actual, expected)| actual != expected)
                .unwrap();
            panic!(
                "fixture {pkm}, byte {index}: actual {}, expected {}",
                actual[index], expected[index]
            );
        }
    }
}
