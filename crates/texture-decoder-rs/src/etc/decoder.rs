//! ETC1, ETC2, and EAC physical block decoding.
//!
//! The implementation follows the block layouts in the Khronos ETC2/EAC
//! specification. ETCPACK is used only as a source of conformance fixtures.

use crate::DecodeError;

const BLOCK_WIDTH: usize = 4;
const BLOCK_HEIGHT: usize = 4;
const RGBA_BLOCK_LEN: usize = BLOCK_WIDTH * BLOCK_HEIGHT * 4;

const DISTANCES: [i32; 8] = [3, 6, 11, 16, 23, 32, 41, 64];

// Ordered by the two selector bits as they appear in the block.
const MODIFIERS: [[i32; 4]; 8] = [
    [2, 8, -2, -8],
    [5, 17, -5, -17],
    [9, 29, -9, -29],
    [13, 42, -13, -42],
    [18, 60, -18, -60],
    [24, 80, -24, -80],
    [33, 106, -33, -106],
    [47, 183, -47, -183],
];

const EAC_MODIFIERS: [[i32; 8]; 16] = [
    [-3, -6, -9, -15, 2, 5, 8, 14],
    [-3, -7, -10, -13, 2, 6, 9, 12],
    [-2, -5, -8, -13, 1, 4, 7, 12],
    [-2, -4, -6, -13, 1, 3, 5, 12],
    [-3, -6, -8, -12, 2, 5, 7, 11],
    [-3, -7, -9, -11, 2, 6, 8, 10],
    [-4, -7, -8, -11, 3, 6, 7, 10],
    [-3, -5, -8, -11, 2, 4, 7, 10],
    [-2, -6, -8, -10, 1, 5, 7, 9],
    [-2, -5, -8, -10, 1, 4, 7, 9],
    [-2, -4, -8, -10, 1, 3, 7, 9],
    [-2, -5, -7, -10, 1, 4, 6, 9],
    [-3, -4, -7, -10, 2, 3, 6, 9],
    [-1, -2, -3, -10, 0, 1, 2, 9],
    [-4, -6, -8, -9, 3, 5, 7, 8],
    [-3, -5, -7, -9, 2, 4, 6, 8],
];

#[derive(Clone, Copy)]
enum ColorProfile {
    Etc1,
    Etc2,
    Punchthrough,
}

#[derive(Clone, Copy)]
enum EacProfile {
    Unsigned,
    Signed,
}

macro_rules! image_api_8 {
    ($block_fn:ident, $image_fn:ident, $decoder:expr, $doc:literal) => {
        #[doc = concat!("Decode one 4x4 ", $doc, " block into tightly packed RGBA8.")]
        pub fn $block_fn(block: &[u8; 8], output: &mut [u8]) -> Result<(), DecodeError> {
            check_block_output(output)?;
            $decoder(block, &mut output[..RGBA_BLOCK_LEN])
        }

        #[doc = concat!("Decode a ", $doc, " image into tightly packed RGBA8.")]
        pub fn $image_fn(
            data: &[u8],
            width: usize,
            height: usize,
            output: &mut [u8],
        ) -> Result<(), DecodeError> {
            decode_image::<8>(data, width, height, output, $decoder)
        }
    };
}

macro_rules! image_api_16 {
    ($block_fn:ident, $image_fn:ident, $decoder:expr, $doc:literal) => {
        #[doc = concat!("Decode one 4x4 ", $doc, " block into tightly packed RGBA8.")]
        pub fn $block_fn(block: &[u8; 16], output: &mut [u8]) -> Result<(), DecodeError> {
            check_block_output(output)?;
            $decoder(block, &mut output[..RGBA_BLOCK_LEN])
        }

        #[doc = concat!("Decode a ", $doc, " image into tightly packed RGBA8.")]
        pub fn $image_fn(
            data: &[u8],
            width: usize,
            height: usize,
            output: &mut [u8],
        ) -> Result<(), DecodeError> {
            decode_image::<16>(data, width, height, output, $decoder)
        }
    };
}

