//! ASTC physical block decoding.
//!
//! This module is an integer-only port of the decoding rules used by Arm's
//! astc-encoder reference implementation. See `LICENSE-APACHE` for the
//! license applying to copied tables and algorithms.

use super::tables::{
    COLOR_UNQUANT_LENGTHS, QUINTS_OF_INTEGER, TRITS_OF_INTEGER, WEIGHT_UNQUANT, color_unquant,
};

use crate::DecodeError;

macro_rules! footprint_api {
    ($block:ident, $image:ident, $x:expr, $y:expr) => {
        #[doc = concat!("Decode one ", stringify!($x), "x", stringify!($y), " ASTC LDR block into tightly packed RGBA8.")]
        pub fn $block(block: &[u8; 16], output: &mut [u8]) -> Result<(), DecodeError> {
            decode_block(block, $x, $y, output)
        }

        #[doc = concat!("Decode a ", stringify!($x), "x", stringify!($y), " ASTC LDR image into tightly packed RGBA8.")]
        pub fn $image(data: &[u8], width: usize, height: usize, output: &mut [u8]) -> Result<(), DecodeError> {
            decode_image(data, width, height, $x, $y, output)
        }
    };
}

footprint_api!(decode_ldr_block_4_4, decode_ldr_4_4, 4, 4);
footprint_api!(decode_ldr_block_5_4, decode_ldr_5_4, 5, 4);
footprint_api!(decode_ldr_block_5_5, decode_ldr_5_5, 5, 5);
footprint_api!(decode_ldr_block_6_5, decode_ldr_6_5, 6, 5);
footprint_api!(decode_ldr_block_6_6, decode_ldr_6_6, 6, 6);
footprint_api!(decode_ldr_block_8_5, decode_ldr_8_5, 8, 5);
footprint_api!(decode_ldr_block_8_6, decode_ldr_8_6, 8, 6);
footprint_api!(decode_ldr_block_8_8, decode_ldr_8_8, 8, 8);
footprint_api!(decode_ldr_block_10_5, decode_ldr_10_5, 10, 5);
footprint_api!(decode_ldr_block_10_6, decode_ldr_10_6, 10, 6);
footprint_api!(decode_ldr_block_10_8, decode_ldr_10_8, 10, 8);
footprint_api!(decode_ldr_block_10_10, decode_ldr_10_10, 10, 10);
footprint_api!(decode_ldr_block_12_10, decode_ldr_12_10, 12, 10);
footprint_api!(decode_ldr_block_12_12, decode_ldr_12_12, 12, 12);

fn decode_image(
    data: &[u8],
    width: usize,
    height: usize,
    block_x: usize,
    block_y: usize,
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let texels = width.checked_mul(height).ok_or(DecodeError::SizeOverflow)?;
    let output_len = texels.checked_mul(4).ok_or(DecodeError::SizeOverflow)?;
    if output.len() < output_len {
        return Err(DecodeError::OutputTooShort);
    }

    let blocks_x = div_ceil(width, block_x);
    let blocks_y = div_ceil(height, block_y);
    let blocks = blocks_x
        .checked_mul(blocks_y)
        .ok_or(DecodeError::SizeOverflow)?;
    let input_len = blocks.checked_mul(16).ok_or(DecodeError::SizeOverflow)?;
    if data.len() < input_len {
        return Err(DecodeError::InputTooShort);
    }

    // A 12x12 footprint is the largest legal 2D ASTC footprint.
    let mut block_output = [0_u8; 12 * 12 * 4];
    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let block_offset = (by * blocks_x + bx) * 16;
            let block: &[u8; 16] = data[block_offset..block_offset + 16]
                .try_into()
                .map_err(|_| DecodeError::InputTooShort)?;
            let block_len = block_x * block_y * 4;
            decode_block(block, block_x, block_y, &mut block_output[..block_len])?;

            let copy_width = core::cmp::min(block_x, width - bx * block_x);
            let copy_height = core::cmp::min(block_y, height - by * block_y);
            for y in 0..copy_height {
                let src = y * block_x * 4;
                let dst = ((by * block_y + y) * width + bx * block_x) * 4;
                output[dst..dst + copy_width * 4]
                    .copy_from_slice(&block_output[src..src + copy_width * 4]);
            }
        }
    }

    Ok(())
}

