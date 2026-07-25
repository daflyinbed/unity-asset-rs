//! Crunch (CRN v1.04) decompression to raw BC1/BC3 blocks.
//!
//! This is a direct port of the decompression path in
//! `crunch/inc/crn_decomp.h` (crnlib v1.04, by Richard Geldreich, public
//! domain). It implements the subset needed to transcode every face and mip
//! level of a CRN file into raw DXT1 (BC1, 8 bytes/block) or DXT5 (BC3,
//! 16 bytes/block) blocks.
//!
//! Pipeline:
//!
//! 1. Parse the big-endian [`Header`] and validate it.
//! 2. `init_tables`: decode the static Huffman codebooks stored in the
//!    `tables` section (used to encode the per-chunk tile layout and the
//!    palette index deltas).
//! 3. `decode_palettes`: decode the four delta-coded endpoint/selector
//!    palettes, each from its own independently coded byte range.
//! 4. For each mip, walk every face and 8x8 chunk in zig-zag order, reading
//!    tile-layout and palette-index deltas, and assemble raw BC blocks from
//!    palette lookups. The bitstream and predictors are continuous across the
//!    faces of one mip, then reset for the next mip.
//!
//! All multi-byte header fields are big-endian (`crn_packed_uint`). The bit
//! streams (tables, palettes, level data) are MSB-first.

use alloc::vec;
use alloc::vec::Vec;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// The Crunch texture format decoded from the file header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrnFormat {
    /// DXT1 / BC1, 8 bytes per block.
    Bc1,
    /// DXT5 / BC3, 16 bytes per block.
    Bc3,
}

/// A decoded Crunch texture containing one 2D face or all six cubemap faces.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrnTexture {
    /// The level-0 width. A zero header width is clamped to one.
    pub width: u32,
    /// The level-0 height. A zero header height is clamped to one.
    pub height: u32,
    pub format: CrnFormat,
    /// One face for a 2D texture, or cubemap faces in +X, -X, +Y, -Y, +Z, -Z
    /// order.
    pub faces: Vec<CrnFace>,
}

/// All mip levels belonging to one texture face.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrnFace {
    /// Mips ordered largest to smallest, starting with level 0.
    pub levels: Vec<CrnMipLevel>,
}

/// One decoded mip level containing tightly packed physical BC blocks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrnMipLevel {
    pub width: u32,
    pub height: u32,
    /// `ceil(width / 4) * ceil(height / 4) * block_bytes` bytes, row-major in
    /// block space. `block_bytes` is 8 for BC1 and 16 for BC3.
    pub data: Vec<u8>,
}

/// A failure while decoding a Crunch file.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrnDecodeError {
    /// The compressed input is too short to hold the header or declared data.
    InputTooShort,
    /// A dimension or byte-count calculation overflowed.
    SizeOverflow,
    /// The file signature is not `0x4878` (`"Hx"`).
    InvalidSignature,
    /// The CRN format is not one of the supported DXT1/DXT5 variants.
    UnsupportedFormat,
    /// A header field has an illegal value (faces, offsets, missing palettes,
    /// ...).
    InvalidHeader,
    /// The file uses the segmented CRN variant, which stores mip data
    /// out-of-line and is not supported here.
    UnsupportedSegmented,
    /// The Huffman/bit stream is corrupt or ran past its declared range.
    InvalidStream,
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// CRN file signature: `('H' << 8) | 'x'` = `0x4878`.
const CRN_SIG: u32 = 0x4878;

/// `cCRNHeaderFlagSegmented` — mip data is stored out-of-line.
const CRN_HEADER_FLAG_SEGMENTED: u32 = 1;

/// Maximum number of mip levels representable by the Crunch API.
const CRN_MAX_LEVELS: u32 = 16;

/// `cCRNMaxLevelResolution` from `crnlib.h`.
const CRN_MAX_LEVEL_RESOLUTION: u32 = 4096;

/// Initial validation threshold from `cCRNHeaderMinSize` in `crn_decomp.h`.
/// The complete variable header is checked separately below.
const CRN_HEADER_MIN_SIZE: usize = 62;

/// Byte offset of the variably-sized `m_level_ofs[]` array within the header.
/// Everything before it is fixed; each mip level appends one big-endian `u32`.
const LEVEL_OFS_OFFSET: usize = 70;

// CRN format enum values (from `inc/crnlib.h`).
const FMT_DXT1: u8 = 0;
const FMT_DXT5: u8 = 2;
// DXT5 swizzled variants unpack identically to plain DXT5.
const FMT_DXT5_CCXY: u8 = 3;
const FMT_DXT5_XGXR: u8 = 4;
const FMT_DXT5_XGBR: u8 = 5;
const FMT_DXT5_AGBR: u8 = 6;

// Prefix-coding limits (from `prefix_coding` in `crn_decomp.h`).
const MAX_EXPECTED_CODE_SIZE: usize = 16;
const MAX_SUPPORTED_SYMS: u32 = 8192;
const MAX_TABLE_BITS: u32 = 11;

/// `math::total_bits`: the number of bits needed to represent `v`, counting
/// right-shifts to zero. For 8192 (= 2^13) this is 14, not 13, which is why the
/// symbol count read by `decode_receive_static_data_model` is 14 bits wide.
#[inline]
fn total_bits(v: u32) -> u32 {
    let mut l = 0u32;
    let mut v = v;
    while v > 0 {
        v >>= 1;
        l += 1;
    }
    l
}

/// `math::ceil_log2i`: ceil(log2(v)). For exact powers of two this returns the
/// exponent (ceil_log2i(32) = 5), matching the C++ reference.
#[inline]
fn ceil_log2i(v: u32) -> u32 {
    let mut l = 0u32;
    let mut x = v;
    while x > 1 {
        x >>= 1;
        l += 1;
    }
    if v > (1u32 << l) { l + 1 } else { l }
}

// Code-length RLE symbols (deflate-like). See `symbol_codec::decode_receive_static_data_model`.
const SMALL_ZERO_RUN_CODE: u32 = 17;
const LARGE_ZERO_RUN_CODE: u32 = 18;
const SMALL_REPEAT_CODE: u32 = 19;
const LARGE_REPEAT_CODE: u32 = 20;

const SMALL_ZERO_RUN_EXTRA_BITS: u32 = 3;
const LARGE_ZERO_RUN_EXTRA_BITS: u32 = 7;
const SMALL_NONZERO_RUN_EXTRA_BITS: u32 = 2;
const LARGE_NONZERO_RUN_EXTRA_BITS: u32 = 6;

const MIN_SMALL_ZERO_RUN_SIZE: u32 = 3;
const MIN_LARGE_ZERO_RUN_SIZE: u32 = 11;
const SMALL_MIN_NONZERO_RUN_SIZE: u32 = 3;
const LARGE_MIN_NONZERO_RUN_SIZE: u32 = 7;

/// Order in which meta-Huffman code lengths are transmitted.
/// `g_most_probable_codelength_codes` in `crn_decomp.h`.
const MOST_PROBABLE_CODELENGTH_CODES: [u8; 21] = [
    17, 18, 19, 20, // small/large zero-run, small/large repeat
    0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15, 16,
];

/// `g_crnd_chunk_encoding_num_tiles`.
const CHUNK_ENCODING_NUM_TILES: [u32; 8] = [1, 2, 2, 3, 3, 3, 3, 4];

/// `g_crnd_chunk_encoding_tiles`: maps the 4 block positions (bx + by*2) within
/// an 8x8 chunk to a tile index, per chunk encoding.
const CHUNK_ENCODING_TILES: [[u8; 4]; 8] = [
    [0, 0, 0, 0],
    [0, 0, 1, 1],
    [0, 1, 0, 1],
    [0, 0, 1, 2],
    [1, 2, 0, 0],
    [0, 1, 0, 2],
    [1, 0, 2, 0],
    [0, 1, 2, 3],
];

/// `g_dxt1_from_linear`: BC1 selector remap applied at color-selector palette
/// decode time.
const DXT1_FROM_LINEAR: [u32; 4] = [0, 2, 3, 1];

/// `g_dxt5_from_linear`: BC4/BC5 alpha selector remap applied at alpha-selector
/// palette decode time.
const DXT5_FROM_LINEAR: [u32; 8] = [0, 2, 3, 4, 5, 6, 7, 1];

// ---------------------------------------------------------------------------
// Big-endian packed-uint header reader
// ---------------------------------------------------------------------------

/// Reads a big-endian packed uint of `N` bytes (1..=4) at `offset`.
/// Equivalent to `crn_packed_uint<N>::operator unsigned int()`.
fn read_packed(input: &[u8], offset: usize, n: usize) -> Option<u32> {
    let end = offset.checked_add(n)?;
    if end > input.len() {
        return None;
    }
    let bytes = &input[offset..end];
    Some(match n {
        1 => u32::from(bytes[0]),
        2 => (u32::from(bytes[0]) << 8) | u32::from(bytes[1]),
        3 => (u32::from(bytes[0]) << 16) | (u32::from(bytes[1]) << 8) | u32::from(bytes[2]),
        _ => {
            (u32::from(bytes[0]) << 24)
                | (u32::from(bytes[1]) << 16)
                | (u32::from(bytes[2]) << 8)
                | u32::from(bytes[3])
        }
    })
}