image_api_8!(
    decode_etc1_rgb8_block,
    decode_etc1_rgb8,
    decode_etc1_block,
    "ETC1 RGB8"
);
image_api_8!(
    decode_etc2_rgb8_block,
    decode_etc2_rgb8,
    decode_etc2_block,
    "ETC2 RGB8"
);
image_api_8!(
    decode_etc2_rgb8a1_block,
    decode_etc2_rgb8a1,
    decode_punchthrough_block,
    "ETC2 RGB8 punch-through alpha"
);
image_api_16!(
    decode_etc2_rgba8_block,
    decode_etc2_rgba8,
    decode_rgba8_block,
    "ETC2 RGBA8 EAC"
);
image_api_8!(
    decode_eac_r11_block,
    decode_eac_r11,
    decode_r11_block,
    "EAC R11"
);
image_api_8!(
    decode_eac_signed_r11_block,
    decode_eac_signed_r11,
    decode_signed_r11_block,
    "signed EAC R11"
);
image_api_16!(
    decode_eac_rg11_block,
    decode_eac_rg11,
    decode_rg11_block,
    "EAC RG11"
);
image_api_16!(
    decode_eac_signed_rg11_block,
    decode_eac_signed_rg11,
    decode_signed_rg11_block,
    "signed EAC RG11"
);

fn check_block_output(output: &[u8]) -> Result<(), DecodeError> {
    if output.len() < RGBA_BLOCK_LEN {
        Err(DecodeError::OutputTooShort)
    } else {
        Ok(())
    }
}

fn decode_image<const BLOCK_BYTES: usize>(
    data: &[u8],
    width: usize,
    height: usize,
    output: &mut [u8],
    decode: fn(&[u8; BLOCK_BYTES], &mut [u8]) -> Result<(), DecodeError>,
) -> Result<(), DecodeError> {
    // Avoid iterating an enormous number of empty block rows when one
    // dimension is zero (for example width=0, height=usize::MAX).
    if width == 0 || height == 0 {
        return Ok(());
    }

    let output_len = width
        .checked_mul(height)
        .and_then(|value| value.checked_mul(4))
        .ok_or(DecodeError::SizeOverflow)?;
    if output.len() < output_len {
        return Err(DecodeError::OutputTooShort);
    }

    let blocks_x = width.div_ceil(BLOCK_WIDTH);
    let blocks_y = height.div_ceil(BLOCK_HEIGHT);
    let input_len = blocks_x
        .checked_mul(blocks_y)
        .and_then(|value| value.checked_mul(BLOCK_BYTES))
        .ok_or(DecodeError::SizeOverflow)?;
    if data.len() < input_len {
        return Err(DecodeError::InputTooShort);
    }

    let mut block_output = [0_u8; RGBA_BLOCK_LEN];
    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let offset = (by * blocks_x + bx) * BLOCK_BYTES;
            let block = data[offset..offset + BLOCK_BYTES]
                .try_into()
                .map_err(|_| DecodeError::InputTooShort)?;
            decode(block, &mut block_output)?;

            let copy_width = core::cmp::min(BLOCK_WIDTH, width - bx * BLOCK_WIDTH);
            let copy_height = core::cmp::min(BLOCK_HEIGHT, height - by * BLOCK_HEIGHT);
            for y in 0..copy_height {
                let src = y * BLOCK_WIDTH * 4;
                let dst = ((by * BLOCK_HEIGHT + y) * width + bx * BLOCK_WIDTH) * 4;
                output[dst..dst + copy_width * 4]
                    .copy_from_slice(&block_output[src..src + copy_width * 4]);
            }
        }
    }
    Ok(())
}

fn decode_etc1_block(block: &[u8; 8], output: &mut [u8]) -> Result<(), DecodeError> {
    decode_color_block(block, output, ColorProfile::Etc1)
}

fn decode_etc2_block(block: &[u8; 8], output: &mut [u8]) -> Result<(), DecodeError> {
    decode_color_block(block, output, ColorProfile::Etc2)
}

fn decode_punchthrough_block(block: &[u8; 8], output: &mut [u8]) -> Result<(), DecodeError> {
    decode_color_block(block, output, ColorProfile::Punchthrough)
}

