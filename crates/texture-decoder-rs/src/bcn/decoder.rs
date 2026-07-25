//! BC1–BC7 physical block decoding.
//!
//! The block layouts follow the Microsoft Direct3D BC and the AMD/Compressonator
//! BC6H/BC7 reference decoding rules. Compressonator is used only as a source of
//! conformance fixtures and reference tables; see `LICENSE-MIT-COMPRESSONATOR`.
//!
//! All entry points write tightly packed RGBA8 (row-major). Output conventions:
//!
//! - BC1: RGBA; three-color mode selector 3 is transparent black.
//! - BC2: RGBA; 4-bit alpha expanded to `value * 17`.
//! - BC3: RGBA; uses the 8-entry alpha interpolation.
//! - BC4: `[R, 0, 0, 255]`.
//! - BC5: `[R, G, 0, 255]`.
//! - BC6H: `[R, G, B, 255]` with the clamped `f16` → 8-bit mapping.
//! - BC7: the decoded RGBA.
//!
//! No sRGB, gamma, exposure, or tone mapping is performed. BC4/BC5 SNORM is not
//! implemented.

use super::tables::{
    BC6_INDEX_FIXUPS, BC6_MODES, BC6_PARTITIONS, BC7_FIXUP_INDICES, BC7_MODES, BC7_PARTITIONS,
    WEIGHTS2, WEIGHTS3, WEIGHTS4,
};
use crate::DecodeError;

use half::f16;

const BLOCK_WIDTH: usize = 4;
const BLOCK_HEIGHT: usize = 4;
const RGBA_BLOCK_LEN: usize = BLOCK_WIDTH * BLOCK_HEIGHT * 4;

// ---------------------------------------------------------------------------
// Public single-block API
// ---------------------------------------------------------------------------

macro_rules! image_api {
    ($block_fn:ident, $image_fn:ident, $decoder:expr, $block_bytes:literal, $doc:literal) => {
        #[doc = concat!("Decode one 4x4 ", $doc, " block into tightly packed RGBA8.")]
        pub fn $block_fn(block: &[u8; $block_bytes], output: &mut [u8]) -> Result<(), DecodeError> {
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
            decode_image::<$block_bytes>(data, width, height, output, $decoder)
        }
    };
}

image_api!(decode_bc1_block, decode_bc1, decode_bc1_inner, 8, "BC1");
image_api!(decode_bc2_block, decode_bc2, decode_bc2_inner, 16, "BC2");
image_api!(decode_bc3_block, decode_bc3, decode_bc3_inner, 16, "BC3");
image_api!(
    decode_bc4_block,
    decode_bc4,
    decode_bc4_inner,
    8,
    "BC4 UNORM"
);
image_api!(
    decode_bc5_block,
    decode_bc5,
    decode_bc5_inner,
    16,
    "BC5 UNORM"
);
image_api!(
    decode_bc6h_uf16_block,
    decode_bc6h_uf16,
    decode_bc6h_uf16_inner,
    16,
    "BC6H UF16"
);
image_api!(
    decode_bc6h_sf16_block,
    decode_bc6h_sf16,
    decode_bc6h_sf16_inner,
    16,
    "BC6H SF16"
);
image_api!(decode_bc7_block, decode_bc7, decode_bc7_inner, 16, "BC7");

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
    // A zero width or height yields no output, but the block loop below would
    // still iterate over `blocks_y = div_ceil(height, BLOCK_HEIGHT)` rows. With
    // a huge height (e.g. `usize::MAX`) and zero width, the inner loop is a no
    // op yet the outer counter still runs ~`usize::MAX/4` times — a near-infinite
    // spin. Bail out early so zero-dimension images return immediately.
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

// ---------------------------------------------------------------------------
// BC1–BC3 shared color decode
// ---------------------------------------------------------------------------

/// Decode the DXT color portion (first 8 bytes). `allow_transparent` selects the
/// BC1 three-color + transparent-black mode; BC2/BC3 force the four-color mode.
fn decode_dxt_color(block: &[u8; 8], output: &mut [u8], allow_transparent: bool) {
    let n0 = u16::from_le_bytes([block[0], block[1]]);
    let n1 = u16::from_le_bytes([block[2], block[3]]);
    let indices = u32::from_le_bytes([block[4], block[5], block[6], block[7]]);

    let r0 = replicate_565_to_8(n0, 11, 5);
    let g0 = replicate_565_to_8(n0, 5, 6);
    let b0 = replicate_565_to_8(n0, 0, 5);
    let r1 = replicate_565_to_8(n1, 11, 5);
    let g1 = replicate_565_to_8(n1, 5, 6);
    let b1 = replicate_565_to_8(n1, 0, 5);

    let four_color = !allow_transparent || n0 > n1;

    // Compressonator rounds the 1/3 and 2/3 colors using `(2*a+b+1)/3`.
    let palette = if four_color {
        [
            [r0, g0, b0, 255],
            [r1, g1, b1, 255],
            [
                ((2 * r0 as u32 + r1 as u32 + 1) / 3) as u8,
                ((2 * g0 as u32 + g1 as u32 + 1) / 3) as u8,
                ((2 * b0 as u32 + b1 as u32 + 1) / 3) as u8,
                255,
            ],
            [
                ((2 * r1 as u32 + r0 as u32 + 1) / 3) as u8,
                ((2 * g1 as u32 + g0 as u32 + 1) / 3) as u8,
                ((2 * b1 as u32 + b0 as u32 + 1) / 3) as u8,
                255,
            ],
        ]
    } else {
        [
            [r0, g0, b0, 255],
            [r1, g1, b1, 255],
            [
                ((r0 as u32 + r1 as u32) / 2) as u8,
                ((g0 as u32 + g1 as u32) / 2) as u8,
                ((b0 as u32 + b1 as u32) / 2) as u8,
                255,
            ],
            [0, 0, 0, 0],
        ]
    };

    for i in 0..16 {
        let index = ((indices >> (2 * i)) & 3) as usize;
        output[i * 4..i * 4 + 4].copy_from_slice(&palette[index]);
    }
}

#[inline]
fn replicate_565_to_8(value: u16, shift: u32, bits: u32) -> u8 {
    let channel = ((value >> shift) & ((1 << bits) - 1)) as u32;
    let replicated = if bits == 5 {
        (channel << 3) | (channel >> 2)
    } else if bits == 6 {
        (channel << 2) | (channel >> 4)
    } else {
        channel
    };
    replicated as u8
}