#[inline]
fn div_ceil(value: usize, divisor: usize) -> usize {
    value.div_ceil(divisor)
}

fn decode_block(
    block: &[u8; 16],
    block_x: usize,
    block_y: usize,
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let output_len = block_x
        .checked_mul(block_y)
        .and_then(|value| value.checked_mul(4))
        .ok_or(DecodeError::SizeOverflow)?;
    if output.len() < output_len {
        return Err(DecodeError::OutputTooShort);
    }

    let block_mode = read_bits(block, 11, 0);
    if (block_mode & 0x1ff) == 0x1fc {
        return decode_void_extent_ldr(block, output_len, output);
    }

    decode_nonconstant_block(block, block_x, block_y, &mut output[..output_len])
}

fn decode_void_extent_ldr(
    block: &[u8; 16],
    output_len: usize,
    output: &mut [u8],
) -> Result<(), DecodeError> {
    // A floating-point void extent is not part of the LDR profile.
    if (read_bits(block, 11, 0) & 0x200) != 0 || read_bits(block, 2, 10) != 3 {
        return Err(DecodeError::InvalidBlock);
    }

    let low_s = read_bits(block, 8, 12) | (read_bits(block, 5, 20) << 8);
    let high_s = read_bits(block, 13, 25);
    let low_t = read_bits(block, 8, 38) | (read_bits(block, 5, 46) << 8);
    let high_t = read_bits(block, 13, 51);
    let all_ones = low_s == 0x1fff && high_s == 0x1fff && low_t == 0x1fff && high_t == 0x1fff;
    if !all_ones && (low_s >= high_s || low_t >= high_t) {
        return Err(DecodeError::InvalidBlock);
    }

    let color = [block[9], block[11], block[13], block[15]];
    for pixel in output[..output_len].chunks_exact_mut(4) {
        pixel.copy_from_slice(&color);
    }
    Ok(())
}