fn decode_rgba8_block(block: &[u8; 16], output: &mut [u8]) -> Result<(), DecodeError> {
    let color: &[u8; 8] = block[8..]
        .try_into()
        .map_err(|_| DecodeError::InputTooShort)?;
    decode_color_block(color, output, ColorProfile::Etc2)?;
    let alpha: &[u8; 8] = block[..8]
        .try_into()
        .map_err(|_| DecodeError::InputTooShort)?;
    decode_eac_alpha(alpha, output);
    Ok(())
}

fn decode_r11_block(block: &[u8; 8], output: &mut [u8]) -> Result<(), DecodeError> {
    decode_eac_channels(block, None, output, EacProfile::Unsigned);
    Ok(())
}

fn decode_signed_r11_block(block: &[u8; 8], output: &mut [u8]) -> Result<(), DecodeError> {
    decode_eac_channels(block, None, output, EacProfile::Signed);
    Ok(())
}

fn decode_rg11_block(block: &[u8; 16], output: &mut [u8]) -> Result<(), DecodeError> {
    let r = block[..8]
        .try_into()
        .map_err(|_| DecodeError::InputTooShort)?;
    let g = block[8..]
        .try_into()
        .map_err(|_| DecodeError::InputTooShort)?;
    decode_eac_channels(r, Some(g), output, EacProfile::Unsigned);
    Ok(())
}

fn decode_signed_rg11_block(block: &[u8; 16], output: &mut [u8]) -> Result<(), DecodeError> {
    let r = block[..8]
        .try_into()
        .map_err(|_| DecodeError::InputTooShort)?;
    let g = block[8..]
        .try_into()
        .map_err(|_| DecodeError::InputTooShort)?;
    decode_eac_channels(r, Some(g), output, EacProfile::Signed);
    Ok(())
}

fn decode_color_block(
    block: &[u8; 8],
    output: &mut [u8],
    profile: ColorProfile,
) -> Result<(), DecodeError> {
    let word = u64::from_be_bytes(*block);
    let opaque = bit(word, 33) != 0;
    let punchthrough = matches!(profile, ColorProfile::Punchthrough) && !opaque;

    if !opaque && !matches!(profile, ColorProfile::Punchthrough) {
        decode_individual(word, output);
        return Ok(());
    }

    let base = [bits(word, 5, 63), bits(word, 5, 55), bits(word, 5, 47)];
    let delta = [
        sign3(bits(word, 3, 58)),
        sign3(bits(word, 3, 50)),
        sign3(bits(word, 3, 42)),
    ];
    let second = [
        base[0] as i32 + delta[0],
        base[1] as i32 + delta[1],
        base[2] as i32 + delta[2],
    ];

    if !(0..=31).contains(&second[0]) {
        if matches!(profile, ColorProfile::Etc1) {
            return Err(DecodeError::InvalidBlock);
        }
        decode_t_mode(word, output, punchthrough);
    } else if !(0..=31).contains(&second[1]) {
        if matches!(profile, ColorProfile::Etc1) {
            return Err(DecodeError::InvalidBlock);
        }
        decode_h_mode(word, output, punchthrough);
    } else if !(0..=31).contains(&second[2]) {
        if matches!(profile, ColorProfile::Etc1) {
            return Err(DecodeError::InvalidBlock);
        }
        decode_planar_mode(word, output);
    } else {
        decode_differential(word, output, punchthrough);
    }
    Ok(())
}

fn decode_individual(word: u64, output: &mut [u8]) {
    let colors = [
        [
            expand4(bits(word, 4, 63)),
            expand4(bits(word, 4, 55)),
            expand4(bits(word, 4, 47)),
        ],
        [
            expand4(bits(word, 4, 59)),
            expand4(bits(word, 4, 51)),
            expand4(bits(word, 4, 43)),
        ],
    ];
    decode_subblocks(word, output, colors, false);
}

fn decode_differential(word: u64, output: &mut [u8], punchthrough: bool) {
    let base = [bits(word, 5, 63), bits(word, 5, 55), bits(word, 5, 47)];
    let colors = [
        [expand5(base[0]), expand5(base[1]), expand5(base[2])],
        [
            expand5((base[0] as i32 + sign3(bits(word, 3, 58))) as u32),
            expand5((base[1] as i32 + sign3(bits(word, 3, 50))) as u32),
            expand5((base[2] as i32 + sign3(bits(word, 3, 42))) as u32),
        ],
    ];
    decode_subblocks(word, output, colors, punchthrough);
}