fn decode_bc1_inner(block: &[u8; 8], output: &mut [u8]) -> Result<(), DecodeError> {
    decode_dxt_color(block, output, true);
    Ok(())
}

fn decode_bc2_inner(block: &[u8; 16], output: &mut [u8]) -> Result<(), DecodeError> {
    // BC2 stores 16× 4-bit alpha in the first 8 bytes, color in the last 8.
    let mut alpha = [0_u8; 16];
    for i in 0..16 {
        let nibble = if i % 2 == 0 {
            block[i / 2] & 0x0f
        } else {
            block[i / 2] >> 4
        };
        alpha[i] = nibble * 17;
    }
    let color_block: &[u8; 8] = block[8..]
        .try_into()
        .map_err(|_| DecodeError::InputTooShort)?;
    decode_dxt_color(color_block, output, false);
    for i in 0..16 {
        output[i * 4 + 3] = alpha[i];
    }
    Ok(())
}

fn decode_bc3_inner(block: &[u8; 16], output: &mut [u8]) -> Result<(), DecodeError> {
    let alpha_block: &[u8; 8] = block[..8]
        .try_into()
        .map_err(|_| DecodeError::InputTooShort)?;
    let alpha = decode_bc4_channel(alpha_block);
    let color_block: &[u8; 8] = block[8..]
        .try_into()
        .map_err(|_| DecodeError::InputTooShort)?;
    decode_dxt_color(color_block, output, false);
    for i in 0..16 {
        output[i * 4 + 3] = alpha[i];
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// BC4 / BC5 shared channel decode
// ---------------------------------------------------------------------------

/// Decode a BC4 UNORM channel block into 16 byte values.
fn decode_bc4_channel(block: &[u8; 8]) -> [u8; 16] {
    let a0 = u32::from(block[0]);
    let a1 = u32::from(block[1]);

    let ramp = if a0 > a1 {
        [
            a0,
            a1,
            (6 * a0 + a1 + 3) / 7,
            (5 * a0 + 2 * a1 + 3) / 7,
            (4 * a0 + 3 * a1 + 3) / 7,
            (3 * a0 + 4 * a1 + 3) / 7,
            (2 * a0 + 5 * a1 + 3) / 7,
            (a0 + 6 * a1 + 3) / 7,
        ]
    } else {
        [
            a0,
            a1,
            (4 * a0 + a1 + 2) / 5,
            (3 * a0 + 2 * a1 + 2) / 5,
            (2 * a0 + 3 * a1 + 2) / 5,
            (a0 + 4 * a1 + 2) / 5,
            0,
            255,
        ]
    };

    // 16× 3-bit indices packed across the 48-bit payload that follows the two
    // endpoint bytes. Index 5 straddles the byte boundary between bytes 6 and 7.
    let bits = u64::from_le_bytes([
        block[2], block[3], block[4], block[5], block[6], block[7], 0, 0,
    ]);
    let indices: [u8; 16] = core::array::from_fn(|i| ((bits >> (3 * i)) & 0x7) as u8);

    let mut out = [0_u8; 16];
    for (slot, &index) in out.iter_mut().zip(&indices) {
        *slot = ramp[index as usize] as u8;
    }
    out
}

fn decode_bc4_inner(block: &[u8; 8], output: &mut [u8]) -> Result<(), DecodeError> {
    let r = decode_bc4_channel(block);
    for i in 0..16 {
        output[i * 4] = r[i];
        output[i * 4 + 1] = 0;
        output[i * 4 + 2] = 0;
        output[i * 4 + 3] = 255;
    }
    Ok(())
}

fn decode_bc5_inner(block: &[u8; 16], output: &mut [u8]) -> Result<(), DecodeError> {
    let r_block: &[u8; 8] = block[..8]
        .try_into()
        .map_err(|_| DecodeError::InputTooShort)?;
    let g_block: &[u8; 8] = block[8..]
        .try_into()
        .map_err(|_| DecodeError::InputTooShort)?;
    let r = decode_bc4_channel(r_block);
    let g = decode_bc4_channel(g_block);
    for i in 0..16 {
        output[i * 4] = r[i];
        output[i * 4 + 1] = g[i];
        output[i * 4 + 2] = 0;
        output[i * 4 + 3] = 255;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Bit reader (LSB-first) shared by BC6H and BC7
// ---------------------------------------------------------------------------

struct BitReader<'a> {
    data: &'a [u8; 16],
    pos: usize,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8; 16]) -> Self {
        Self { data, pos: 0 }
    }

    fn read(&mut self, bits: usize) -> u32 {
        let mut value = 0_u32;
        for i in 0..bits {
            let bit = (self.data[(self.pos + i) >> 3] >> ((self.pos + i) & 7)) & 1;
            value |= u32::from(bit) << i;
        }
        self.pos += bits;
        value
    }
}

// ---------------------------------------------------------------------------
// BC6H
// ---------------------------------------------------------------------------

const BC6_U16MAX: i32 = 0xffff;
const BC6_S16MAX: i32 = 0x7fff;

/// Sign-extend `value` (interpreted as `bits` wide) to an `i32`.
#[inline]
fn sign_extend(value: i32, bits: i32) -> i32 {
    let mask = -1 << bits;
    if value & (1 << (bits - 1)) != 0 {
        mask | value
    } else {
        value
    }
}

#[derive(Clone, Copy)]
enum Bc6Format {
    Unsigned,
    Signed,
}

fn bc6_unquantize(format: Bc6Format, q: i32, prec: i32) -> i32 {
    match format {
        Bc6Format::Unsigned => {
            if prec >= 15 {
                q
            } else if q == 0 {
                0
            } else if q == (1 << prec) - 1 {
                BC6_U16MAX
            } else {
                (q * (BC6_U16MAX + 1) + (BC6_U16MAX + 1) / 2) >> prec
            }
        }
        Bc6Format::Signed => {
            if prec >= 16 {
                q
            } else {
                let (s, q_abs) = if q < 0 { (1, -q) } else { (0, q) };
                if q_abs == 0 {
                    0
                } else if q_abs >= (1 << (prec - 1)) - 1 {
                    if s != 0 { -BC6_S16MAX } else { BC6_S16MAX }
                } else {
                    let unq = (q_abs * (BC6_S16MAX + 1) + (BC6_S16MAX + 1) / 2) >> (prec - 1);
                    if s != 0 { -unq } else { unq }
                }
            }
        }
    }
}

fn bc6_finish_unquantize(format: Bc6Format, q: i32) -> i32 {
    match format {
        Bc6Format::Unsigned => (q * 31) >> 6,
        Bc6Format::Signed => {
            if q < 0 {
                -(((-q) * 31) >> 5)
            } else {
                (q * 31) >> 5
            }
        }
    }
}

/// Integer interpolation using the 3-bit (denom 7) or 4-bit (denom 15) weights.
///
/// Uses `+ 32` rounding so the `>> 6` divides by 64 with round-to-nearest,
/// matching bcdec (and the BC6H spec's intended behavior).
fn bc6_lerp(a: i32, b: i32, i: usize, denom: usize) -> i32 {
    // denom 3 reuses the 4-bit weights scaled by 5.
    let weights = if denom == 7 {
        &WEIGHTS3[..]
    } else {
        &WEIGHTS4[..]
    };
    let index_count = denom + 1;
    let wa = weights[index_count - 1 - i] as i32;
    let wb = weights[i] as i32;
    (a * wa + b * wb + 32) >> 6
}

/// Parsed BC6H block, in compressed-endpoint form.
struct Bc6Header {
    two_region: bool,
    transformed: bool,
    w_bits: i32,
    t_bits: [i32; 3],
    shape_index: usize,
    /// Compressed endpoints `[region][A/B][channel]`.
    ec: [[[i32; 3]; 2]; 2],
    indices: [usize; 16],
}

fn parse_bc6(block: &[u8; 16]) -> Result<Bc6Header, DecodeError> {
    // Mode field: if bit 1 is set, read 5 bits; otherwise read 2 bits.
    let decvalue = if block[0] & 0x02 != 0 {
        block[0] & 0x1f
    } else {
        block[0] & 0x01
    } as usize;

    // Map the on-disk decvalue to the canonical m_mode (1..14).
    let m_mode = match decvalue {
        0x00 => 1,
        0x01 => 2,
        0x02 => 3,
        0x06 => 4,
        0x0a => 5,
        0x0e => 6,
        0x12 => 7,
        0x16 => 8,
        0x1a => 9,
        0x1e => 10,
        0x03 => 11,
        0x07 => 12,
        0x0b => 13,
        0x0f => 14,
        _ => return Err(DecodeError::InvalidBlock),
    };

    let (w_bits, t_bits, transformed, two_region) = match BC6_MODES[m_mode] {
        Some(v) => (
            v.0 as i32,
            [v.1[0] as i32, v.1[1] as i32, v.1[2] as i32],
            v.2,
            v.3,
        ),
        None => return Err(DecodeError::InvalidBlock),
    };

    // The two-region modes share a 5-bit shape index at bit 77; one-region modes
    // have no partition bits.
    let shape_index = if two_region {
        // Re-read from a fixed offset to match the reference exactly.
        let mut tmp = BitReader::new(block);
        tmp.pos = 77;
        tmp.read(5) as usize
    } else {
        0
    };

    // Endpoint bit layout differs per mode. Rather than hand-transcribe all 14
    // cases, we reuse the reference's per-field bit offsets by re-reading the
    // raw endpoint words the way `extract_format` does.
    let ec = read_bc6_endpoints(block, m_mode)?;

    // Indices: one-region starts at bit 65 (first 3 bits, rest 4 bits);
    // two-region starts at bit 82 with per-index fixup widths.
    let indices = read_bc6_indices(block, two_region, shape_index)?;

    Ok(Bc6Header {
        two_region,
        transformed,
        w_bits,
        t_bits,
        shape_index,
        ec,
        indices,
    })
}

/// Read the 12 (one-region) or 24 (two-region) compressed endpoint components,
/// matching the per-mode bit offsets used by Compressonator's `extract_format`.
fn read_bc6_endpoints(block: &[u8; 16], mode: usize) -> Result<[[[i32; 3]; 2]; 2], DecodeError> {
    let get = |offset: usize, nbits: usize| -> i32 {
        let mut r = BitReader::new(block);
        r.pos = offset;
        r.read(nbits) as i32
    };
    // Helper to assemble a multi-field value from `(src_offset, src_bits, dst_shift)`.
    let assemble = |parts: &[(usize, usize, u32)]| -> i32 {
        parts.iter().map(|&(o, n, sh)| get(o, n) << sh).sum::<i32>()
    };

    let mut ec = [[[0_i32; 3]; 2]; 2];
    match mode {
        1 => {
            // 10:5:5:5
            ec[0][0][0] = get(5, 10);
            ec[0][1][0] = get(35, 5);
            ec[1][0][0] = get(65, 5);
            ec[1][1][0] = get(71, 5);
            ec[0][0][1] = get(15, 10);
            ec[0][1][1] = get(45, 5);
            ec[1][0][1] = get(41, 4) | (get(2, 1) << 4);
            ec[1][1][1] = get(51, 4) | (get(40, 1) << 4);
            ec[0][0][2] = get(25, 10);
            ec[0][1][2] = get(55, 5);
            ec[1][0][2] = get(61, 4) | (get(3, 1) << 4);
            ec[1][1][2] = assemble(&[(50, 1, 0), (60, 1, 1), (70, 1, 2), (76, 1, 3), (4, 1, 4)]);
        }
        2 => {
            // 7:6:6:6
            ec[0][0][0] = get(5, 7);
            ec[0][1][0] = get(35, 6);
            ec[1][0][0] = get(65, 6);
            ec[1][1][0] = get(71, 6);
            ec[0][0][1] = get(15, 7);
            ec[0][1][1] = get(45, 6);
            ec[1][0][1] = get(41, 4) | (get(24, 1) << 4) | (get(2, 1) << 5);
            ec[1][1][1] = get(51, 4) | (get(3, 1) << 4) | (get(4, 1) << 5);
            ec[0][0][2] = get(25, 7);
            ec[0][1][2] = get(55, 6);
            ec[1][0][2] = get(61, 4) | (get(14, 1) << 4) | (get(22, 1) << 5);
            ec[1][1][2] = assemble(&[
                (12, 1, 0),
                (13, 1, 1),
                (23, 1, 2),
                (32, 1, 3),
                (34, 1, 4),
                (33, 1, 5),
            ]);
        }
        3 => {
            // 11:5:4:4
            ec[0][0][0] = get(5, 10) | (get(40, 1) << 10);
            ec[0][1][0] = get(35, 5);
            ec[1][0][0] = get(65, 5);
            ec[1][1][0] = get(71, 5);
            ec[0][0][1] = get(15, 10) | (get(49, 1) << 10);
            ec[0][1][1] = get(45, 4);
            ec[1][0][1] = get(41, 4);
            ec[1][1][1] = get(51, 4);
            ec[0][0][2] = get(25, 10) | (get(59, 1) << 10);
            ec[0][1][2] = get(55, 4);
            ec[1][0][2] = get(61, 4);
            ec[1][1][2] = assemble(&[(50, 1, 0), (60, 1, 1), (70, 1, 2), (76, 1, 3)]);
        }
        4 => {
            // 11:4:5:4
            ec[0][0][0] = get(5, 10) | (get(39, 1) << 10);
            ec[0][1][0] = get(35, 4);
            ec[1][0][0] = get(65, 4);
            ec[1][1][0] = get(71, 4);
            ec[0][0][1] = get(15, 10) | (get(50, 1) << 10);
            ec[0][1][1] = get(45, 5);
            ec[1][0][1] = get(41, 4) | (get(75, 1) << 4);
            ec[1][1][1] = get(51, 4) | (get(40, 1) << 4);
            ec[0][0][2] = get(25, 10) | (get(59, 1) << 10);
            ec[0][1][2] = get(55, 4);
            ec[1][0][2] = get(61, 4);
            ec[1][1][2] = assemble(&[(69, 1, 0), (60, 1, 1), (70, 1, 2), (76, 1, 3)]);
        }
        5 => {
            // 11:4:4:5
            ec[0][0][0] = get(5, 10) | (get(39, 1) << 10);
            ec[0][1][0] = get(35, 4);
            ec[1][0][0] = get(65, 4);
            ec[1][1][0] = get(71, 4);
            ec[0][0][1] = get(15, 10) | (get(49, 1) << 10);
            ec[0][1][1] = get(45, 4);
            ec[1][0][1] = get(41, 4);
            ec[1][1][1] = get(51, 4);
            ec[0][0][2] = get(25, 10) | (get(60, 1) << 10);
            ec[0][1][2] = get(55, 5);
            ec[1][0][2] = get(61, 4) | (get(40, 1) << 4);
            ec[1][1][2] = assemble(&[(50, 1, 0), (69, 1, 1), (70, 1, 2), (76, 1, 3), (75, 1, 4)]);
        }
        6 => {
            // 9:5:5:5
            ec[0][0][0] = get(5, 9);
            ec[0][1][0] = get(35, 5);
            ec[1][0][0] = get(65, 5);
            ec[1][1][0] = get(71, 5);
            ec[0][0][1] = get(15, 9);
            ec[0][1][1] = get(45, 5);
            ec[1][0][1] = get(41, 4) | (get(24, 1) << 4);
            ec[1][1][1] = get(51, 4) | (get(40, 1) << 4);
            ec[0][0][2] = get(25, 9);
            ec[0][1][2] = get(55, 5);
            ec[1][0][2] = get(61, 4) | (get(14, 1) << 4);
            ec[1][1][2] = assemble(&[(50, 1, 0), (60, 1, 1), (70, 1, 2), (76, 1, 3), (34, 1, 4)]);
        }
        7 => {
            // 8:6:5:5
            ec[0][0][0] = get(5, 8);
            ec[0][1][0] = get(35, 6);
            ec[1][0][0] = get(65, 6);
            ec[1][1][0] = get(71, 6);
            ec[0][0][1] = get(15, 8);
            ec[0][1][1] = get(45, 5);
            ec[1][0][1] = get(41, 4) | (get(24, 1) << 4);
            ec[1][1][1] = get(51, 4) | (get(13, 1) << 4);
            ec[0][0][2] = get(25, 8);
            ec[0][1][2] = get(55, 5);
            ec[1][0][2] = get(61, 4) | (get(14, 1) << 4);
            ec[1][1][2] = assemble(&[(50, 1, 0), (60, 1, 1), (23, 1, 2), (33, 1, 3), (34, 1, 4)]);
        }
        8 => {
            // 8:5:6:5
            ec[0][0][0] = get(5, 8);
            ec[0][1][0] = get(35, 5);
            ec[1][0][0] = get(65, 5);
            ec[1][1][0] = get(71, 5);
            ec[0][0][1] = get(15, 8);
            ec[0][1][1] = get(45, 6);
            ec[1][0][1] = get(41, 4) | (get(24, 1) << 4) | (get(23, 1) << 5);
            ec[1][1][1] = get(51, 4) | (get(40, 1) << 4) | (get(33, 1) << 5);
            ec[0][0][2] = get(25, 8);
            ec[0][1][2] = get(55, 5);
            ec[1][0][2] = get(61, 4) | (get(14, 1) << 4);
            ec[1][1][2] = assemble(&[(13, 1, 0), (60, 1, 1), (70, 1, 2), (76, 1, 3), (34, 1, 4)]);
        }
        9 => {
            // 8:5:5:6
            ec[0][0][0] = get(5, 8);
            ec[0][1][0] = get(35, 5);
            ec[1][0][0] = get(65, 5);
            ec[1][1][0] = get(71, 5);
            ec[0][0][1] = get(15, 8);
            ec[0][1][1] = get(45, 5);
            ec[1][0][1] = get(41, 4) | (get(24, 1) << 4);
            ec[1][1][1] = get(51, 4) | (get(40, 1) << 4);
            ec[0][0][2] = get(25, 8);
            ec[0][1][2] = get(55, 6);
            ec[1][0][2] = get(61, 4) | (get(14, 1) << 4) | (get(23, 1) << 5);
            ec[1][1][2] = assemble(&[
                (50, 1, 0),
                (13, 1, 1),
                (70, 1, 2),
                (76, 1, 3),
                (34, 1, 4),
                (33, 1, 5),
            ]);
        }
        10 => {
            // 6:6:6:6 (not transformed)
            ec[0][0][0] = get(5, 6);
            ec[0][1][0] = get(35, 6);
            ec[1][0][0] = get(65, 6);
            ec[1][1][0] = get(71, 6);
            ec[0][0][1] = get(15, 6);
            ec[0][1][1] = get(45, 6);
            ec[1][0][1] = get(41, 4) | (get(24, 1) << 4) | (get(21, 1) << 5);
            ec[1][1][1] = get(51, 4) | (get(11, 1) << 4) | (get(31, 1) << 5);
            ec[0][0][2] = get(25, 6);
            ec[0][1][2] = get(55, 6);
            ec[1][0][2] = get(61, 4) | (get(14, 1) << 4) | (get(22, 1) << 5);
            ec[1][1][2] = assemble(&[
                (12, 1, 0),
                (13, 1, 1),
                (23, 1, 2),
                (32, 1, 3),
                (34, 1, 4),
                (33, 1, 5),
            ]);
        }
        11 => {
            // 10:10 (one region, not transformed)
            ec[0][0][0] = get(5, 10);
            ec[0][1][0] = get(35, 10);
            ec[0][0][1] = get(15, 10);
            ec[0][1][1] = get(45, 10);
            ec[0][0][2] = get(25, 10);
            ec[0][1][2] = get(55, 10);
        }
        12 => {
            // 11:9 (one region, transformed)
            ec[0][0][0] = get(5, 10) | (get(44, 1) << 10);
            ec[0][1][0] = get(35, 9);
            ec[0][0][1] = get(15, 10) | (get(54, 1) << 10);
            ec[0][1][1] = get(45, 9);
            ec[0][0][2] = get(25, 10) | (get(64, 1) << 10);
            ec[0][1][2] = get(55, 9);
        }
        13 => {
            // 12:8 (one region, transformed)
            ec[0][0][0] = get(5, 10) | (get(44, 1) << 10) | (get(43, 1) << 11);
            ec[0][1][0] = get(35, 8);
            ec[0][0][1] = get(15, 10) | (get(54, 1) << 10) | (get(53, 1) << 11);
            ec[0][1][1] = get(45, 8);
            ec[0][0][2] = get(25, 10) | (get(64, 1) << 10) | (get(63, 1) << 11);
            ec[0][1][2] = get(55, 8);
        }
        14 => {
            // 16:4 (one region, transformed)
            ec[0][0][0] = assemble(&[
                (5, 10, 0),
                (44, 1, 10),
                (43, 1, 11),
                (42, 1, 12),
                (41, 1, 13),
                (40, 1, 14),
                (39, 1, 15),
            ]);
            ec[0][1][0] = get(35, 4);
            ec[0][0][1] = assemble(&[
                (15, 10, 0),
                (54, 1, 10),
                (53, 1, 11),
                (52, 1, 12),
                (51, 1, 13),
                (50, 1, 14),
                (49, 1, 15),
            ]);
            ec[0][1][1] = get(45, 4);
            ec[0][0][2] = assemble(&[
                (25, 10, 0),
                (64, 1, 10),
                (63, 1, 11),
                (62, 1, 12),
                (61, 1, 13),
                (60, 1, 14),
                (59, 1, 15),
            ]);
            ec[0][1][2] = get(55, 4);
        }
        _ => return Err(DecodeError::InvalidBlock),
    }
    Ok(ec)
}

fn read_bc6_indices(
    block: &[u8; 16],
    two_region: bool,
    shape_index: usize,
) -> Result<[usize; 16], DecodeError> {
    let mut reader = BitReader::new(block);
    let mut indices = [0_usize; 16];
    if !two_region {
        reader.pos = 65;
        indices[0] = reader.read(3) as usize;
        for entry in &mut indices[1..] {
            *entry = reader.read(4) as usize;
        }
    } else {
        reader.pos = 82;
        indices[0] = reader.read(2) as usize;
        let fixup = BC6_INDEX_FIXUPS[shape_index];
        for (i, slot) in indices.iter_mut().enumerate().skip(1) {
            let nbits = if fixup == i { 2 } else { 3 };
            *slot = reader.read(nbits) as usize;
        }
    }
    Ok(indices)
}

/// Resolve transformed/signed endpoints into unquantized endpoint pairs.
///
/// Returns `[region][endpoint A/B][channel]` unquantized values.
fn bc6_resolve_endpoints(header: &Bc6Header, format: Bc6Format) -> [[[i32; 3]; 2]; 2] {
    let w_bits = header.w_bits;
    let t_bits = header.t_bits;
    let mask = |bits: i32| -> i32 { (1 << bits) - 1 };
    let signed = matches!(format, Bc6Format::Signed);

    // Resolve a single endpoint component. `raw` is the on-disk value, `prec` its
    // bit width, and `base` is the root (region-0 A) value the transform deltas
    // against. When `transformed` is false the value is read directly.
    let resolve = |raw: i32, prec: i32, base: i32, transformed: bool| -> i32 {
        if transformed {
            let delta = sign_extend(raw, prec);
            let combined = (delta + base) & mask(w_bits);
            if signed {
                sign_extend(combined, w_bits)
            } else {
                combined
            }
        } else if signed {
            sign_extend(raw, prec)
        } else {
            raw
        }
    };

    let mut e = [[[0_i32; 3]; 2]; 2];
    let region_count = if header.two_region { 2 } else { 1 };
    for c in 0..3 {
        let a0 = header.ec[0][0][c];
        // The root endpoint A of region 0 is always read at w_bits precision.
        let ea0 = if signed { sign_extend(a0, w_bits) } else { a0 };
        e[0][0][c] = ea0;
        e[0][1][c] = resolve(header.ec[0][1][c], t_bits[c], a0, header.transformed);
        if region_count == 2 {
            e[1][0][c] = resolve(header.ec[1][0][c], t_bits[c], a0, header.transformed);
            e[1][1][c] = resolve(header.ec[1][1][c], t_bits[c], a0, header.transformed);
        }
    }
    e
}

fn decode_bc6h_block_with(
    block: &[u8; 16],
    format: Bc6Format,
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let header = parse_bc6(block)?;
    let endpoints = bc6_resolve_endpoints(&header, format);

    // Build the palette of finish_unquantize'd binary16 bit patterns (signed).
    let palette_count = if header.two_region { 8 } else { 16 };
    let denom = palette_count - 1;
    let region_count = if header.two_region { 2 } else { 1 };
    let mut palette = [[[0_i32; 3]; 16]; 2];
    for region in 0..region_count {
        for c in 0..3 {
            let a = bc6_unquantize(format, endpoints[region][0][c], header.w_bits);
            let b = bc6_unquantize(format, endpoints[region][1][c], header.w_bits);
            for (i, slot) in palette[region].iter_mut().enumerate().take(palette_count) {
                let lerp_val = if denom == 15 {
                    bc6_lerp(a, b, i, 15)
                } else {
                    bc6_lerp(a, b, i, 7)
                };
                slot[c] = bc6_finish_unquantize(format, lerp_val);
            }
        }
    }

    for pixel in 0..16 {
        let region = if header.two_region {
            BC6_PARTITIONS[header.shape_index][pixel] as usize
        } else {
            0
        };
        let index = header.indices[pixel];
        let rgb = palette[region][index];
        for c in 0..3 {
            output[pixel * 4 + c] = bc6_value_to_u8(rgb[c]);
        }
        output[pixel * 4 + 3] = 255;
    }
    Ok(())
}

/// Map a finish_unquantize'd signed integer (magnitude = binary16 bits, sign
/// applied separately as the float does) into an RGBA8 byte using the crate
/// convention: `<= 0 -> 0`, `>= 1 -> 255`, else `floor(v * 255 + 0.5)`.
fn bc6_value_to_u8(value: i32) -> u8 {
    let bits = (value.unsigned_abs()) as u16;
    let f = f16::from_bits(bits).to_f32();
    let signed = if value < 0 { -f } else { f };
    if signed <= 0.0 {
        0
    } else if signed >= 1.0 {
        255
    } else {
        (signed * 255.0 + 0.5) as u8
    }
}

fn decode_bc6h_uf16_inner(block: &[u8; 16], output: &mut [u8]) -> Result<(), DecodeError> {
    decode_bc6h_block_with(block, Bc6Format::Unsigned, output)
}

fn decode_bc6h_sf16_inner(block: &[u8; 16], output: &mut [u8]) -> Result<(), DecodeError> {
    decode_bc6h_block_with(block, Bc6Format::Signed, output)
}

// ---------------------------------------------------------------------------
// BC7
// ---------------------------------------------------------------------------

#[inline]
fn bc7_interpolate(e0: u32, e1: u32, weight: u32) -> u8 {
    (((64 - weight) * e0 + weight * e1 + 32) >> 6) as u8
}

/// Generate a ramp for one component using the 2/3/4-bit weight tables.
fn bc7_ramp_component(e0: u32, e1: u32, index_bits: u32, out: &mut [u8; 16]) {
    let weights = match index_bits {
        2 => &WEIGHTS2[..],
        3 => &WEIGHTS3[..],
        4 => &WEIGHTS4[..],
        _ => return,
    };
    let count = 1 << index_bits;
    for i in 0..count as usize {
        out[i] = bc7_interpolate(e0, e1, weights[i]);
    }
}

/// Expand a `bits`-wide endpoint component to 8 bits via MSB replication.
#[inline]
fn bc7_expand(value: u32, bits: u32) -> u32 {
    if bits == 0 {
        return 0;
    }
    let shifted = value << (8 - bits);
    shifted | (shifted >> bits)
}

const COMP_RED: usize = 0;
const COMP_GREEN: usize = 1;
const COMP_BLUE: usize = 2;
const COMP_ALPHA: usize = 3;

const NO_ALPHA: u32 = 0;
const COMBINED_ALPHA: u32 = 1;
const SEPARATE_ALPHA: u32 = 2;

const PBIT_NONE: u32 = 0;
const PBIT_ONE: u32 = 1;

fn decode_bc7_inner(block: &[u8; 16], output: &mut [u8]) -> Result<(), DecodeError> {
    // Mode = position of the first set bit among the first 8 bits.
    let mut mode = 0;
    loop {
        if block[mode >> 3] & (1 << (mode & 7)) != 0 {
            break;
        }
        mode += 1;
        if mode >= 8 {
            return Err(DecodeError::InvalidBlock);
        }
    }

    let mut reader = BitReader::new(block);
    reader.pos = mode + 1; // skip the mode unary bits.

    let mode_desc = &BC7_MODES[mode];

    let rotation = reader.read(mode_desc.rotation_bits as usize);
    let index_swap = reader.read(mode_desc.index_mode_bits as usize);
    let partition = reader.read(mode_desc.partition_bits as usize);

    // Component bit widths.
    let mut component_bits = [0_u32; 4];
    match mode_desc.encoding {
        NO_ALPHA => {
            let each = mode_desc.vector_bits / 3;
            component_bits[COMP_RED] = each;
            component_bits[COMP_GREEN] = each;
            component_bits[COMP_BLUE] = each;
            component_bits[COMP_ALPHA] = 0;
        }
        COMBINED_ALPHA => {
            let each = mode_desc.vector_bits / 4;
            component_bits[COMP_RED] = each;
            component_bits[COMP_GREEN] = each;
            component_bits[COMP_BLUE] = each;
            component_bits[COMP_ALPHA] = each;
        }
        SEPARATE_ALPHA => {
            let each = mode_desc.vector_bits / 3;
            component_bits[COMP_RED] = each;
            component_bits[COMP_GREEN] = each;
            component_bits[COMP_BLUE] = each;
            component_bits[COMP_ALPHA] = mode_desc.scalar_bits;
        }
        _ => return Err(DecodeError::InvalidBlock),
    }

    let subset_count = mode_desc.subset_count as usize;
    // Endpoints [subset][endpoint 0/1][component].
    let mut endpoint = [[[0_u32; 4]; 2]; 3];
    for (component, &bits) in component_bits.iter().enumerate() {
        for subset_ep in endpoint.iter_mut().take(subset_count) {
            for ep_slot in subset_ep {
                ep_slot[component] = reader.read(bits as usize);
            }
        }
    }

    // Parity bits: shared (PBIT_ONE) or per-endpoint, applied to all components.
    if mode_desc.p_bit_type != PBIT_NONE {
        for subset_ep in endpoint.iter_mut().take(subset_count) {
            let (p0, p1) = if mode_desc.p_bit_type == PBIT_ONE {
                let p = reader.read(1);
                (p, p)
            } else {
                // One parity bit per endpoint (TWO_PBIT modes).
                (reader.read(1), reader.read(1))
            };
            for component in 0..4 {
                if component_bits[component] != 0 {
                    subset_ep[0][component] = (subset_ep[0][component] << 1) | p0;
                    subset_ep[1][component] = (subset_ep[1][component] << 1) | p1;
                }
            }
        }
        // Component widths now include the p-bit for the ramp generator.
        for bits in component_bits.iter_mut() {
            if *bits != 0 {
                *bits += 1;
            }
        }
    }

    if mode_desc.encoding == SEPARATE_ALPHA {
        decode_bc7_dual_index(
            &mut reader,
            endpoint,
            mode,
            rotation,
            index_swap,
            component_bits,
            output,
        )?;
        return Ok(());
    }

    // Partition table and fixup anchors.
    let partition_table = &BC7_PARTITIONS[subset_count - 1][partition as usize];
    let mut fixup = [0_usize; 3];
    match subset_count {
        3 => {
            fixup[1] = BC7_FIXUP_INDICES[2][partition as usize][1];
            fixup[2] = BC7_FIXUP_INDICES[2][partition as usize][2];
        }
        2 => {
            fixup[1] = BC7_FIXUP_INDICES[1][partition as usize][1];
        }
        _ => {}
    }

    let index_bits = mode_desc.index_bits[0] as usize;
    let mut block_indices = [0_u32; 16];
    for i in 0..16 {
        let p = partition_table[i] as usize;
        let mut bits_to_read = index_bits;
        if i == fixup[p] {
            bits_to_read -= 1;
        }
        block_indices[i] = reader.read(bits_to_read);
    }

    // Build the colour ramps for each subset.
    let mut ramps = [[[0_u8; 16]; 4]; 3];
    let clusters = 1usize << index_bits;
    for subset in 0..subset_count {
        let e0 = [
            bc7_expand(endpoint[subset][0][COMP_RED], component_bits[COMP_RED]),
            bc7_expand(endpoint[subset][0][COMP_GREEN], component_bits[COMP_GREEN]),
            bc7_expand(endpoint[subset][0][COMP_BLUE], component_bits[COMP_BLUE]),
            bc7_expand(endpoint[subset][0][COMP_ALPHA], component_bits[COMP_ALPHA]),
        ];
        let e1 = [
            bc7_expand(endpoint[subset][1][COMP_RED], component_bits[COMP_RED]),
            bc7_expand(endpoint[subset][1][COMP_GREEN], component_bits[COMP_GREEN]),
            bc7_expand(endpoint[subset][1][COMP_BLUE], component_bits[COMP_BLUE]),
            bc7_expand(endpoint[subset][1][COMP_ALPHA], component_bits[COMP_ALPHA]),
        ];
        for component in 0..4 {
            if component_bits[component] == 0 {
                let fill = if component == COMP_ALPHA { 255 } else { 0 };
                for slot in ramps[subset][component].iter_mut().take(clusters) {
                    *slot = fill;
                }
            } else {
                bc7_ramp_component(
                    e0[component],
                    e1[component],
                    index_bits as u32,
                    &mut ramps[subset][component],
                );
            }
        }
    }

    for i in 0..16 {
        let subset = partition_table[i] as usize;
        let idx = block_indices[i] as usize;
        output[i * 4 + COMP_RED] = ramps[subset][COMP_RED][idx];
        output[i * 4 + COMP_GREEN] = ramps[subset][COMP_GREEN][idx];
        output[i * 4 + COMP_BLUE] = ramps[subset][COMP_BLUE][idx];
        output[i * 4 + COMP_ALPHA] = ramps[subset][COMP_ALPHA][idx];
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn decode_bc7_dual_index(
    reader: &mut BitReader,
    endpoint: [[[u32; 4]; 2]; 3],
    mode: usize,
    rotation: u32,
    index_swap: u32,
    component_bits: [u32; 4],
    output: &mut [u8],
) -> Result<(), DecodeError> {
    let mode_desc = &BC7_MODES[mode];
    let mut clusters = [0_usize; 2];
    clusters[0] = 1 << mode_desc.index_bits[0];
    clusters[1] = 1 << mode_desc.index_bits[1];
    if index_swap != 0 {
        clusters.swap(0, 1);
    }

    // Ramp for color (RGB) uses index_bits[0] cluster count (after swap),
    // alpha uses index_bits[1].
    let mut ramp_rgb = [[0_u8; 16]; 3];
    let mut ramp_a = [0_u8; 16];
    let e0 = [
        bc7_expand(endpoint[0][0][COMP_RED], component_bits[COMP_RED]),
        bc7_expand(endpoint[0][0][COMP_GREEN], component_bits[COMP_GREEN]),
        bc7_expand(endpoint[0][0][COMP_BLUE], component_bits[COMP_BLUE]),
        bc7_expand(endpoint[0][0][COMP_ALPHA], component_bits[COMP_ALPHA]),
    ];
    let e1 = [
        bc7_expand(endpoint[0][1][COMP_RED], component_bits[COMP_RED]),
        bc7_expand(endpoint[0][1][COMP_GREEN], component_bits[COMP_GREEN]),
        bc7_expand(endpoint[0][1][COMP_BLUE], component_bits[COMP_BLUE]),
        bc7_expand(endpoint[0][1][COMP_ALPHA], component_bits[COMP_ALPHA]),
    ];

    let rgb_bits = if clusters[0] == 4 {
        2
    } else if clusters[0] == 8 {
        3
    } else {
        4
    };
    let alpha_bits = if clusters[1] == 4 {
        2
    } else if clusters[1] == 8 {
        3
    } else {
        4
    };
    for component in 0..3 {
        bc7_ramp_component(
            e0[component],
            e1[component],
            rgb_bits,
            &mut ramp_rgb[component],
        );
    }
    bc7_ramp_component(e0[COMP_ALPHA], e1[COMP_ALPHA], alpha_bits, &mut ramp_a);

    // Read two independent index sets; index 0 of each has the fixup bit cleared.
    let index_swap = index_swap as usize;
    let mut block_indices = [[0_u32; 16]; 2];
    for (set, indices) in block_indices.iter_mut().enumerate() {
        let bits = mode_desc.index_bits[set] as usize;
        for (j, slot) in indices.iter_mut().enumerate() {
            let n = if j == 0 { bits - 1 } else { bits };
            *slot = reader.read(n);
        }
    }

    let color_set = index_swap;
    let alpha_set = index_swap ^ 1;
    for i in 0..16 {
        let c = block_indices[color_set][i] as usize;
        let a = block_indices[alpha_set][i] as usize;
        output[i * 4 + COMP_RED] = ramp_rgb[COMP_RED][c];
        output[i * 4 + COMP_GREEN] = ramp_rgb[COMP_GREEN][c];
        output[i * 4 + COMP_BLUE] = ramp_rgb[COMP_BLUE][c];
        output[i * 4 + COMP_ALPHA] = ramp_a[a];
    }

    // Resolve component rotation.
    match rotation {
        0 => {}
        1 => {
            for i in 0..16 {
                output.swap(i * 4 + COMP_ALPHA, i * 4 + COMP_RED);
            }
        }
        2 => {
            for i in 0..16 {
                output.swap(i * 4 + COMP_ALPHA, i * 4 + COMP_GREEN);
            }
        }
        3 => {
            for i in 0..16 {
                output.swap(i * 4 + COMP_ALPHA, i * 4 + COMP_BLUE);
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_short_output() {
        let block = [0_u8; 8];
        let mut output = [0xa5_u8; 3];
        assert_eq!(
            decode_bc1_block(&block, &mut output),
            Err(DecodeError::OutputTooShort)
        );
        assert_eq!(output, [0xa5; 3]);
    }

    #[test]
    fn bc1_three_color_transparent_selector() {
        // n0 == n1 -> three-color mode; all selector bits set -> selector 3 -> transparent black.
        let block = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let mut output = [0_u8; 64];
        decode_bc1_block(&block, &mut output).unwrap();
        // Every 2-bit index of 0xffffffff is 3.
        assert!(output.chunks_exact(4).all(|px| px == [0, 0, 0, 0]));
    }

    #[test]
    fn bc1_four_color_endpoint_replication() {
        // n0 = 0xffff (white), n1 = 0x0000 (black); n0 > n1 -> four-color.
        let block = [0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut output = [0_u8; 64];
        decode_bc1_block(&block, &mut output).unwrap();
        assert_eq!(&output[..4], &[255, 255, 255, 255]); // index 0 -> c0 white
    }

    #[test]
    fn bc2_alpha_expansion() {
        // Color = white (0xffff/0xffff -> four-color), alpha nibbles = 0xf/0x0 alternating.
        let mut block = [0_u8; 16];
        block[8..16].copy_from_slice(&[0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00]);
        block[0] = 0x0f; // pixel 0 alpha nibble = 0xf -> 255, pixel 1 = 0 -> 0
        let mut output = [0_u8; 64];
        decode_bc2_block(&block, &mut output).unwrap();
        assert_eq!(output[3], 255);
        assert_eq!(output[7], 0);
    }

    #[test]
    fn bc4_channel_fills_red_only() {
        // a0=255, a1=0 -> 8-interp ramp; all selectors 0 -> a0.
        let block = [255, 0, 0, 0, 0, 0, 0, 0];
        let mut output = [0_u8; 64];
        decode_bc4_block(&block, &mut output).unwrap();
        assert!(output.chunks_exact(4).all(|px| px == [255, 0, 0, 255]));
    }

    #[test]
    fn bc5_two_channels() {
        // R channel a0=255, G channel a0=128.
        let mut block = [0_u8; 16];
        block[0] = 255;
        block[8] = 128;
        let mut output = [0_u8; 64];
        decode_bc5_block(&block, &mut output).unwrap();
        assert_eq!(&output[..4], &[255, 128, 0, 255]);
    }

    #[test]
    fn bc7_all_zero_mode_is_invalid() {
        let block = [0_u8; 16];
        let mut output = [0_u8; 64];
        assert_eq!(
            decode_bc7_block(&block, &mut output),
            Err(DecodeError::InvalidBlock)
        );
    }

    #[test]
    fn bc6h_reserved_mode_is_invalid() {
        // Bit 1 set -> read 5-bit mode field. decvalue 0x04 (0b00100) is not a
        // valid BC6H mode, so the block must be rejected.
        let mut block = [0_u8; 16];
        block[0] = 0b0001_0011; // decvalue = 0x13 (19) -> invalid
        let mut output = [0_u8; 64];
        assert_eq!(
            decode_bc6h_uf16_block(&block, &mut output),
            Err(DecodeError::InvalidBlock)
        );
    }

    #[test]
    fn bc6_value_to_u8_boundary() {
        // binary16 0x3c00 = 1.0, 0x3800 = 0.5, 0xbc00 = -1.0
        assert_eq!(bc6_value_to_u8(0x3c00), 255);
        assert_eq!(bc6_value_to_u8(-0x3c00), 0);
        assert_eq!(bc6_value_to_u8(0), 0);
        // 0x3800 = 0.5 -> floor(0.5*255 + 0.5) = floor(128.0) = 128
        assert_eq!(bc6_value_to_u8(0x3800), 128);
    }

    #[test]
    fn arbitrary_bc_blocks_do_not_panic() {
        let mut state = 0x6a09_e667_f3bc_c909_u64;
        let mut output8 = [0_u8; 64];
        let mut output16 = [0_u8; 64];
        for _ in 0..5000 {
            let mut block8 = [0_u8; 8];
            let mut block16 = [0_u8; 16];
            for byte in block8.iter_mut() {
                state ^= state << 7;
                state ^= state >> 9;
                state ^= state << 8;
                *byte = state as u8;
            }
            for byte in block16.iter_mut() {
                state ^= state << 7;
                state ^= state >> 9;
                state ^= state << 8;
                *byte = state as u8;
            }
            let _ = decode_bc1_block(&block8, &mut output8);
            let _ = decode_bc2_block(&block16, &mut output16);
            let _ = decode_bc3_block(&block16, &mut output16);
            let _ = decode_bc4_block(&block8, &mut output8);
            let _ = decode_bc5_block(&block16, &mut output16);
            let _ = decode_bc6h_uf16_block(&block16, &mut output16);
            let _ = decode_bc6h_sf16_block(&block16, &mut output16);
            let _ = decode_bc7_block(&block16, &mut output16);
        }
    }
}