/// A single `crn_palette` triple: `(ofs:3, size:3, num:2)` = 8 bytes.
struct PaletteDesc {
    ofs: u32,
    size: u32,
    num: u32,
}

/// Parsed CRN header fields. Field widths follow `struct crn_header`.
#[allow(dead_code)]
struct Header {
    header_size: u32,
    data_size: u32,
    width: u32,
    height: u32,
    levels: u32,
    faces: u32,
    format: u8,
    flags: u32,
    color_endpoints: PaletteDesc,
    color_selectors: PaletteDesc,
    alpha_endpoints: PaletteDesc,
    alpha_selectors: PaletteDesc,
    tables_size: u32,
    tables_ofs: u32,
    level_offsets: Vec<u32>,
}

impl Header {
    /// Parse and validate the header.
    fn parse(input: &[u8]) -> Result<Header, CrnDecodeError> {
        if input.len() < CRN_HEADER_MIN_SIZE {
            return Err(CrnDecodeError::InputTooShort);
        }

        let sig = read_packed(input, 0, 2).ok_or(CrnDecodeError::InputTooShort)?;
        if sig != CRN_SIG {
            return Err(CrnDecodeError::InvalidSignature);
        }

        let header_size = read_packed(input, 2, 2).ok_or(CrnDecodeError::InputTooShort)?;
        // m_header_crc16 at offset 4 (2 bytes) is not checked.
        let data_size = read_packed(input, 6, 4).ok_or(CrnDecodeError::InputTooShort)?;
        // m_data_crc16 at offset 10 (2 bytes) is not checked.
        let width = read_packed(input, 12, 2).ok_or(CrnDecodeError::InputTooShort)?;
        let height = read_packed(input, 14, 2).ok_or(CrnDecodeError::InputTooShort)?;
        // Some unchecked reference paths treat zero as a single level. Keep
        // that compatibility while still bounding the variable offset table.
        let levels = read_packed(input, 16, 1)
            .ok_or(CrnDecodeError::InputTooShort)?
            .max(1);
        let faces = read_packed(input, 17, 1).ok_or(CrnDecodeError::InputTooShort)?;
        let format = read_packed(input, 18, 1).ok_or(CrnDecodeError::InputTooShort)? as u8;
        let flags = read_packed(input, 19, 2).ok_or(CrnDecodeError::InputTooShort)?;
        // m_reserved (4), m_userdata0 (4), m_userdata1 (4) at offsets 21..33.

        let color_endpoints = read_palette(input, 33)?;
        let color_selectors = read_palette(input, 41)?;
        let alpha_endpoints = read_palette(input, 49)?;
        let alpha_selectors = read_palette(input, 57)?;

        let tables_size = read_packed(input, 65, 2).ok_or(CrnDecodeError::InputTooShort)?;
        let tables_ofs = read_packed(input, 67, 3).ok_or(CrnDecodeError::InputTooShort)?;

        if levels > CRN_MAX_LEVELS {
            return Err(CrnDecodeError::InvalidHeader);
        }
        // Keep the existing unchecked-reference compatibility for zero-sized
        // headers (the public result clamps them to one), but bound all
        // non-zero dimensions to Crunch's documented maximum.
        if width > CRN_MAX_LEVEL_RESOLUTION || height > CRN_MAX_LEVEL_RESOLUTION {
            return Err(CrnDecodeError::InvalidHeader);
        }
        if faces != 1 && faces != 6 {
            return Err(CrnDecodeError::InvalidHeader);
        }

        let offsets_size = (levels as usize)
            .checked_mul(4)
            .ok_or(CrnDecodeError::InvalidHeader)?;
        let required_header_size = LEVEL_OFS_OFFSET
            .checked_add(offsets_size)
            .ok_or(CrnDecodeError::InvalidHeader)?;

        // The declared header must cover the entire variable offset table, and
        // the declared file data must cover both the header and supplied data.
        if (header_size as usize) < required_header_size || header_size > data_size {
            return Err(CrnDecodeError::InvalidHeader);
        }
        if input.len() < required_header_size {
            return Err(CrnDecodeError::InputTooShort);
        }
        if input.len() < data_size as usize {
            return Err(CrnDecodeError::InvalidHeader);
        }

        let mut level_offsets = Vec::with_capacity(levels as usize);
        for level in 0..levels as usize {
            let offset = level
                .checked_mul(4)
                .and_then(|offset| LEVEL_OFS_OFFSET.checked_add(offset))
                .ok_or(CrnDecodeError::InvalidHeader)?;
            level_offsets.push(read_packed(input, offset, 4).ok_or(CrnDecodeError::InputTooShort)?);
        }

        // Mip streams are packed sequentially. Their order must remain valid
        // even in a segmented header; only the fact that they lie out-of-line
        // is classified as UnsupportedSegmented.
        if level_offsets.windows(2).any(|pair| pair[0] >= pair[1]) || level_offsets[0] < header_size
        {
            return Err(CrnDecodeError::InvalidHeader);
        }
        if level_offsets[level_offsets.len() - 1] >= data_size {
            return Err(if flags & CRN_HEADER_FLAG_SEGMENTED != 0 {
                CrnDecodeError::UnsupportedSegmented
            } else {
                CrnDecodeError::InvalidHeader
            });
        }

        Ok(Header {
            header_size,
            data_size,
            width,
            height,
            levels,
            faces,
            format,
            flags,
            color_endpoints,
            color_selectors,
            alpha_endpoints,
            alpha_selectors,
            tables_size,
            tables_ofs,
            level_offsets,
        })
    }
}

/// Reads a `crn_palette` (8 bytes: ofs:3, size:3, num:2) at `offset`.
fn read_palette(input: &[u8], offset: usize) -> Result<PaletteDesc, CrnDecodeError> {
    let ofs = read_packed(input, offset, 3).ok_or(CrnDecodeError::InputTooShort)?;
    let size = read_packed(input, offset + 3, 3).ok_or(CrnDecodeError::InputTooShort)?;
    let num = read_packed(input, offset + 6, 2).ok_or(CrnDecodeError::InputTooShort)?;
    Ok(PaletteDesc { ofs, size, num })
}

/// Returns the byte slice for a palette/table section, validating its bounds.
fn section(input: &[u8], ofs: u32, size: u32) -> Result<&[u8], CrnDecodeError> {
    let start = ofs as usize;
    let end = start
        .checked_add(size as usize)
        .ok_or(CrnDecodeError::InvalidHeader)?;
    input.get(start..end).ok_or(CrnDecodeError::InputTooShort)
}

// ---------------------------------------------------------------------------
// MSB-first bit reader
// ---------------------------------------------------------------------------

/// An MSB-first bit reader over a byte slice.
///
/// Direct port of `symbol_codec`'s `get_bits` / `decode_bits` / refill loop
/// in `crn_decomp.h`. Bits are consumed from the most-significant end of a
/// 32-bit accumulator; bytes past the end of the buffer read as zero (the
/// reference behaviour) so over-reads are silent at the bit level and are
/// caught instead as invalid symbols by the callers.
struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    bit_buf: u32,
    bit_count: i32,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        BitReader {
            data,
            pos: 0,
            bit_buf: 0,
            bit_count: 0,
        }
    }

    /// Fetch `num_bits` from the buffer, MSB-first.
    #[inline]
    fn get_bits(&mut self, num_bits: u32) -> u32 {
        let num_bits = num_bits as i32;
        while self.bit_count < num_bits {
            let c = if self.pos < self.data.len() {
                let b = self.data[self.pos];
                self.pos += 1;
                b
            } else {
                0
            };
            self.bit_count += 8;
            // Place the new byte at the top of the accumulator, after the
            // currently-valid bits. Matches `m_bit_buf |= (c << (32 - count))`.
            self.bit_buf |= (u32::from(c)) << (32 - self.bit_count);
        }

        let result = self.bit_buf >> (32 - num_bits as u32);
        self.bit_buf <<= num_bits as u32;
        self.bit_count -= num_bits;
        result
    }

    /// `decode_bits`: reads up to 32 bits, splitting reads > 16 bits so each
    /// half fits in the refill loop above.
    #[inline]
    fn decode_bits(&mut self, num_bits: u32) -> u32 {
        if num_bits == 0 {
            0
        } else if num_bits > 16 {
            let a = self.get_bits(num_bits - 16);
            let b = self.get_bits(16);
            (a << 16) | b
        } else {
            self.get_bits(num_bits)
        }
    }

    /// Refills the accumulator to at least 24 valid bits, mirroring the inline
    /// refill at the top of `symbol_codec::decode`. Used before a Huffman
    /// lookup so the fast path can read directly from `bit_buf`.
    #[inline]
    fn refill_for_decode(&mut self) {
        if self.bit_count >= 24 {
            return;
        }
        if self.bit_count < 16 {
            let c0 = self.next_byte();
            let c1 = self.next_byte();
            self.bit_count += 16;
            let c = (u32::from(c0) << 8) | u32::from(c1);
            self.bit_buf |= c << (32 - self.bit_count);
        } else {
            let c = self.next_byte();
            self.bit_count += 8;
            self.bit_buf |= u32::from(c) << (32 - self.bit_count);
        }
    }

    #[inline]
    fn next_byte(&mut self) -> u8 {
        if self.pos < self.data.len() {
            let b = self.data[self.pos];
            self.pos += 1;
            b
        } else {
            0
        }
    }
}