fn decode_subblocks(word: u64, output: &mut [u8], colors: [[u8; 3]; 2], punchthrough: bool) {
    let flip = bit(word, 32) != 0;
    let tables = [bits(word, 3, 39) as usize, bits(word, 3, 36) as usize];
    for y in 0..4 {
        for x in 0..4 {
            let subblock = usize::from(if flip { y >= 2 } else { x >= 2 });
            let selector = selector(word, x, y);
            let transparent = punchthrough && selector == 2;
            let modifier = if punchthrough && (selector == 0 || selector == 2) {
                0
            } else {
                MODIFIERS[tables[subblock]][selector]
            };
            let color = if transparent {
                [0, 0, 0]
            } else {
                add_modifier(colors[subblock], modifier)
            };
            write_pixel(output, x, y, color, if transparent { 0 } else { 255 });
        }
    }
}

fn decode_t_mode(word: u64, output: &mut [u8], punchthrough: bool) {
    let normalized = unstuff_t(word);
    let c0 = [
        expand4(bits(normalized, 4, 58)),
        expand4(bits(normalized, 4, 54)),
        expand4(bits(normalized, 4, 50)),
    ];
    let c1 = [
        expand4(bits(normalized, 4, 46)),
        expand4(bits(normalized, 4, 42)),
        expand4(bits(normalized, 4, 38)),
    ];
    let distance = DISTANCES[bits(normalized, 3, 34) as usize];
    let palette = [
        c0,
        add_modifier(c1, distance),
        c1,
        add_modifier(c1, -distance),
    ];
    decode_palette(word, output, palette, punchthrough);
}

fn decode_h_mode(word: u64, output: &mut [u8], punchthrough: bool) {
    let normalized = unstuff_h(word);
    let c0_444 = bits(normalized, 12, 57);
    let c1_444 = bits(normalized, 12, 45);
    let c0 = [
        expand4(bits(normalized, 4, 57)),
        expand4(bits(normalized, 4, 53)),
        expand4(bits(normalized, 4, 49)),
    ];
    let c1 = [
        expand4(bits(normalized, 4, 45)),
        expand4(bits(normalized, 4, 41)),
        expand4(bits(normalized, 4, 37)),
    ];
    let distance_index = (bits(normalized, 2, 33) << 1) | u32::from(c0_444 >= c1_444);
    let distance = DISTANCES[distance_index as usize];
    let palette = [
        add_modifier(c0, distance),
        add_modifier(c0, -distance),
        add_modifier(c1, distance),
        add_modifier(c1, -distance),
    ];
    decode_palette(word, output, palette, punchthrough);
}

fn decode_palette(word: u64, output: &mut [u8], palette: [[u8; 3]; 4], punchthrough: bool) {
    for y in 0..4 {
        for x in 0..4 {
            let index = selector(word, x, y);
            let transparent = punchthrough && index == 2;
            write_pixel(
                output,
                x,
                y,
                if transparent {
                    [0, 0, 0]
                } else {
                    palette[index]
                },
                if transparent { 0 } else { 255 },
            );
        }
    }
}

fn decode_planar_mode(word: u64, output: &mut [u8]) {
    let normalized = unstuff_planar(word);
    let origin = [
        expand6(bits(normalized, 6, 63)),
        expand7(bits(normalized, 7, 57)),
        expand6(bits(normalized, 6, 50)),
    ];
    let horizontal = [
        expand6(bits(normalized, 6, 44)),
        expand7(bits(normalized, 7, 38)),
        expand6(bits(normalized, 6, 31)),
    ];
    let vertical = [
        expand6(bits(normalized, 6, 25)),
        expand7(bits(normalized, 7, 19)),
        expand6(bits(normalized, 6, 12)),
    ];
    for y in 0..4 {
        for x in 0..4 {
            let mut color = [0_u8; 3];
            for component in 0..3 {
                let o = i32::from(origin[component]);
                let value = (x as i32 * (i32::from(horizontal[component]) - o)
                    + y as i32 * (i32::from(vertical[component]) - o)
                    + 4 * o
                    + 2)
                    >> 2;
                color[component] = clamp_u8(value);
            }
            write_pixel(output, x, y, color, 255);
        }
    }
}