fn decode_nonconstant_block(
    block: &[u8; 16],
    block_x: usize,
    block_y: usize,
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let block_mode = read_bits(block, 11, 0) as usize;
    let mode = decode_block_mode_2d(block_mode).ok_or(DecodeError::InvalidBlock)?;
    if mode.weight_x > block_x || mode.weight_y > block_y {
        return Err(DecodeError::InvalidBlock);
    }

    let weight_count = mode.weight_x * mode.weight_y;
    let real_weight_count = weight_count * if mode.dual_plane { 2 } else { 1 };
    let weight_bits = ise_bit_count(real_weight_count, mode.quant);
    if !(24..=96).contains(&weight_bits) {
        return Err(DecodeError::InvalidBlock);
    }

    let partition_count = read_bits(block, 2, 11) as usize + 1;
    if mode.dual_plane && partition_count == 4 {
        return Err(DecodeError::InvalidBlock);
    }

    let mut reversed = [0_u8; 16];
    for (index, byte) in reversed.iter_mut().enumerate() {
        *byte = block[15 - index].reverse_bits();
    }
    let mut weight_indices = [0_u8; 64];
    decode_ise(
        mode.quant,
        real_weight_count,
        &reversed,
        0,
        &mut weight_indices,
    )?;
    let mut weights1 = [0_i32; 64];
    let mut weights2 = [0_i32; 64];
    for i in 0..weight_count {
        weights1[i] = i32::from(
            WEIGHT_UNQUANT[mode.quant]
                [weight_indices[if mode.dual_plane { 2 * i } else { i }] as usize],
        );
        if mode.dual_plane {
            weights2[i] = i32::from(WEIGHT_UNQUANT[mode.quant][weight_indices[2 * i + 1] as usize]);
        }
    }

    let mut below_weights_pos = 128 - weight_bits;
    let mut formats = [0_u8; 4];
    let partition_index;
    let mut encoded_type_highpart_size = 0_usize;
    if partition_count == 1 {
        formats[0] = read_bits(block, 4, 13) as u8;
        partition_index = 0;
    } else {
        encoded_type_highpart_size = 3 * partition_count - 4;
        below_weights_pos = below_weights_pos
            .checked_sub(encoded_type_highpart_size)
            .ok_or(DecodeError::InvalidBlock)?;
        let encoded_type = read_bits(block, 6, 23)
            | (read_bits(block, encoded_type_highpart_size, below_weights_pos) << 6);
        let baseclass = encoded_type & 3;
        if baseclass == 0 {
            let format = ((encoded_type >> 2) & 0xf) as u8;
            formats[..partition_count].fill(format);
            below_weights_pos += encoded_type_highpart_size;
            encoded_type_highpart_size = 0;
        } else {
            let mut bitpos = 2;
            let baseclass = baseclass - 1;
            for format in &mut formats[..partition_count] {
                *format = ((((encoded_type >> bitpos) & 1) + baseclass) << 2) as u8;
                bitpos += 1;
            }
            for format in &mut formats[..partition_count] {
                *format |= ((encoded_type >> bitpos) & 3) as u8;
                bitpos += 2;
            }
        }
        partition_index = read_bits(block, 10, 13) as usize;
    }

    let color_integer_count: usize = formats[..partition_count]
        .iter()
        .map(|format| 2 * (usize::from(*format >> 2) + 1))
        .sum();
    if color_integer_count > 18 {
        return Err(DecodeError::InvalidBlock);
    }
    let mut color_bits: usize = match partition_count {
        1 => 111,
        2..=4 => 99,
        _ => return Err(DecodeError::InvalidBlock),
    };
    color_bits = color_bits
        .checked_sub(weight_bits + encoded_type_highpart_size + usize::from(mode.dual_plane) * 2)
        .ok_or(DecodeError::InvalidBlock)?;
    let color_quant =
        color_quant_level(color_integer_count, color_bits).ok_or(DecodeError::InvalidBlock)?;
    let mut packed_colors = [0_u8; 32];
    let color_offset = if partition_count == 1 { 17 } else { 29 };
    decode_ise(
        color_quant + 4,
        color_integer_count,
        block,
        color_offset,
        &mut packed_colors,
    )?;

    let mut endpoints = [([0_i32; 4], [0_i32; 4]); 4];
    let mut color_offset = 0;
    for partition in 0..partition_count {
        let count = 2 * (usize::from(formats[partition] >> 2) + 1);
        let mut values = [0_u8; 8];
        for i in 0..count {
            values[i] = color_unquant(color_quant, packed_colors[color_offset + i]);
        }
        endpoints[partition] = unpack_ldr_endpoints(formats[partition], &values)?;
        color_offset += count;
    }

    let plane2_component = if mode.dual_plane {
        if below_weights_pos < 2 {
            return Err(DecodeError::InvalidBlock);
        }
        read_bits(block, 2, below_weights_pos - 2) as usize
    } else {
        4
    };

    for y in 0..block_y {
        for x in 0..block_x {
            let texel = y * block_x + x;
            let partition = if partition_count == 1 {
                0
            } else {
                select_partition(
                    partition_index as u32,
                    x as u32,
                    y as u32,
                    partition_count as u32,
                    block_x * block_y < 32,
                ) as usize
            };
            let weight1 = undecimate_weight(
                x,
                y,
                block_x,
                block_y,
                mode.weight_x,
                mode.weight_y,
                &weights1,
            );
            let weight2 = if mode.dual_plane {
                undecimate_weight(
                    x,
                    y,
                    block_x,
                    block_y,
                    mode.weight_x,
                    mode.weight_y,
                    &weights2,
                )
            } else {
                0
            };
            let (endpoint0, endpoint1) = endpoints[partition];
            for component in 0..4 {
                let weight = if component == plane2_component {
                    weight2
                } else {
                    weight1
                };
                let value = ((endpoint0[component] * 257 * (64 - weight)
                    + endpoint1[component] * 257 * weight
                    + 32)
                    >> 6)
                    >> 8;
                output[texel * 4 + component] = value as u8;
            }
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct BlockMode {
    weight_x: usize,
    weight_y: usize,
    dual_plane: bool,
    quant: usize,
}

fn decode_block_mode_2d(mode: usize) -> Option<BlockMode> {
    let mut base_quant = (mode >> 4) & 1;
    let mut high_precision = (mode >> 9) & 1;
    let mut dual_plane = (mode >> 10) & 1;
    let a = (mode >> 5) & 3;
    let (weight_x, weight_y) = if mode & 3 != 0 {
        base_quant |= (mode & 3) << 1;
        let mut b = (mode >> 7) & 3;
        match (mode >> 2) & 3 {
            0 => (b + 4, a + 2),
            1 => (b + 8, a + 2),
            2 => (a + 2, b + 8),
            3 if mode & 0x100 != 0 => {
                b &= 1;
                (b + 2, a + 2)
            }
            3 => {
                b &= 1;
                (a + 2, b + 6)
            }
            _ => return None,
        }
    } else {
        base_quant |= ((mode >> 2) & 3) << 1;
        if (mode >> 2) & 3 == 0 {
            return None;
        }
        let b = (mode >> 9) & 3;
        match (mode >> 7) & 3 {
            0 => (12, a + 2),
            1 => (a + 2, 12),
            2 => {
                dual_plane = 0;
                high_precision = 0;
                (a + 6, b + 6)
            }
            3 => match (mode >> 5) & 3 {
                0 => (6, 10),
                1 => (10, 6),
                _ => return None,
            },
            _ => return None,
        }
    };
    let quant = (base_quant.checked_sub(2)?) + 6 * high_precision;
    let weight_count = weight_x * weight_y * (dual_plane + 1);
    let bits = ise_bit_count(weight_count, quant);
    if weight_count > 64 || !(24..=96).contains(&bits) || quant >= WEIGHT_UNQUANT.len() {
        return None;
    }
    Some(BlockMode {
        weight_x,
        weight_y,
        dual_plane: dual_plane != 0,
        quant,
    })
}

const ISE_SIZES: [(usize, usize); 21] = [
    (1, 1),
    (8, 5),
    (2, 1),
    (7, 3),
    (13, 5),
    (3, 1),
    (10, 3),
    (18, 5),
    (4, 1),
    (13, 3),
    (23, 5),
    (5, 1),
    (16, 3),
    (28, 5),
    (6, 1),
    (19, 3),
    (33, 5),
    (7, 1),
    (22, 3),
    (38, 5),
    (8, 1),
];

const ISE_COUNTS: [(usize, bool, bool); 21] = [
    (1, false, false),
    (0, true, false),
    (2, false, false),
    (0, false, true),
    (1, true, false),
    (3, false, false),
    (1, false, true),
    (2, true, false),
    (4, false, false),
    (2, false, true),
    (3, true, false),
    (5, false, false),
    (3, false, true),
    (4, true, false),
    (6, false, false),
    (4, false, true),
    (5, true, false),
    (7, false, false),
    (5, false, true),
    (6, true, false),
    (8, false, false),
];

#[inline]
fn ise_bit_count(count: usize, quant: usize) -> usize {
    let (scale, divisor) = ISE_SIZES[quant];
    (scale * count).div_ceil(divisor)
}

fn color_quant_level(count: usize, bits: usize) -> Option<usize> {
    if count == 0 || count > 18 {
        return None;
    }
    (0..COLOR_UNQUANT_LENGTHS.len())
        .rev()
        .find(|level| ise_bit_count(count, level + 4) <= bits)
}

fn decode_ise(
    quant: usize,
    count: usize,
    input: &[u8; 16],
    mut bit_offset: usize,
    output: &mut [u8],
) -> Result<(), DecodeError> {
    if quant >= ISE_COUNTS.len() || count > output.len() || count > 64 {
        return Err(DecodeError::InvalidBlock);
    }
    let (bits, trits, quints) = ISE_COUNTS[quant];
    let mut tq_blocks = [0_u8; 22];
    let mut lcounter = 0;
    let mut hcounter = 0;
    for item in output.iter_mut().take(count) {
        if bit_offset + bits > 128 {
            return Err(DecodeError::InvalidBlock);
        }
        *item = read_bits(input, bits, bit_offset) as u8;
        bit_offset += bits;
        if trits {
            const BITS: [usize; 5] = [2, 2, 1, 2, 1];
            const SHIFT: [usize; 5] = [0, 2, 4, 5, 7];
            if bit_offset + BITS[lcounter] > 128 || hcounter >= tq_blocks.len() {
                return Err(DecodeError::InvalidBlock);
            }
            tq_blocks[hcounter] |=
                (read_bits(input, BITS[lcounter], bit_offset) as u8) << SHIFT[lcounter];
            bit_offset += BITS[lcounter];
            hcounter += usize::from(lcounter == 4);
            lcounter = (lcounter + 1) % 5;
        }
        if quints {
            const BITS: [usize; 3] = [3, 2, 2];
            const SHIFT: [usize; 3] = [0, 3, 5];
            if bit_offset + BITS[lcounter] > 128 || hcounter >= tq_blocks.len() {
                return Err(DecodeError::InvalidBlock);
            }
            tq_blocks[hcounter] |=
                (read_bits(input, BITS[lcounter], bit_offset) as u8) << SHIFT[lcounter];
            bit_offset += BITS[lcounter];
            hcounter += usize::from(lcounter == 2);
            lcounter = (lcounter + 1) % 3;
        }
    }
    if trits {
        for block in 0..count.div_ceil(5) {
            for item in 0..5 {
                if block * 5 + item < count {
                    output[block * 5 + item] |=
                        TRITS_OF_INTEGER[tq_blocks[block] as usize][item] << bits;
                }
            }
        }
    }
    if quints {
        for block in 0..count.div_ceil(3) {
            for item in 0..3 {
                if block * 3 + item < count {
                    output[block * 3 + item] |=
                        QUINTS_OF_INTEGER[tq_blocks[block] as usize][item] << bits;
                }
            }
        }
    }
    Ok(())
}

fn unpack_ldr_endpoints(format: u8, values: &[u8; 8]) -> Result<([i32; 4], [i32; 4]), DecodeError> {
    let value = |index: usize| i32::from(values[index]);
    let result = match format {
        0 => {
            let a = [value(0), value(0), value(0), 255];
            let b = [value(1), value(1), value(1), 255];
            (a, b)
        }
        1 => {
            let l0 = (value(0) >> 2) | (value(1) & 0xc0);
            let l1 = (l0 + (value(1) & 0x3f)).min(255);
            ([l0, l0, l0, 255], [l1, l1, l1, 255])
        }
        4 => (
            [value(0), value(0), value(0), value(2)],
            [value(1), value(1), value(1), value(3)],
        ),
        5 => {
            let mut lum0 = value(0) | ((value(1) & 0x80) << 1);
            let mut alpha0 = value(2) | ((value(3) & 0x80) << 1);
            let mut lum1 = value(1) & 0x7f;
            let mut alpha1 = value(3) & 0x7f;
            if lum1 & 0x40 != 0 {
                lum1 -= 0x80;
            }
            if alpha1 & 0x40 != 0 {
                alpha1 -= 0x80;
            }
            lum0 >>= 1;
            alpha0 >>= 1;
            lum1 = (lum1 >> 1) + lum0;
            alpha1 = (alpha1 >> 1) + alpha0;
            lum1 = lum1.clamp(0, 255);
            alpha1 = alpha1.clamp(0, 255);
            ([lum0, lum0, lum0, alpha0], [lum1, lum1, lum1, alpha1])
        }
        6 => {
            let endpoint1 = [value(0), value(1), value(2), 255];
            let scale = value(3);
            (
                [
                    (endpoint1[0] * scale) >> 8,
                    (endpoint1[1] * scale) >> 8,
                    (endpoint1[2] * scale) >> 8,
                    255,
                ],
                endpoint1,
            )
        }
        8 => unpack_direct(
            [value(0), value(2), value(4), 0],
            [value(1), value(3), value(5), 0],
            false,
        ),
        9 => unpack_delta(
            [value(0), value(2), value(4), 0],
            [value(1), value(3), value(5), 0],
            false,
        ),
        10 => {
            let endpoint1 = [value(0), value(1), value(2), value(5)];
            let scale = value(3);
            (
                [
                    (endpoint1[0] * scale) >> 8,
                    (endpoint1[1] * scale) >> 8,
                    (endpoint1[2] * scale) >> 8,
                    value(4),
                ],
                endpoint1,
            )
        }
        12 => unpack_direct(
            [value(0), value(2), value(4), value(6)],
            [value(1), value(3), value(5), value(7)],
            true,
        ),
        13 => unpack_delta(
            [value(0), value(2), value(4), value(6)],
            [value(1), value(3), value(5), value(7)],
            true,
        ),
        // The remaining formats encode HDR endpoints and are invalid in the LDR profile.
        _ => return Err(DecodeError::InvalidBlock),
    };
    Ok(result)
}

fn unpack_direct(
    mut endpoint0: [i32; 4],
    mut endpoint1: [i32; 4],
    alpha: bool,
) -> ([i32; 4], [i32; 4]) {
    if endpoint0[0] + endpoint0[1] + endpoint0[2] > endpoint1[0] + endpoint1[1] + endpoint1[2] {
        endpoint0 = uncontract(endpoint0);
        endpoint1 = uncontract(endpoint1);
        core::mem::swap(&mut endpoint0, &mut endpoint1);
    }
    if !alpha {
        endpoint0[3] = 255;
        endpoint1[3] = 255;
    }
    (endpoint0, endpoint1)
}

fn unpack_delta(
    mut endpoint0: [i32; 4],
    mut endpoint1: [i32; 4],
    alpha: bool,
) -> ([i32; 4], [i32; 4]) {
    for component in 0..4 {
        endpoint0[component] = (endpoint0[component] >> 1) | (endpoint1[component] & 0x80);
        endpoint1[component] = (endpoint1[component] >> 1) & 0x3f;
        if endpoint1[component] & 0x20 != 0 {
            endpoint1[component] -= 0x40;
        }
    }
    let sum = endpoint1[0] + endpoint1[1] + endpoint1[2];
    for component in 0..4 {
        endpoint1[component] += endpoint0[component];
    }
    if sum < 0 {
        endpoint0 = uncontract(endpoint0);
        endpoint1 = uncontract(endpoint1);
        core::mem::swap(&mut endpoint0, &mut endpoint1);
    }
    for endpoint in [&mut endpoint0, &mut endpoint1] {
        for value in endpoint.iter_mut() {
            *value = (*value).clamp(0, 255);
        }
    }
    if !alpha {
        endpoint0[3] = 255;
        endpoint1[3] = 255;
    }
    (endpoint0, endpoint1)
}

#[inline]
fn uncontract(mut endpoint: [i32; 4]) -> [i32; 4] {
    endpoint[0] = (endpoint[0] + endpoint[2]) >> 1;
    endpoint[1] = (endpoint[1] + endpoint[2]) >> 1;
    endpoint
}

fn undecimate_weight(
    x: usize,
    y: usize,
    texels_x: usize,
    texels_y: usize,
    weights_x: usize,
    weights_y: usize,
    weights: &[i32; 64],
) -> i32 {
    let x_weight = (((1024 + texels_x / 2) / (texels_x - 1)) * x * (weights_x - 1) + 32) >> 6;
    let y_weight = (((1024 + texels_y / 2) / (texels_y - 1)) * y * (weights_y - 1) + 32) >> 6;
    let xf = x_weight & 15;
    let yf = y_weight & 15;
    let xi = x_weight >> 4;
    let yi = y_weight >> 4;
    let indices = [
        xi + yi * weights_x,
        xi + yi * weights_x + 1,
        xi + (yi + 1) * weights_x,
        xi + (yi + 1) * weights_x + 1,
    ];
    let product = xf * yf;
    let w3 = (product + 8) >> 4;
    let factors = [16 + w3 - xf - yf, xf - w3, yf - w3, w3];
    // ASTC specifies a rounding bias before the truncated bilinear sum.
    let mut value = 8;
    for i in 0..4 {
        if factors[i] != 0 {
            value += weights[indices[i]] * factors[i] as i32;
        }
    }
    value >> 4
}

fn select_partition(
    seed: u32,
    mut x: u32,
    mut y: u32,
    partition_count: u32,
    small_block: bool,
) -> u8 {
    if small_block {
        x <<= 1;
        y <<= 1;
    }
    let seed = seed + (partition_count - 1) * 1024;
    let rnum = hash52(seed);
    let mut seeds = [
        rnum & 0xf,
        (rnum >> 4) & 0xf,
        (rnum >> 8) & 0xf,
        (rnum >> 12) & 0xf,
        (rnum >> 16) & 0xf,
        (rnum >> 20) & 0xf,
        (rnum >> 24) & 0xf,
        (rnum >> 28) & 0xf,
        (rnum >> 18) & 0xf,
        (rnum >> 22) & 0xf,
        (rnum >> 26) & 0xf,
        rnum.rotate_left(2) & 0xf,
    ];
    for value in &mut seeds {
        *value *= *value;
    }
    let (sh1, sh2) = if seed & 1 != 0 {
        (
            if seed & 2 != 0 { 4 } else { 5 },
            if partition_count == 3 { 6 } else { 5 },
        )
    } else {
        (
            if partition_count == 3 { 6 } else { 5 },
            if seed & 2 != 0 { 4 } else { 5 },
        )
    };
    let sh3 = if seed & 0x10 != 0 { sh1 } else { sh2 };
    for index in [0, 2, 4, 6] {
        seeds[index] >>= sh1;
    }
    for index in [1, 3, 5, 7] {
        seeds[index] >>= sh2;
    }
    for seed in seeds.iter_mut().skip(8) {
        *seed >>= sh3;
    }
    let mut values = [
        (seeds[0] * x + seeds[1] * y + (rnum >> 14)) & 0x3f,
        (seeds[2] * x + seeds[3] * y + (rnum >> 10)) & 0x3f,
        (seeds[4] * x + seeds[5] * y + (rnum >> 6)) & 0x3f,
        (seeds[6] * x + seeds[7] * y + (rnum >> 2)) & 0x3f,
    ];
    if partition_count <= 3 {
        values[3] = 0;
    }
    if partition_count <= 2 {
        values[2] = 0;
    }
    if partition_count <= 1 {
        values[1] = 0;
    }
    if values[0] >= values[1] && values[0] >= values[2] && values[0] >= values[3] {
        0
    } else if values[1] >= values[2] && values[1] >= values[3] {
        1
    } else if values[2] >= values[3] {
        2
    } else {
        3
    }
}

fn hash52(mut value: u32) -> u32 {
    value ^= value >> 15;
    value = value.wrapping_mul(0xeede_0891);
    value ^= value >> 5;
    value = value.wrapping_add(value << 16);
    value ^= value >> 7;
    value ^= value >> 3;
    value ^= value << 6;
    value ^= value >> 17;
    value
}

#[inline]
fn read_bits(data: &[u8; 16], bit_count: usize, bit_offset: usize) -> u32 {
    debug_assert!(bit_count <= 16);
    debug_assert!(bit_offset + bit_count <= 128);
    let mut value = 0_u32;
    for bit in 0..bit_count {
        value |= u32::from((data[(bit_offset + bit) >> 3] >> ((bit_offset + bit) & 7)) & 1) << bit;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_a_ldr_void_extent_block() {
        let block = [
            0xfc, 0xfd, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xff, 0xff,
        ];
        let mut output = [0_u8; 4 * 4 * 4];
        decode_ldr_block_4_4(&block, &mut output).unwrap();
        assert!(output.chunks_exact(4).all(|pixel| pixel == [0, 0, 0, 255]));
    }

    #[test]
    fn rejects_short_output_before_writing() {
        let block = [0_u8; 16];
        let mut output = [0xa5_u8; 3];
        assert_eq!(
            decode_ldr_block_4_4(&block, &mut output),
            Err(DecodeError::OutputTooShort)
        );
        assert_eq!(output, [0xa5; 3]);
    }

    #[test]
    fn decodes_reference_nonconstant_12x12_block() {
        // `decompress12x12` from astc-encoder's unit tests, decoded with the
        // reference CLI into an RGBA8 golden image.
        let block = [
            0x29, 0x00, 0x1a, 0x97, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcf,
            0x97, 0x86,
        ];
        let mut output = [0_u8; 12 * 12 * 4];
        decode_ldr_block_12_12(&block, &mut output).unwrap();
        assert_eq!(&output[..4], &[0xcb, 0xcb, 0xcb, 0xff]);
        assert_eq!(&output[4 * 5..4 * 6], &[0x91, 0x91, 0x91, 0xff]);
        assert_eq!(
            &output[4 * 12 * 6..4 * 12 * 6 + 4],
            &[0x8d, 0x8d, 0x8d, 0xff]
        );
        let hash = output.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
        });
        assert_eq!(hash, 0x66fc_9350_78ca_06fe);
    }

    #[test]
    fn decodes_copied_astcenc_4x4_image_resource() {
        let data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/astcenc-tiles-ldr-4x4.astc"
        ));
        let mut output = [0_u8; 8 * 8 * 4];
        decode_ldr_4_4(&data[16..], 8, 8, &mut output).unwrap();
        let hash = output.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
        });
        assert_eq!(hash, 0xe94b_54a8_849a_63c5);
    }

    #[test]
    fn clips_copied_6x6_resource_at_image_edges() {
        let data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/astcenc-ldr-a-1x1-6x6.astc"
        ));
        let mut output = [0_u8; 4];
        decode_ldr_6_6(&data[16..], 1, 1, &mut output).unwrap();
        assert_eq!(output, [0x2b, 0xad, 0x00, 0xff]);
    }

    #[test]
    fn decodes_reference_compressed_complex_4x4_image_byte_exactly() {
        let data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/astcenc-tiles-ldr-complex-4x4.astc"
        ));
        let expected = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/astcenc-tiles-ldr-complex-4x4.rgba"
        ));
        let mut output = [0_u8; 8 * 8 * 4];
        decode_ldr_4_4(&data[16..], 8, 8, &mut output).unwrap();
        assert_eq!(&output, expected);
    }

    #[test]
    fn reports_input_and_dimension_errors() {
        let mut output = [0_u8; 4 * 4 * 4];
        assert_eq!(
            decode_ldr_4_4(&[], 4, 4, &mut output),
            Err(DecodeError::InputTooShort)
        );
        assert_eq!(
            decode_ldr_4_4(&[], usize::MAX, 2, &mut output),
            Err(DecodeError::SizeOverflow)
        );
    }

    #[test]
    fn rejects_an_illegal_block_without_an_error_color() {
        let block = [0_u8; 16];
        let mut output = [0xa5_u8; 4 * 4 * 4];
        assert_eq!(
            decode_ldr_block_4_4(&block, &mut output),
            Err(DecodeError::InvalidBlock)
        );
        assert_eq!(output, [0xa5; 4 * 4 * 4]);
    }

    #[test]
    fn arbitrary_physical_blocks_do_not_panic() {
        let mut state = 0x6a09_e667_f3bc_c909_u64;
        let mut output = [0_u8; 12 * 12 * 4];
        for _ in 0..10_000 {
            let mut block = [0_u8; 16];
            for byte in &mut block {
                state ^= state << 7;
                state ^= state >> 9;
                state ^= state << 8;
                *byte = state as u8;
            }
            let _ = decode_ldr_block_12_12(&block, &mut output);
        }
    }
}