// ---------------------------------------------------------------------------
// Canonical Huffman decoder
// ---------------------------------------------------------------------------

/// A built canonical-Huffman decoder table.
///
/// Port of `prefix_coding::decoder_tables`. The fast path consults `lookup`;
/// the slow path walks `max_codes` / `val_ptrs` / `sorted_order`.
///
/// `code_sizes`, `min_code_size`, and `max_code_size` are retained for clarity
/// and potential future validation even though decoding does not read them.
#[allow(dead_code)]
struct HuffmanTable {
    code_sizes: Vec<u8>,
    lookup: Vec<u32>,
    sorted_order: Vec<u16>,
    // Indexed by `[0..=MAX_EXPECTED_CODE_SIZE]`; the last entry is a sentinel.
    max_codes: [u32; MAX_EXPECTED_CODE_SIZE + 1],
    val_ptrs: [i32; MAX_EXPECTED_CODE_SIZE + 1],
    table_bits: u32,
    table_max_code: u32,
    decode_start_code_size: u32,
    min_code_size: u32,
    max_code_size: u32,
}

impl HuffmanTable {
    /// Build a decoder table from per-symbol code sizes. Port of
    /// `decoder_tables::init`.
    fn build(code_sizes: Vec<u8>) -> Result<HuffmanTable, CrnDecodeError> {
        let num_syms = code_sizes.len();
        if num_syms == 0 {
            return Err(CrnDecodeError::InvalidStream);
        }

        // Choose the fast-lookup table size. Port of
        // `compute_decoder_table_bits`: 0 for tiny alphabets, else
        // min(1 + ceil_log2i(syms), MAX_TABLE_BITS). Note ceil_log2i of a power
        // of two is the exponent itself (e.g. ceil_log2i(32) = 5, not 6).
        let table_bits = if num_syms > 16 {
            let log2 = ceil_log2i(num_syms as u32);
            core::cmp::min(log2 + 1, MAX_TABLE_BITS)
        } else {
            0
        };

        // Count codes per size.
        let mut num_codes = [0u32; MAX_EXPECTED_CODE_SIZE + 1];
        for &c in &code_sizes {
            if c != 0 {
                if c as usize > MAX_EXPECTED_CODE_SIZE {
                    return Err(CrnDecodeError::InvalidStream);
                }
                num_codes[c as usize] += 1;
            }
        }

        // Reject oversubscribed canonical code sets before deriving lookup
        // indices. Incomplete sets (including a single one-bit symbol) are
        // valid in Crunch, but the Kraft sum must never exceed one.
        let mut remaining = 1i32;
        for &count in num_codes.iter().skip(1) {
            remaining = (remaining << 1) - count as i32;
            if remaining < 0 {
                return Err(CrnDecodeError::InvalidStream);
            }
        }

        let mut min_codes = [0u32; MAX_EXPECTED_CODE_SIZE];
        let mut max_codes = [0u32; MAX_EXPECTED_CODE_SIZE + 1];
        let mut val_ptrs = [0i32; MAX_EXPECTED_CODE_SIZE + 1];
        let mut sorted_positions = [0u32; MAX_EXPECTED_CODE_SIZE + 1];

        let mut cur_code = 0u32;
        let mut total_used_syms = 0u32;
        let mut min_code_size = u32::MAX;
        let mut max_code_size = 0u32;

        for i in 1..=MAX_EXPECTED_CODE_SIZE {
            let n = num_codes[i];
            if n == 0 {
                max_codes[i - 1] = 0;
            } else {
                min_code_size = core::cmp::min(min_code_size, i as u32);
                max_code_size = core::cmp::max(max_code_size, i as u32);

                min_codes[i - 1] = cur_code;
                // Stored shifted form: `1 + ((max << (16-i)) | ((1<<(16-i)) - 1))`.
                let raw_max = cur_code + n - 1;
                let shifted = (raw_max << (16 - i)) | ((1u32 << (16 - i)) - 1);
                max_codes[i - 1] = 1 + shifted;

                val_ptrs[i - 1] = total_used_syms as i32;
                sorted_positions[i] = total_used_syms;

                cur_code += n;
                total_used_syms += n;
            }
            cur_code <<= 1;
        }

        if total_used_syms == 0 {
            return Err(CrnDecodeError::InvalidStream);
        }

        // Build the symbol order sorted by (code_size, symbol).
        let mut sorted_order = vec![0u16; total_used_syms as usize];
        for (sym, &c) in code_sizes.iter().enumerate() {
            if c != 0 {
                let pos = sorted_positions[c as usize];
                sorted_positions[c as usize] += 1;
                sorted_order[pos as usize] = sym as u16;
            }
        }

        // Adjust table_bits down if it's below the smallest code size.
        let table_bits = if table_bits <= min_code_size {
            0
        } else {
            table_bits
        };

        // Build the fast lookup table.
        let lookup = if table_bits > 0 {
            let table_size = 1usize << table_bits;
            let mut lookup = vec![u32::MAX; table_size];
            for codesize in 1..=table_bits as usize {
                if num_codes[codesize] == 0 {
                    continue;
                }
                let fillsize = table_bits as usize - codesize;
                let fillnum = 1usize << fillsize;
                let min_code = min_codes[codesize - 1];
                // Unshift the stored max code to recover the raw upper bound.
                let max_code = Self::unshifted_max_code(max_codes[codesize - 1], codesize as u32);
                let val_ptr = val_ptrs[codesize - 1] as usize;

                for code in min_code..=max_code {
                    let sym_index = sorted_order[val_ptr + (code - min_code) as usize];
                    for j in 0..fillnum {
                        let t = j + ((code as usize) << fillsize);
                        lookup[t] = sym_index as u32 | ((codesize as u32) << 16);
                    }
                }
            }
            lookup
        } else {
            Vec::new()
        };

        // Finalize val_ptrs: subtract the min_code of each length so they
        // become offsets relative to the first code of that length.
        for i in 0..MAX_EXPECTED_CODE_SIZE {
            val_ptrs[i] -= min_codes[i] as i32;
        }

        // Determine the fast-path max code and the slow-path start length.
        let mut table_max_code = 0u32;
        let mut decode_start_code_size = min_code_size;
        if table_bits > 0 {
            let mut i = table_bits as usize;
            while i >= 1 {
                if num_codes[i] != 0 {
                    table_max_code = max_codes[i - 1];
                    break;
                }
                i -= 1;
            }
            if i >= 1 {
                decode_start_code_size = table_bits + 1;
                for (j, &count) in num_codes
                    .iter()
                    .enumerate()
                    .take(max_code_size as usize + 1)
                    .skip(table_bits as usize + 1)
                {
                    if count != 0 {
                        decode_start_code_size = j as u32;
                        break;
                    }
                }
            }
        }

        // Sentinels at the end, matching the C++ table layout.
        max_codes[MAX_EXPECTED_CODE_SIZE] = u32::MAX;
        val_ptrs[MAX_EXPECTED_CODE_SIZE] = 0xFFFFF;

        Ok(HuffmanTable {
            code_sizes,
            lookup,
            sorted_order,
            max_codes,
            val_ptrs,
            table_bits,
            table_max_code,
            decode_start_code_size,
            min_code_size,
            max_code_size,
        })
    }

    /// Inverts the shifted `max_codes` storage to get back the raw maximum
    /// canonical code of a given length. The stored value is
    /// `1 + ((raw << (16-i)) | mask)`, so the raw value is
    /// `(stored - 1) >> (16 - i)`.
    #[inline]
    fn unshifted_max_code(stored: u32, i: u32) -> u32 {
        (stored - 1) >> (16 - i)
    }