fn decode_eac_alpha(block: &[u8; 8], output: &mut [u8]) {
    let base = i32::from(block[0]);
    let multiplier = i32::from(block[1] >> 4);
    let table = usize::from(block[1] & 0x0f);
    let indices = eac_indices(block);
    for y in 0..4 {
        for x in 0..4 {
            let i = x * 4 + y;
            let selector = ((indices >> (45 - 3 * i)) & 7) as usize;
            let alpha = base + multiplier * EAC_MODIFIERS[table][selector];
            output[(y * 4 + x) * 4 + 3] = clamp_u8(alpha);
        }
    }
}

fn decode_eac_channels(r: &[u8; 8], g: Option<&[u8; 8]>, output: &mut [u8], profile: EacProfile) {
    let r_values = decode_eac11(r, profile);
    let g_values = g.map(|block| decode_eac11(block, profile));
    for y in 0..4 {
        for x in 0..4 {
            let source = x * 4 + y;
            let destination = (y * 4 + x) * 4;
            output[destination] = r_values[source];
            output[destination + 1] = g_values.as_ref().map_or(0, |values| values[source]);
            output[destination + 2] = 0;
            output[destination + 3] = 255;
        }
    }
}

fn decode_eac11(block: &[u8; 8], profile: EacProfile) -> [u8; 16] {
    let table = usize::from(block[1] & 0x0f);
    let multiplier = i32::from(block[1] >> 4);
    let scale = if multiplier == 0 { 1 } else { multiplier * 8 };
    let indices = eac_indices(block);
    let base = match profile {
        EacProfile::Unsigned => i32::from(block[0]) * 8 + 4,
        EacProfile::Signed => {
            let signed = i32::from(block[0] as i8);
            if signed == -128 { -127 * 8 } else { signed * 8 }
        }
    };
    let mut result = [0_u8; 16];
    for (i, value) in result.iter_mut().enumerate() {
        let selector = ((indices >> (45 - 3 * i)) & 7) as usize;
        let decoded = base + EAC_MODIFIERS[table][selector] * scale;
        *value = match profile {
            EacProfile::Unsigned => unorm11_to_u8(decoded.clamp(0, 2047)),
            EacProfile::Signed => snorm11_to_u8(decoded.clamp(-1023, 1023)),
        };
    }
    result
}

fn eac_indices(block: &[u8; 8]) -> u64 {
    u64::from_be_bytes([
        0, 0, block[2], block[3], block[4], block[5], block[6], block[7],
    ])
}

fn unorm11_to_u8(value: i32) -> u8 {
    ((value * 255 + 1023) / 2047) as u8
}

fn snorm11_to_u8(value: i32) -> u8 {
    (((value + 1023) * 255 + 1023) / 2046) as u8
}

fn unstuff_t(word: u64) -> u64 {
    let mut result = 0_u64;
    result = set_bits(result, bits(word, 25, 57), 25, 56);
    result = set_bits(result, bit(word, 32), 1, 32);
    set_bits(result, bits(word, 2, 60), 2, 58)
}

fn unstuff_h(word: u64) -> u64 {
    let mut result = 0_u64;
    result = set_bits(result, bits(word, 7, 62), 7, 57);
    result = set_bits(result, bits(word, 2, 52), 2, 50);
    result = set_bits(result, bits(word, 16, 49), 16, 48);
    set_bits(result, bit(word, 32), 1, 32)
}

fn unstuff_planar(word: u64) -> u64 {
    let mappings = [
        (6, 62, 63),
        (1, 56, 57),
        (6, 54, 56),
        (1, 48, 50),
        (2, 44, 49),
        (3, 41, 47),
        (5, 38, 44),
        (1, 32, 39),
        (7, 31, 38),
        (6, 24, 31),
        (6, 18, 25),
        (7, 12, 19),
        (6, 5, 12),
    ];
    let mut result = 0_u64;
    for (size, source, destination) in mappings {
        result = set_bits(result, bits(word, size, source), size, destination);
    }
    result
}

fn selector(word: u64, x: usize, y: usize) -> usize {
    let shift = x * 4 + y;
    ((((word >> (16 + shift)) & 1) << 1) | ((word >> shift) & 1)) as usize
}

fn write_pixel(output: &mut [u8], x: usize, y: usize, color: [u8; 3], alpha: u8) {
    let offset = (y * 4 + x) * 4;
    output[offset..offset + 3].copy_from_slice(&color);
    output[offset + 3] = alpha;
}

fn add_modifier(color: [u8; 3], modifier: i32) -> [u8; 3] {
    [
        clamp_u8(i32::from(color[0]) + modifier),
        clamp_u8(i32::from(color[1]) + modifier),
        clamp_u8(i32::from(color[2]) + modifier),
    ]
}

fn clamp_u8(value: i32) -> u8 {
    value.clamp(0, 255) as u8
}

fn expand4(value: u32) -> u8 {
    ((value << 4) | value) as u8
}

fn expand5(value: u32) -> u8 {
    ((value << 3) | (value >> 2)) as u8
}

fn expand6(value: u32) -> u8 {
    ((value << 2) | (value >> 4)) as u8
}

fn expand7(value: u32) -> u8 {
    ((value << 1) | (value >> 6)) as u8
}

fn sign3(value: u32) -> i32 {
    if value & 4 != 0 {
        value as i32 - 8
    } else {
        value as i32
    }
}

fn bit(word: u64, position: usize) -> u32 {
    ((word >> position) & 1) as u32
}

fn bits(word: u64, size: usize, start: usize) -> u32 {
    ((word >> (start + 1 - size)) & ((1_u64 << size) - 1)) as u32
}

fn set_bits(word: u64, value: u32, size: usize, start: usize) -> u64 {
    let shift = start + 1 - size;
    let mask = ((1_u64 << size) - 1) << shift;
    (word & !mask) | ((u64::from(value) << shift) & mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_buffer_and_size_errors() {
        assert_eq!(
            decode_etc2_rgb8_block(&[0; 8], &mut [0; 63]),
            Err(DecodeError::OutputTooShort)
        );
        assert_eq!(
            decode_etc2_rgb8(&[], 1, 1, &mut [0; 4]),
            Err(DecodeError::InputTooShort)
        );
        assert_eq!(
            decode_etc2_rgb8(&[], usize::MAX, 2, &mut []),
            Err(DecodeError::SizeOverflow)
        );
    }

    #[test]
    fn empty_images_require_no_storage() {
        decode_etc2_rgba8(&[], 0, usize::MAX, &mut []).unwrap();
        decode_eac_rg11(&[], usize::MAX, 0, &mut []).unwrap();
    }

    #[test]
    fn clips_partial_blocks_and_preserves_extra_output() {
        let data = [0_u8; 16];
        let mut output = [0xcc_u8; 5 * 3 * 4 + 4];
        decode_etc2_rgb8(&data, 5, 3, &mut output).unwrap();
        for pixel in output[..5 * 3 * 4].chunks_exact(4) {
            assert_eq!(pixel, [2, 2, 2, 255]);
        }
        assert_eq!(&output[5 * 3 * 4..], &[0xcc; 4]);
    }

    #[test]
    fn signed_and_unsigned_eac_mapping_reaches_expected_values() {
        assert_eq!(unorm11_to_u8(0), 0);
        assert_eq!(unorm11_to_u8(2047), 255);
        assert_eq!(snorm11_to_u8(-1023), 0);
        assert_eq!(snorm11_to_u8(0), 128);
        assert_eq!(snorm11_to_u8(1023), 255);
    }

    #[test]
    fn arbitrary_blocks_do_not_panic() {
        let mut output = [0_u8; 64];
        for byte in 0..=u8::MAX {
            let block8 = [byte; 8];
            let block16 = [byte; 16];
            let _ = decode_etc1_rgb8_block(&block8, &mut output);
            decode_etc2_rgb8_block(&block8, &mut output).unwrap();
            decode_etc2_rgb8a1_block(&block8, &mut output).unwrap();
            decode_etc2_rgba8_block(&block16, &mut output).unwrap();
            decode_eac_r11_block(&block8, &mut output).unwrap();
            decode_eac_signed_rg11_block(&block16, &mut output).unwrap();
        }
    }
}