    /// Decode one symbol. Port of `symbol_codec::decode`.
    ///
    /// A fast-path lookup miss in the precomputed table is a hard
    /// [`InvalidStream`]. The slow path, however, mirrors the reference
    /// decompressor's recovery: if the resolved symbol index falls outside the
    /// sorted table, the reference prints `CRND_ASSERT(0)` and `return 0` —
    /// recovering as symbol 0 without consuming any bits. Degenerate
    /// single-symbol delta models (e.g. a constant alpha channel on a DXT5
    /// texture, whose alpha-endpoint palette has a single entry) hit this path
    /// on real crunch-encoded files, so the recovery must be reproduced
    /// verbatim, including the zero-bit consumption, or the colour stream's bit
    /// alignment diverges.
    #[inline]
    fn decode(&self, reader: &mut BitReader) -> Result<u32, CrnDecodeError> {
        reader.refill_for_decode();

        let k = (reader.bit_buf >> 16) + 1;

        let (sym, len) = if k <= self.table_max_code {
            // Fast path: one lookup in the precomputed table.
            let idx = (reader.bit_buf >> (32 - self.table_bits)) as usize;
            let t = self.lookup[idx];
            if t == u32::MAX {
                return Err(CrnDecodeError::InvalidStream);
            }
            (t & 0xFFFF, t >> 16)
        } else {
            // Slow path: find the code length whose max code covers `k`. The
            // loop walks `max_codes[len-1]` upward. The sentinel at
            // `max_codes[MAX_EXPECTED_CODE_SIZE] == u32::MAX` (set in `build`)
            // guarantees termination at `len == MAX_EXPECTED_CODE_SIZE + 1`,
            // matching the unbounded `for(;;)` in `crn_decomp.h`.
            let mut len = self.decode_start_code_size;
            loop {
                if k <= self.max_codes[(len - 1) as usize] {
                    break;
                }
                len += 1;
            }
            let val_ptr =
                self.val_ptrs[(len - 1) as usize] + ((reader.bit_buf >> (32 - len)) as i32);
            if val_ptr < 0 || (val_ptr as usize) >= self.sorted_order.len() {
                // The slow-path index fell outside the symbol table. The
                // reference decompressor prints `CRND_ASSERT(0)` here and then
                // `return 0` — recovering as symbol 0 without consuming any
                // bits. Files produced by the crunch encoder legitimately hit
                // this path for degenerate single-symbol delta models (e.g. a
                // DXT5 texture whose alpha channel is constant, so the
                // alpha-endpoint palette has a single entry). We mirror that
                // recovery exactly, including the zero-bit consumption, or the
                // downstream bit alignment of the colour stream would diverge.
                return Ok(0);
            }
            (u32::from(self.sorted_order[val_ptr as usize]), len)
        };

        reader.bit_buf <<= len;
        reader.bit_count -= len as i32;
        Ok(sym)
    }
}

/// Reads a static-Huffman codebook from the stream and builds its decoder
/// table. Port of `decode_receive_static_data_model`.
fn receive_static_data_model(reader: &mut BitReader) -> Result<HuffmanTable, CrnDecodeError> {
    // `total_bits(cMaxSupportedSyms)` = total_bits(8192). The C++ helper counts
    // how many right-shifts until 0, which is 14 for 8192 (= 1 + ceil(log2)),
    // not 13 — so the symbol count occupies 14 bits, not the obvious 13.
    let total_used_syms = reader.decode_bits(total_bits(MAX_SUPPORTED_SYMS));
    if total_used_syms == 0 {
        // Empty model: build a degenerate table with no symbols. Callers only
        // hit this when the section genuinely has no data, which we reject.
        return Err(CrnDecodeError::InvalidStream);
    }
    // The reference only asserts that this is <= 8192. Its assertion handler
    // is non-fatal, so all values representable by the 14-bit field (up to
    // 16383) continue through table construction and can decode successfully.

    let mut code_sizes = vec![0u8; total_used_syms as usize];

    let num_cl_codes = reader.decode_bits(5);
    if !(1..=21).contains(&num_cl_codes) {
        return Err(CrnDecodeError::InvalidStream);
    }

    // Meta-Huffman: 3-bit code lengths for the code-length alphabet.
    let mut meta_sizes = [0u8; 21];
    for i in 0..num_cl_codes as usize {
        meta_sizes[MOST_PROBABLE_CODELENGTH_CODES[i] as usize] = reader.decode_bits(3) as u8;
    }
    let meta = HuffmanTable::build(meta_sizes.to_vec())?;

    let mut ofs = 0u32;
    while ofs < total_used_syms {
        let num_remaining = total_used_syms - ofs;
        let code = meta.decode(reader)?;
        if code <= 16 {
            code_sizes[ofs as usize] = code as u8;
            ofs += 1;
        } else if code == SMALL_ZERO_RUN_CODE {
            let len = reader.decode_bits(SMALL_ZERO_RUN_EXTRA_BITS) + MIN_SMALL_ZERO_RUN_SIZE;
            if len > num_remaining {
                return Err(CrnDecodeError::InvalidStream);
            }
            ofs += len;
        } else if code == LARGE_ZERO_RUN_CODE {
            let len = reader.decode_bits(LARGE_ZERO_RUN_EXTRA_BITS) + MIN_LARGE_ZERO_RUN_SIZE;
            if len > num_remaining {
                return Err(CrnDecodeError::InvalidStream);
            }
            ofs += len;
        } else if code == SMALL_REPEAT_CODE || code == LARGE_REPEAT_CODE {
            let len = if code == SMALL_REPEAT_CODE {
                reader.decode_bits(SMALL_NONZERO_RUN_EXTRA_BITS) + SMALL_MIN_NONZERO_RUN_SIZE
            } else {
                reader.decode_bits(LARGE_NONZERO_RUN_EXTRA_BITS) + LARGE_MIN_NONZERO_RUN_SIZE
            };
            if ofs == 0 || len > num_remaining {
                return Err(CrnDecodeError::InvalidStream);
            }
            let prev = code_sizes[(ofs - 1) as usize];
            if prev == 0 {
                return Err(CrnDecodeError::InvalidStream);
            }
            let end = ofs + len;
            while ofs < end {
                code_sizes[ofs as usize] = prev;
                ofs += 1;
            }
        } else {
            return Err(CrnDecodeError::InvalidStream);
        }
    }

    if ofs != total_used_syms {
        return Err(CrnDecodeError::InvalidStream);
    }
    HuffmanTable::build(code_sizes)
}

// ---------------------------------------------------------------------------
// Palettes
// ---------------------------------------------------------------------------

/// All decoded palette data needed to assemble BC blocks.
struct Palettes {
    color_endpoints: Vec<u32>,
    color_selectors: Vec<u32>,
    alpha_endpoints: Vec<u16>,
    /// `3 * num_alpha_selectors` entries: three packed u16 per selector entry.
    alpha_selectors: Vec<u16>,
}

impl Palettes {
    fn decode(
        header: &Header,
        input: &[u8],
        format: CrnFormat,
    ) -> Result<Palettes, CrnDecodeError> {
        let mut p = Palettes {
            color_endpoints: Vec::new(),
            color_selectors: Vec::new(),
            alpha_endpoints: Vec::new(),
            alpha_selectors: Vec::new(),
        };

        if header.color_endpoints.num == 0 || header.color_selectors.num == 0 {
            return Err(CrnDecodeError::InvalidHeader);
        }
        p.color_endpoints = decode_color_endpoints(
            section(
                input,
                header.color_endpoints.ofs,
                header.color_endpoints.size,
            )?,
            header.color_endpoints.num,
        )?;
        p.color_selectors = decode_color_selectors(
            section(
                input,
                header.color_selectors.ofs,
                header.color_selectors.size,
            )?,
            header.color_selectors.num,
        )?;

        if matches!(format, CrnFormat::Bc3) {
            if header.alpha_endpoints.num == 0 || header.alpha_selectors.num == 0 {
                return Err(CrnDecodeError::InvalidHeader);
            }
            p.alpha_endpoints = decode_alpha_endpoints(
                section(
                    input,
                    header.alpha_endpoints.ofs,
                    header.alpha_endpoints.size,
                )?,
                header.alpha_endpoints.num,
            )?;
            p.alpha_selectors = decode_alpha_selectors(
                section(
                    input,
                    header.alpha_selectors.ofs,
                    header.alpha_selectors.size,
                )?,
                header.alpha_selectors.num,
            )?;
        }

        Ok(p)
    }
}

/// Decode the color-endpoint palette. Each entry packs two 565 endpoint colors
/// into one u32 (the first 4 bytes of a BC1 block, little-endian).
fn decode_color_endpoints(data: &[u8], count: u32) -> Result<Vec<u32>, CrnDecodeError> {
    let mut reader = BitReader::new(data);
    let dm0 = receive_static_data_model(&mut reader)?;
    let dm1 = receive_static_data_model(&mut reader)?;

    let mut out = Vec::with_capacity(count as usize);
    let (mut a, mut b, mut c, mut d, mut e, mut f) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    for _ in 0..count {
        a = (a + dm0.decode(&mut reader)?) & 31;
        b = (b + dm1.decode(&mut reader)?) & 63;
        c = (c + dm0.decode(&mut reader)?) & 31;
        d = (d + dm0.decode(&mut reader)?) & 31;
        e = (e + dm1.decode(&mut reader)?) & 63;
        f = (f + dm0.decode(&mut reader)?) & 31;
        // Little-endian packing: low 16 bits = low color 565, high 16 = high.
        out.push(c | (b << 5) | (a << 11) | (f << 16) | (e << 21) | (d << 27));
    }
    Ok(out)
}

/// Decode the color-selector palette. Each entry is the 4 selector bytes of a
/// BC1 block (16 x 2-bit selectors) as a little-endian u32.
fn decode_color_selectors(data: &[u8], count: u32) -> Result<Vec<u32>, CrnDecodeError> {
    let mut reader = BitReader::new(data);
    let dm = receive_static_data_model(&mut reader)?;

    // Paired-delta table over [-3, +3] x [-3, +3] = 49 symbols.
    let (delta0, delta1) = build_delta_tables(3);

    let mut cur = [0u32; 16];
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        for j in 0..8 {
            let sym = dm.decode(&mut reader)? as usize;
            let (d0, d1) = paired_delta(&delta0, &delta1, sym)?;
            // C++ does `(int32_delta + uint32_cur) & 3`; emulate the implicit
            // signed-to-unsigned conversion with wrapping arithmetic.
            cur[j * 2] = (d0 as u32).wrapping_add(cur[j * 2]) & 3;
            cur[j * 2 + 1] = (d1 as u32).wrapping_add(cur[j * 2 + 1]) & 3;
        }
        let fl = DXT1_FROM_LINEAR;
        let packed = (fl[cur[0] as usize])
            | (fl[cur[1] as usize] << 2)
            | (fl[cur[2] as usize] << 4)
            | (fl[cur[3] as usize] << 6)
            | (fl[cur[4] as usize] << 8)
            | (fl[cur[5] as usize] << 10)
            | (fl[cur[6] as usize] << 12)
            | (fl[cur[7] as usize] << 14)
            | (fl[cur[8] as usize] << 16)
            | (fl[cur[9] as usize] << 18)
            | (fl[cur[10] as usize] << 20)
            | (fl[cur[11] as usize] << 22)
            | (fl[cur[12] as usize] << 24)
            | (fl[cur[13] as usize] << 26)
            | (fl[cur[14] as usize] << 28)
            | (fl[cur[15] as usize] << 30);
        out.push(packed);
    }
    Ok(out)
}

/// Decode the alpha-endpoint palette. Each entry is two 8-bit alphas packed as
/// `a | (b << 8)` (the first 2 bytes of a BC3 alpha block).
fn decode_alpha_endpoints(data: &[u8], count: u32) -> Result<Vec<u16>, CrnDecodeError> {
    let mut reader = BitReader::new(data);
    let dm = receive_static_data_model(&mut reader)?;

    let mut out = Vec::with_capacity(count as usize);
    let (mut a, mut b) = (0u32, 0u32);
    for _ in 0..count {
        let sa = dm.decode(&mut reader)?;
        let sb = dm.decode(&mut reader)?;
        a = (sa + a) & 255;
        b = (sb + b) & 255;
        out.push((a | (b << 8)) as u16);
    }
    Ok(out)
}

/// Decode the alpha-selector palette. Each entry produces three packed u16
/// (the 6 selector bytes of a BC3 alpha block, 16 x 3-bit). `count` here is
/// the number of selector entries; the returned Vec has `3 * count` elements.
fn decode_alpha_selectors(data: &[u8], count: u32) -> Result<Vec<u16>, CrnDecodeError> {
    let mut reader = BitReader::new(data);
    let dm = receive_static_data_model(&mut reader)?;

    let (delta0, delta1) = build_delta_tables(7);

    let mut cur = [0u32; 16];
    let mut out = Vec::with_capacity((count as usize) * 3);
    for _ in 0..count {
        for j in 0..8 {
            let sym = dm.decode(&mut reader)? as usize;
            let (d0, d1) = paired_delta(&delta0, &delta1, sym)?;
            cur[j * 2] = (d0 as u32).wrapping_add(cur[j * 2]) & 7;
            cur[j * 2 + 1] = (d1 as u32).wrapping_add(cur[j * 2 + 1]) & 7;
        }
        let fl = DXT5_FROM_LINEAR;
        // Bit-carry packing exactly as in `crn_decomp.h`.
        out.push(
            (fl[cur[0] as usize]
                | (fl[cur[1] as usize] << 3)
                | (fl[cur[2] as usize] << 6)
                | (fl[cur[3] as usize] << 9)
                | (fl[cur[4] as usize] << 12)
                | (fl[cur[5] as usize] << 15)) as u16,
        );
        out.push(
            ((fl[cur[5] as usize] >> 1)
                | (fl[cur[6] as usize] << 2)
                | (fl[cur[7] as usize] << 5)
                | (fl[cur[8] as usize] << 8)
                | (fl[cur[9] as usize] << 11)
                | (fl[cur[10] as usize] << 14)) as u16,
        );
        out.push(
            ((fl[cur[10] as usize] >> 2)
                | (fl[cur[11] as usize] << 1)
                | (fl[cur[12] as usize] << 4)
                | (fl[cur[13] as usize] << 7)
                | (fl[cur[14] as usize] << 10)
                | (fl[cur[15] as usize] << 13)) as u16,
        );
    }
    Ok(out)
}

/// Builds the paired-delta lookup tables `(delta0, delta1)` over the square
/// `[-max, +max] x [-max, +max]`, matching the C++ loop in
/// `decode_color_selectors` / `decode_alpha_selectors`.
fn build_delta_tables(max: i32) -> (Vec<i32>, Vec<i32>) {
    let side = (max * 2 + 1) as usize;
    let total = side * side;
    let mut delta0 = vec![0i32; total];
    let mut delta1 = vec![0i32; total];
    let (mut l, mut m) = (-max, -max);
    for i in 0..total {
        delta0[i] = l;
        delta1[i] = m;
        l += 1;
        if l > max {
            l = -max;
            m += 1;
        }
    }
    (delta0, delta1)
}

/// Safely resolves one symbol in the paired selector-delta tables.
#[inline]
fn paired_delta(
    delta0: &[i32],
    delta1: &[i32],
    symbol: usize,
) -> Result<(i32, i32), CrnDecodeError> {
    let d0 = delta0
        .get(symbol)
        .copied()
        .ok_or(CrnDecodeError::InvalidStream)?;
    let d1 = delta1
        .get(symbol)
        .copied()
        .ok_or(CrnDecodeError::InvalidStream)?;
    Ok((d0, d1))
}

// ---------------------------------------------------------------------------
// Tables section
// ---------------------------------------------------------------------------

/// The static Huffman codebooks shared across every chunk of a level.
struct Tables {
    chunk_encoding: HuffmanTable,
    endpoint_delta: [HuffmanTable; 2], // 0 = color, 1 = alpha
    selector_delta: [HuffmanTable; 2], // 0 = color, 1 = alpha
}

impl Tables {
    fn decode(header: &Header, input: &[u8], format: CrnFormat) -> Result<Tables, CrnDecodeError> {
        let data = section(input, header.tables_ofs, header.tables_size)?;
        let mut reader = BitReader::new(data);

        let chunk_encoding = receive_static_data_model(&mut reader)?;
        // Color endpoint/selector delta books always follow.
        let color_endpoint = receive_static_data_model(&mut reader)?;
        let color_selector = receive_static_data_model(&mut reader)?;

        let (alpha_endpoint, alpha_selector) = if matches!(format, CrnFormat::Bc3) {
            (
                receive_static_data_model(&mut reader)?,
                receive_static_data_model(&mut reader)?,
            )
        } else {
            // Placeholder books for the unused alpha slot. A degenerate single
            // symbol avoids touching the stream.
            (HuffmanTable::build(vec![1])?, HuffmanTable::build(vec![1])?)
        };

        Ok(Tables {
            chunk_encoding,
            endpoint_delta: [color_endpoint, alpha_endpoint],
            selector_delta: [color_selector, alpha_selector],
        })
    }
}

// ---------------------------------------------------------------------------
// Block assembly
// ---------------------------------------------------------------------------

/// Branchless single-subtract wrap used to keep palette indices in range.
/// The encoder guarantees the running index stays within `[0, 2n)`, so one
/// conditional subtract of `n` restores it to `[0, n)`. Port of `limit`.
#[inline]
fn limit(x: &mut u32, n: u32) {
    let v = (*x).wrapping_sub(n) as i32;
    let msk = v >> 31;
    *x = (*x & msk as u32) | (v as u32 & !(msk as u32));
}

/// Applies one delta to a palette predictor and validates the reference
/// codec's single-subtract wrap before the result is used as a Vec index.
#[inline]
fn advance_palette_index(
    index: &mut u32,
    delta: u32,
    palette_len: u32,
) -> Result<usize, CrnDecodeError> {
    *index = index.wrapping_add(delta);
    limit(index, palette_len);
    if *index >= palette_len {
        return Err(CrnDecodeError::InvalidStream);
    }
    Ok(*index as usize)
}

/// Decodes one mip's continuous face stream into raw BC1 blocks.
fn unpack_bc1(
    data: &[u8],
    blocks_x: u32,
    blocks_y: u32,
    tables: &Tables,
    palettes: &Palettes,
    outputs: &mut [Vec<u8>],
) -> Result<(), CrnDecodeError> {
    let chunks_x = (blocks_x + 1) >> 1;
    let chunks_y = (blocks_y + 1) >> 1;
    let block_bytes: usize = 8;

    let num_color_endpoints = palettes.color_endpoints.len() as u32;
    let num_color_selectors = palettes.color_selectors.len() as u32;

    let mut reader = BitReader::new(data);
    let mut chunk_encoding_bits: u32 = 1;
    let mut prev_endpoint: u32 = 0;
    let mut prev_selector: u32 = 0;

    // The codec and all predictors deliberately remain outside the face loop,
    // matching crn_unpacker::unpack_dxt1.
    for output in outputs {
        for y in 0..chunks_y {
            let (mut x, end_x, dir_x) = if y & 1 == 1 {
                (chunks_x as i32 - 1, -1, -1)
            } else {
                (0, chunks_x as i32, 1)
            };
            // Convert the leftmost block column of this chunk row into a byte offset.
            let mut block_col = if y & 1 == 1 {
                chunks_x.saturating_sub(1) as usize * block_bytes * 2
            } else {
                0
            };
            let block_delta = if y & 1 == 1 {
                -(block_bytes as isize * 2)
            } else {
                block_bytes as isize * 2
            };
            let skip_bottom_row = (y == chunks_y - 1) && (blocks_y & 1 == 1);

            while x != end_x {
                if chunk_encoding_bits == 1 {
                    chunk_encoding_bits = tables.chunk_encoding.decode(&mut reader)?;
                    chunk_encoding_bits |= 512;
                }
                let encoding_index = (chunk_encoding_bits & 7) as usize;
                chunk_encoding_bits >>= 3;

                let num_tiles = CHUNK_ENCODING_NUM_TILES[encoding_index];
                let tiles = CHUNK_ENCODING_TILES[encoding_index];

                let mut color_endpoints = [0u32; 4];
                for slot in color_endpoints.iter_mut().take(num_tiles as usize) {
                    let delta = tables.endpoint_delta[0].decode(&mut reader)?;
                    let endpoint_index =
                        advance_palette_index(&mut prev_endpoint, delta, num_color_endpoints)?;
                    *slot = palettes.color_endpoints[endpoint_index];
                }

                let skip_right_col = (blocks_x & 1 == 1) && (x == chunks_x as i32 - 1);

                let block_base = y as usize * 2 * blocks_x as usize * block_bytes + block_col;

                if !skip_bottom_row && !skip_right_col {
                    // Fast path: the whole 2x2 chunk is in-bounds. Write four
                    // contiguous blocks (two per row).
                    for by in 0..2usize {
                        for bx in 0..2usize {
                            let tile = tiles[bx + by * 2] as usize;
                            let di = (bx + by * blocks_x as usize) * block_bytes;
                            let off = block_base + di;
                            let endpoints = color_endpoints[tile].to_le_bytes();
                            let delta_s = tables.selector_delta[0].decode(&mut reader)?;
                            let selector_index = advance_palette_index(
                                &mut prev_selector,
                                delta_s,
                                num_color_selectors,
                            )?;
                            let selectors = palettes.color_selectors[selector_index].to_le_bytes();
                            output[off..off + 4].copy_from_slice(&endpoints);
                            output[off + 4..off + 8].copy_from_slice(&selectors);
                        }
                    }
                } else {
                    // Edge path: decode and consume every block's selector delta,
                    // but skip writing blocks that fall outside the image.
                    for by in 0..2usize {
                        for bx in 0..2usize {
                            let delta_s = tables.selector_delta[0].decode(&mut reader)?;
                            let selector_index = advance_palette_index(
                                &mut prev_selector,
                                delta_s,
                                num_color_selectors,
                            )?;

                            if (bx == 1 && skip_right_col) || (by == 1 && skip_bottom_row) {
                                continue;
                            }
                            let tile = tiles[bx + by * 2] as usize;
                            let di = (bx + by * blocks_x as usize) * block_bytes;
                            let off = block_base + di;
                            let endpoints = color_endpoints[tile].to_le_bytes();
                            let selectors = palettes.color_selectors[selector_index].to_le_bytes();
                            output[off..off + 4].copy_from_slice(&endpoints);
                            output[off + 4..off + 8].copy_from_slice(&selectors);
                        }
                    }
                }

                x += dir_x;
                block_col = (block_col as isize + block_delta) as usize;
            }
        }
    }

    Ok(())
}

/// Decodes one mip's continuous face stream into raw BC3 blocks.
fn unpack_bc3(
    data: &[u8],
    blocks_x: u32,
    blocks_y: u32,
    tables: &Tables,
    palettes: &Palettes,
    outputs: &mut [Vec<u8>],
) -> Result<(), CrnDecodeError> {
    let chunks_x = (blocks_x + 1) >> 1;
    let chunks_y = (blocks_y + 1) >> 1;
    let block_bytes: usize = 16;

    let num_color_endpoints = palettes.color_endpoints.len() as u32;
    let num_color_selectors = palettes.color_selectors.len() as u32;
    let num_alpha_endpoints = palettes.alpha_endpoints.len() as u32;
    let num_alpha_selectors = palettes.alpha_selectors.len() as u32 / 3;

    let mut reader = BitReader::new(data);
    let mut chunk_encoding_bits: u32 = 1;
    let mut prev_color_endpoint: u32 = 0;
    let mut prev_color_selector: u32 = 0;
    let mut prev_alpha_endpoint: u32 = 0;
    let mut prev_alpha_selector: u32 = 0;

    // The codec and all predictors deliberately remain outside the face loop,
    // matching crn_unpacker::unpack_dxt5.
    for output in outputs {
        for y in 0..chunks_y {
            let (mut x, end_x, dir_x) = if y & 1 == 1 {
                (chunks_x as i32 - 1, -1, -1)
            } else {
                (0, chunks_x as i32, 1)
            };
            let mut block_col = if y & 1 == 1 {
                chunks_x.saturating_sub(1) as usize * block_bytes * 2
            } else {
                0
            };
            let block_delta = if y & 1 == 1 {
                -(block_bytes as isize * 2)
            } else {
                block_bytes as isize * 2
            };
            let skip_bottom_row = (y == chunks_y - 1) && (blocks_y & 1 == 1);

            while x != end_x {
                if chunk_encoding_bits == 1 {
                    chunk_encoding_bits = tables.chunk_encoding.decode(&mut reader)?;
                    chunk_encoding_bits |= 512;
                }
                let encoding_index = (chunk_encoding_bits & 7) as usize;
                chunk_encoding_bits >>= 3;

                let num_tiles = CHUNK_ENCODING_NUM_TILES[encoding_index];
                let tiles = CHUNK_ENCODING_TILES[encoding_index];

                // Alpha endpoints, then color endpoints, per tile.
                let mut alpha_endpoints = [0u16; 4];
                for slot in alpha_endpoints.iter_mut().take(num_tiles as usize) {
                    let delta = tables.endpoint_delta[1].decode(&mut reader)?;
                    let endpoint_index = advance_palette_index(
                        &mut prev_alpha_endpoint,
                        delta,
                        num_alpha_endpoints,
                    )?;
                    *slot = palettes.alpha_endpoints[endpoint_index];
                }
                let mut color_endpoints = [0u32; 4];
                for slot in color_endpoints.iter_mut().take(num_tiles as usize) {
                    let delta = tables.endpoint_delta[0].decode(&mut reader)?;
                    let endpoint_index = advance_palette_index(
                        &mut prev_color_endpoint,
                        delta,
                        num_color_endpoints,
                    )?;
                    *slot = palettes.color_endpoints[endpoint_index];
                }

                let skip_right_col = (blocks_x & 1 == 1) && (x == chunks_x as i32 - 1);
                let block_base = y as usize * 2 * blocks_x as usize * block_bytes + block_col;

                for by in 0..2usize {
                    for bx in 0..2usize {
                        // Alpha selector delta, then color selector delta, per block.
                        let delta_a = tables.selector_delta[1].decode(&mut reader)?;
                        let alpha_selector_index = advance_palette_index(
                            &mut prev_alpha_selector,
                            delta_a,
                            num_alpha_selectors,
                        )?;

                        let delta_c = tables.selector_delta[0].decode(&mut reader)?;
                        let color_selector_index = advance_palette_index(
                            &mut prev_color_selector,
                            delta_c,
                            num_color_selectors,
                        )?;

                        if (bx == 1 && skip_right_col) || (by == 1 && skip_bottom_row) {
                            continue;
                        }

                        let tile = tiles[bx + by * 2] as usize;
                        let di = (bx + by * blocks_x as usize) * block_bytes;
                        let off = block_base + di;

                        let sel_base = alpha_selector_index * 3;
                        let a0 = u32::from(palettes.alpha_selectors[sel_base]);
                        let a1 = u32::from(palettes.alpha_selectors[sel_base + 1]);
                        let a2 = u32::from(palettes.alpha_selectors[sel_base + 2]);

                        // pD[0] = alpha_endpoints | (sel0 << 16)
                        let dw0 = u32::from(alpha_endpoints[tile]) | (a0 << 16);
                        // pD[1] = sel1 | (sel2 << 16)
                        let dw1 = a1 | (a2 << 16);
                        // pD[2] = color_endpoints
                        let dw2 = color_endpoints[tile];
                        // pD[3] = color_selectors
                        let dw3 = palettes.color_selectors[color_selector_index];

                        output[off..off + 4].copy_from_slice(&dw0.to_le_bytes());
                        output[off + 4..off + 8].copy_from_slice(&dw1.to_le_bytes());
                        output[off + 8..off + 12].copy_from_slice(&dw2.to_le_bytes());
                        output[off + 12..off + 16].copy_from_slice(&dw3.to_le_bytes());
                    }
                }

                x += dir_x;
                block_col = (block_col as isize + block_delta) as usize;
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Decompress every face and mip of a Crunch (CRN v1.04) file into raw BC1 or
/// BC3 blocks.
///
/// The CRN format is read from the header: DXT1 (and only DXT1) yields BC1
/// output; DXT5 and its swizzled variants yield BC3 output. All other formats
/// return [`CrnDecodeError::UnsupportedFormat`]. A 2D texture has one face; a
/// cubemap has six faces in +X, -X, +Y, -Y, +Z, -Z order.
///
/// Each returned [`CrnMipLevel`] owns its standard physical BC1/BC3 block
/// bytes, laid out row-major in block space.
pub fn decode(input: &[u8]) -> Result<CrnTexture, CrnDecodeError> {
    let header = Header::parse(input)?;

    let format = match header.format {
        FMT_DXT1 => CrnFormat::Bc1,
        FMT_DXT5 | FMT_DXT5_CCXY | FMT_DXT5_XGXR | FMT_DXT5_XGBR | FMT_DXT5_AGBR => CrnFormat::Bc3,
        _ => return Err(CrnDecodeError::UnsupportedFormat),
    };

    let block_bytes = if matches!(format, CrnFormat::Bc1) {
        8usize
    } else {
        16usize
    };
    let tables = Tables::decode(&header, input, format)?;
    let palettes = Palettes::decode(&header, input, format)?;

    let mut faces = Vec::with_capacity(header.faces as usize);
    for _ in 0..header.faces {
        faces.push(CrnFace {
            levels: Vec::with_capacity(header.levels as usize),
        });
    }

    // CRN groups its bitstreams by mip, with all faces concatenated inside a
    // level. Decode in that order, then move each buffer into the face-first
    // public hierarchy.
    for level_index in 0..header.levels as usize {
        let width = core::cmp::max(header.width >> level_index, 1);
        let height = core::cmp::max(header.height >> level_index, 1);
        let blocks_x = width.div_ceil(4);
        let blocks_y = height.div_ceil(4);
        let total = (blocks_x as usize)
            .checked_mul(blocks_y as usize)
            .and_then(|n| n.checked_mul(block_bytes))
            .ok_or(CrnDecodeError::SizeOverflow)?;

        let level_start = header.level_offsets[level_index] as usize;
        let level_end = header
            .level_offsets
            .get(level_index + 1)
            .copied()
            .unwrap_or(header.data_size) as usize;
        let level_data = input
            .get(level_start..level_end)
            .ok_or(CrnDecodeError::InvalidHeader)?;

        let mut outputs = Vec::with_capacity(header.faces as usize);
        for _ in 0..header.faces {
            outputs.push(vec![0u8; total]);
        }
        match format {
            CrnFormat::Bc1 => unpack_bc1(
                level_data,
                blocks_x,
                blocks_y,
                &tables,
                &palettes,
                &mut outputs,
            )?,
            CrnFormat::Bc3 => unpack_bc3(
                level_data,
                blocks_x,
                blocks_y,
                &tables,
                &palettes,
                &mut outputs,
            )?,
        }

        for (face, data) in faces.iter_mut().zip(outputs) {
            face.levels.push(CrnMipLevel {
                width,
                height,
                data,
            });
        }
    }

    // `crn_unpacker::unpack_level` clamps zero header dimensions to one
    // instead of rejecting them.
    Ok(CrnTexture {
        width: core::cmp::max(header.width, 1),
        height: core::cmp::max(header.height, 1),
        format,
        faces,
    })
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    fn synthetic_header(
        raw_levels: u8,
        faces: u8,
        flags: u16,
        header_size: u16,
        data_size: u32,
        offsets: &[u32],
    ) -> Vec<u8> {
        let table_end = LEVEL_OFS_OFFSET + offsets.len() * 4;
        let mut input = vec![0; core::cmp::max(data_size as usize, table_end)];
        input[0..2].copy_from_slice(&(CRN_SIG as u16).to_be_bytes());
        input[2..4].copy_from_slice(&header_size.to_be_bytes());
        input[6..10].copy_from_slice(&data_size.to_be_bytes());
        input[16] = raw_levels;
        input[17] = faces;
        input[19..21].copy_from_slice(&flags.to_be_bytes());
        for (level, offset) in offsets.iter().enumerate() {
            let start = LEVEL_OFS_OFFSET + level * 4;
            input[start..start + 4].copy_from_slice(&offset.to_be_bytes());
        }
        input
    }

    fn push_bits(dst: &mut Vec<u8>, bit_len: &mut usize, value: u32, count: u32) {
        for shift in (0..count).rev() {
            if *bit_len % 8 == 0 {
                dst.push(0);
            }
            let bit = ((value >> shift) & 1) as u8;
            let byte = dst.len() - 1;
            dst[byte] |= bit << (7 - (*bit_len % 8));
            *bit_len += 1;
        }
    }

    #[test]
    fn bit_reader_gets_bits_msb_first() {
        // 0b1011_0010_0000_1111 = 0xB20F
        let data = [0xB2u8, 0x0F];
        let mut r = BitReader::new(&data);
        assert_eq!(r.get_bits(1), 1);
        assert_eq!(r.get_bits(3), 0b011);
        assert_eq!(r.get_bits(4), 0b0010);
        assert_eq!(r.get_bits(8), 0x0F);
    }

    #[test]
    fn header_parses_all_mip_offsets_and_six_faces() {
        let input = synthetic_header(3, 6, 0, 82, 120, &[90, 100, 110]);
        let header = Header::parse(&input).expect("three-level cubemap header");
        assert_eq!(header.levels, 3);
        assert_eq!(header.faces, 6);
        assert_eq!(header.level_offsets, [90, 100, 110]);
    }

    #[test]
    fn header_normalizes_zero_levels_to_one() {
        let input = synthetic_header(0, 1, 0, 74, 100, &[90]);
        let header = Header::parse(&input).expect("zero levels use level zero offset");
        assert_eq!(header.levels, 1);
        assert_eq!(header.level_offsets, [90]);
    }

    #[test]
    fn header_accepts_zero_dimensions_but_enforces_crunch_maximum() {
        let zero = synthetic_header(1, 1, 0, 74, 100, &[90]);
        let header = Header::parse(&zero).expect("zero dimensions remain compatible");
        assert_eq!((header.width, header.height), (0, 0));

        let mut maximum = zero.clone();
        maximum[12..14].copy_from_slice(&(CRN_MAX_LEVEL_RESOLUTION as u16).to_be_bytes());
        maximum[14..16].copy_from_slice(&(CRN_MAX_LEVEL_RESOLUTION as u16).to_be_bytes());
        Header::parse(&maximum).expect("4096x4096 is a valid Crunch dimension");

        let mut too_wide = maximum.clone();
        too_wide[12..14].copy_from_slice(&((CRN_MAX_LEVEL_RESOLUTION + 1) as u16).to_be_bytes());
        assert!(matches!(
            Header::parse(&too_wide),
            Err(CrnDecodeError::InvalidHeader)
        ));

        let mut too_tall = maximum;
        too_tall[14..16].copy_from_slice(&((CRN_MAX_LEVEL_RESOLUTION + 1) as u16).to_be_bytes());
        assert!(matches!(
            Header::parse(&too_tall),
            Err(CrnDecodeError::InvalidHeader)
        ));
    }

    #[test]
    fn header_rejects_invalid_face_and_level_counts() {
        let invalid_faces = synthetic_header(1, 2, 0, 74, 100, &[90]);
        assert!(matches!(
            Header::parse(&invalid_faces),
            Err(CrnDecodeError::InvalidHeader)
        ));

        let too_many_levels = synthetic_header(17, 1, 0, 138, 200, &[150]);
        assert!(matches!(
            Header::parse(&too_many_levels),
            Err(CrnDecodeError::InvalidHeader)
        ));
    }

    #[test]
    fn header_checks_offset_table_coverage_and_order() {
        let short_declared_header = synthetic_header(2, 1, 0, 74, 110, &[90, 100]);
        assert!(matches!(
            Header::parse(&short_declared_header),
            Err(CrnDecodeError::InvalidHeader)
        ));

        let mut truncated_table = synthetic_header(2, 1, 0, 78, 110, &[90, 100]);
        truncated_table.truncate(76);
        assert!(matches!(
            Header::parse(&truncated_table),
            Err(CrnDecodeError::InputTooShort)
        ));

        let repeated_offset = synthetic_header(2, 1, 0, 78, 110, &[90, 90]);
        assert!(matches!(
            Header::parse(&repeated_offset),
            Err(CrnDecodeError::InvalidHeader)
        ));

        let before_header = synthetic_header(1, 1, 0, 74, 100, &[73]);
        assert!(matches!(
            Header::parse(&before_header),
            Err(CrnDecodeError::InvalidHeader)
        ));
    }

    #[test]
    fn header_distinguishes_out_of_line_segmented_mips() {
        let inline_segmented = synthetic_header(2, 1, 1, 78, 110, &[90, 100]);
        Header::parse(&inline_segmented).expect("segmented flag with inline mips");

        let out_of_line_segmented = synthetic_header(1, 1, 1, 74, 100, &[100]);
        assert!(matches!(
            Header::parse(&out_of_line_segmented),
            Err(CrnDecodeError::UnsupportedSegmented)
        ));

        let out_of_range_inline = synthetic_header(1, 1, 0, 74, 100, &[100]);
        assert!(matches!(
            Header::parse(&out_of_range_inline),
            Err(CrnDecodeError::InvalidHeader)
        ));
    }

    #[test]
    fn huffman_table_two_codes_of_equal_length() {
        // Two symbols (17 and 18) each with code length 4. Canonical codes
        // 0000 -> 17, 0001 -> 18.
        let mut sizes = std::vec![0u8; 21];
        sizes[17] = 4;
        sizes[18] = 4;
        let table = HuffmanTable::build(sizes).unwrap();
        // 0x01 = 0000_0001: first nibble 0000 -> code 0 -> sym 17, then 0001
        // -> code 1 -> sym 18.
        let mut r = BitReader::new(&[0x01]);
        assert_eq!(table.decode(&mut r).unwrap(), 17);
        assert_eq!(table.decode(&mut r).unwrap(), 18);
    }

    #[test]
    fn huffman_table_rejects_oversubscribed_code_lengths() {
        assert!(matches!(
            HuffmanTable::build(vec![1, 1, 1]),
            Err(CrnDecodeError::InvalidStream)
        ));
        HuffmanTable::build(vec![1]).expect("an incomplete single-symbol model is valid");
    }

    #[test]
    fn receives_oversized_huffman_alphabet_like_reference() {
        let mut data = Vec::new();
        let mut bit_len = 0usize;

        // The reference reads this count from 14 bits, asserts non-fatally
        // that it is <= 8192, then continues. Define symbols 0 and 8192 with
        // one-bit codes and leave every symbol between them unused.
        push_bits(&mut data, &mut bit_len, 8193, 14);
        push_bits(&mut data, &mut bit_len, 19, 5);

        // Meta-code lengths in MOST_PROBABLE_CODELENGTH_CODES order. Symbols
        // 1 and 18 both have length 1, giving canonical codes 0 and 1.
        for i in 0..19 {
            let size = if i == 1 || i == 18 { 1 } else { 0 };
            push_bits(&mut data, &mut bit_len, size, 3);
        }

        // Code size 1, then 8191 zero sizes (59*138 + 49), then size 1.
        push_bits(&mut data, &mut bit_len, 0, 1);
        for _ in 0..59 {
            push_bits(&mut data, &mut bit_len, 1, 1);
            push_bits(&mut data, &mut bit_len, 127, 7);
        }
        push_bits(&mut data, &mut bit_len, 1, 1);
        push_bits(&mut data, &mut bit_len, 38, 7);
        push_bits(&mut data, &mut bit_len, 0, 1);

        // Verify both canonical branches after receiving the model.
        push_bits(&mut data, &mut bit_len, 0, 1);
        push_bits(&mut data, &mut bit_len, 1, 1);

        let mut reader = BitReader::new(&data);
        let table = receive_static_data_model(&mut reader).expect("8193-symbol model");
        assert_eq!(table.code_sizes.len(), 8193);
        assert_eq!(table.decode(&mut reader).unwrap(), 0);
        assert_eq!(table.decode(&mut reader).unwrap(), 8192);
    }

    #[test]
    fn limit_wraps_around_once() {
        let mut x = 5u32;
        limit(&mut x, 4);
        assert_eq!(x, 1);
        let mut x = 3u32;
        limit(&mut x, 4);
        assert_eq!(x, 3);
        // Encoder guarantees index stays in [0, 2n); 6 with n=4 wraps to 2.
        let mut x = 6u32;
        limit(&mut x, 4);
        assert_eq!(x, 2);
    }

    #[test]
    fn palette_index_helper_rejects_delta_beyond_single_wrap() {
        let mut index = 1u32;
        assert_eq!(advance_palette_index(&mut index, 2, 4), Ok(3));

        let mut index = 0u32;
        assert_eq!(
            advance_palette_index(&mut index, 8, 4),
            Err(CrnDecodeError::InvalidStream)
        );
    }

    #[test]
    fn paired_delta_rejects_out_of_range_symbol() {
        assert_eq!(paired_delta(&[1], &[2], 0), Ok((1, 2)));
        assert_eq!(
            paired_delta(&[1], &[2], 1),
            Err(CrnDecodeError::InvalidStream)
        );
        assert_eq!(
            paired_delta(&[1], &[], 0),
            Err(CrnDecodeError::InvalidStream)
        );
    }

    #[test]
    fn decode_rejects_missing_selector_palettes_without_panicking() {
        let dxt1_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/crunch/dxt1-6x6.crn"
        );
        let mut dxt1 = std::fs::read(dxt1_path).unwrap();
        // m_color_selectors.m_num
        dxt1[47..49].fill(0);
        assert_eq!(decode(&dxt1), Err(CrnDecodeError::InvalidHeader));

        let dxt5_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/crunch/dxt5-6x6.crn"
        );
        let mut dxt5 = std::fs::read(dxt5_path).unwrap();
        // m_alpha_selectors.m_num
        dxt5[63..65].fill(0);
        assert_eq!(decode(&dxt5), Err(CrnDecodeError::InvalidHeader));
    }

    /// Decoding the tables + color palettes of the DXT1 fixture must succeed and
    /// produce the expected palette sizes. Exercises the static-Huffman receiver
    /// and the delta-coded palette decoders without touching the chunk stream.
    #[test]
    fn dxt1_fixture_tables_and_color_palettes_decode() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/crunch/dxt1-6x6.crn"
        );
        let input = std::fs::read(path).unwrap();
        let header = Header::parse(&input).expect("header");

        let tables_section =
            section(&input, header.tables_ofs, header.tables_size).expect("tables section");
        let mut reader = BitReader::new(tables_section);
        let _chunk = receive_static_data_model(&mut reader).expect("chunk_encoding");
        let _ce_delta = receive_static_data_model(&mut reader).expect("color_endpoint_delta");
        let _cs_delta = receive_static_data_model(&mut reader).expect("color_selector_delta");

        let ce = section(
            &input,
            header.color_endpoints.ofs,
            header.color_endpoints.size,
        )
        .expect("ce section");
        let endpoints = decode_color_endpoints(ce, header.color_endpoints.num).expect("endpoints");
        assert_eq!(endpoints.len(), header.color_endpoints.num as usize);

        let cs = section(
            &input,
            header.color_selectors.ofs,
            header.color_selectors.size,
        )
        .expect("cs section");
        let selectors = decode_color_selectors(cs, header.color_selectors.num).expect("selectors");
        assert_eq!(selectors.len(), header.color_selectors.num as usize);
    }

    #[test]
    fn total_bits_and_ceil_log2i_match_reference() {
        // total_bits counts shifts-to-zero; powers of two get one extra bit.
        assert_eq!(total_bits(8192), 14);
        assert_eq!(total_bits(8), 4);
        assert_eq!(total_bits(1), 1);
        // ceil_log2i of a power of two is the exponent itself.
        assert_eq!(ceil_log2i(32), 5);
        assert_eq!(ceil_log2i(33), 6);
        assert_eq!(ceil_log2i(8192), 13);
    }
}
