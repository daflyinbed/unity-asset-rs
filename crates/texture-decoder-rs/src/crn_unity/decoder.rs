//! Unity-fork Crunch decompression to raw DXT1/DXT5/ETC1/ETC2A blocks.
//!
//! Direct port of the decompression path in the Unity fork of Crunch
//! (`unity-crunch/inc/crn_decomp.h`, the `unity` branch). The Unity fork
//! replaced upstream Crunch's "chunk encoding" bitstream with an incompatible
//! "reference encoding" scheme — a per-2×2-block-group endpoint-index
//! predictor — and rebuilt the palette decoders around XOR / modulo deltas. As a
//! result these files are not decodable by the upstream `crn` module.
//!
//! Supported formats (from the `crn_format` enum in `unity-crunch/inc/crnlib.h`):
//!
//! - `cCRNFmtDXT1`  (0)  → DXT1 / BC1, 8 bytes/block
//! - `cCRNFmtDXT5`  (2)  → DXT5 / BC3, 16 bytes/block
//! - `cCRNFmtETC1`  (10) → ETC_RGB4 / ETC1, 8 bytes/block
//! - `cCRNFmtETC1S` (13) → ETC_RGB4 single-subblock, 8 bytes/block (via DXT1 unpacker)
//! - `cCRNFmtETC2A` (12) → ETC2_RGBA8, 16 bytes/block
//! - `cCRNFmtETC2AS`(14) → ETC2_RGBA8 single-subblock, 16 bytes/block (via DXT5 unpacker)
//!
//! Pipeline:
//!
//! 1. Parse the big-endian [`Header`] and validate it.
//! 2. `init_tables`: decode the static Huffman codebooks stored in the `tables`
//!    section — one `reference_encoding_dm`, plus per-channel endpoint and
//!    selector delta models (color and, where present, alpha).
//! 3. `decode_palettes`: decode the endpoint/selector codebooks from their own
//!    byte ranges. Unity uses XOR / modulo delta reconstruction rather than the
//!    upstream paired-delta tables.
//! 4. For each mip, walk every face and 4×4-block grid (evened up to a multiple
//!    of two) in scanline order, reading per-block-group reference encodings
//!    and per-block selector deltas, and assemble raw block bytes from palette
//!    lookups. The bitstream and predictors are continuous across the faces of
//!    one mip, then reset for the next mip.
//!
//! All multi-byte header fields are big-endian (`crn_packed_uint`). The bit
//! streams (tables, palettes, level data) are MSB-first.

use alloc::vec;
use alloc::vec::Vec;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// The Unity-fork Crunch texture format decoded from the file header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrnUnityFormat {
    /// DXT1 / BC1, 8 bytes per block.
    Bc1,
    /// DXT5 / BC3, 16 bytes per block.
    Bc3,
    /// ETC_RGB4 / ETC1, 8 bytes per block.
    Etc1Rgb4,
    /// ETC2_RGBA8, 16 bytes per block.
    Etc2Rgba8,
}

impl CrnUnityFormat {
    /// Physical block size in bytes for this format.
    pub const fn block_bytes(self) -> usize {
        match self {
            CrnUnityFormat::Bc1 | CrnUnityFormat::Etc1Rgb4 => 8,
            CrnUnityFormat::Bc3 | CrnUnityFormat::Etc2Rgba8 => 16,
        }
    }
}

/// A decoded Unity-fork Crunch texture: one 2D face or all six cubemap faces.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrnUnityTexture {
    /// The level-0 width. A zero header width is clamped to one.
    pub width: u32,
    /// The level-0 height. A zero header height is clamped to one.
    pub height: u32,
    pub format: CrnUnityFormat,
    /// One face for a 2D texture, or cubemap faces in +X, -X, +Y, -Y, +Z, -Z
    /// order.
    pub faces: Vec<CrnUnityFace>,
}

/// All mip levels belonging to one texture face.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrnUnityFace {
    /// Mips ordered largest to smallest, starting with level 0.
    pub levels: Vec<CrnUnityMipLevel>,
}

/// One decoded mip level containing tightly packed physical blocks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrnUnityMipLevel {
    pub width: u32,
    pub height: u32,
    /// `ceil(width / 4) * ceil(height / 4) * block_bytes` bytes, row-major in
    /// block space. `block_bytes` is 8 for BC1/ETC1 and 16 for BC3/ETC2_RGBA8.
    pub data: Vec<u8>,
}

/// A failure while decoding a Unity-fork Crunch file.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrnUnityDecodeError {
    /// The compressed input is too short to hold the header or declared data.
    InputTooShort,
    /// A dimension or byte-count calculation overflowed.
    SizeOverflow,
    /// The file signature is not `0x4878` (`"Hx"`).
    InvalidSignature,
    /// The CRN format is not one of DXT1/DXT5/ETC1/ETC2A.
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
/// Maximum mip levels from `cCRNMaxLevels`. The reference's decompression
/// entry point does not enforce this; kept for documentation.
#[allow(dead_code)]
const CRN_MAX_LEVELS: u32 = 16;

/// Maximum texture dimension from `cCRNMaxLevelResolution`. The reference's
/// decompression entry point does not enforce this; kept for documentation.
#[allow(dead_code)]
const CRN_MAX_LEVEL_RESOLUTION: u32 = 4096;

/// Initial validation threshold from `cCRNHeaderMinSize` in `crn_decomp.h`.
/// Level offset bytes may extend past this into the data section.
const CRN_HEADER_MIN_SIZE: usize = 62;

/// Byte offset of the variably-sized `m_level_ofs[]` array within the header.
/// Everything before it is fixed; each mip level appends one big-endian `u32`.
const LEVEL_OFS_OFFSET: usize = 70;

// CRN format enum values (from `unity-crunch/inc/crnlib.h`).
const FMT_DXT1: u8 = 0;
const FMT_DXT5: u8 = 2;
// DXT5 swizzled variants unpack identically to plain DXT5.
const FMT_DXT5_CCXY: u8 = 3;
const FMT_DXT5_XGXR: u8 = 4;
const FMT_DXT5_XGBR: u8 = 5;
const FMT_DXT5_AGBR: u8 = 6;
const FMT_ETC1: u8 = 10;
const FMT_ETC2A: u8 = 12;
// The "single subblock" ETC variants unpack via the DXT1/DXT5 paths.
const FMT_ETC1S: u8 = 13;
const FMT_ETC2AS: u8 = 14;

// Prefix-coding limits (from `prefix_coding` in `crn_decomp.h`).
const MAX_EXPECTED_CODE_SIZE: usize = 16;
const MAX_SUPPORTED_SYMS: u32 = 8192;
const MAX_TABLE_BITS: u32 = 11;

/// `g_dxt5_from_linear` in `crn_decomp.h:2736`. Maps a linear 3-bit alpha
/// selector index to the DXT5/BC4 selector ordering.
const DXT5_FROM_LINEAR: [u32; 8] = [0, 2, 3, 4, 5, 6, 7, 1];

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
struct Header {
    #[allow(dead_code)]
    header_size: u32,
    #[allow(dead_code)]
    data_size: u32,
    width: u32,
    height: u32,
    levels: u32,
    faces: u32,
    format: u8,
    #[allow(dead_code)]
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
    fn parse(input: &[u8]) -> Result<Header, CrnUnityDecodeError> {
        if input.len() < CRN_HEADER_MIN_SIZE {
            return Err(CrnUnityDecodeError::InputTooShort);
        }

        let sig = read_packed(input, 0, 2).ok_or(CrnUnityDecodeError::InputTooShort)?;
        if sig != CRN_SIG {
            return Err(CrnUnityDecodeError::InvalidSignature);
        }

        let header_size = read_packed(input, 2, 2).ok_or(CrnUnityDecodeError::InputTooShort)?;
        // m_header_crc16 at offset 4 (2 bytes) is not checked.
        let data_size = read_packed(input, 6, 4).ok_or(CrnUnityDecodeError::InputTooShort)?;
        // m_data_crc16 at offset 10 (2 bytes) is not checked.
        let width = read_packed(input, 12, 2).ok_or(CrnUnityDecodeError::InputTooShort)?;
        let height = read_packed(input, 14, 2).ok_or(CrnUnityDecodeError::InputTooShort)?;
        // Some unchecked reference paths treat zero as a single level. Keep
        // that compatibility while still bounding the variable offset table.
        let levels = read_packed(input, 16, 1)
            .ok_or(CrnUnityDecodeError::InputTooShort)?
            .max(1);
        let faces = read_packed(input, 17, 1).ok_or(CrnUnityDecodeError::InputTooShort)?;
        let format = read_packed(input, 18, 1).ok_or(CrnUnityDecodeError::InputTooShort)? as u8;
        let flags = read_packed(input, 19, 2).ok_or(CrnUnityDecodeError::InputTooShort)?;
        // m_reserved (4), m_userdata0 (4), m_userdata1 (4) at offsets 21..33.

        let color_endpoints = read_palette(input, 33)?;
        let color_selectors = read_palette(input, 41)?;
        let alpha_endpoints = read_palette(input, 49)?;
        let alpha_selectors = read_palette(input, 57)?;

        let tables_size = read_packed(input, 65, 2).ok_or(CrnUnityDecodeError::InputTooShort)?;
        let tables_ofs = read_packed(input, 67, 3).ok_or(CrnUnityDecodeError::InputTooShort)?;

        // --- Validation matching `crnd_get_header` (crn_decomp.h:2175-2187) ---
        // The reference's decompression entry point (`crnd_unpack_begin`) uses
        // only these basic checks; the stricter structural validation (faces,
        // dimensions, level ordering, offset coverage) lives in the optional
        // `crnd_validate_file` API which the hot path never calls.
        if header_size < CRN_HEADER_MIN_SIZE as u32 {
            return Err(CrnUnityDecodeError::InvalidHeader);
        }
        if (input.len() as u32) < data_size {
            return Err(CrnUnityDecodeError::InvalidHeader);
        }
        if flags & CRN_HEADER_FLAG_SEGMENTED != 0 {
            return Err(CrnUnityDecodeError::UnsupportedSegmented);
        }

        // Read the variably-sized level offset table directly from the input,
        // matching the reference which accesses `m_pHeader->m_level_ofs[i]`
        // regardless of `m_header_size`.
        let mut level_offsets = Vec::with_capacity(levels as usize);
        for level in 0..levels as usize {
            let offset = LEVEL_OFS_OFFSET + level * 4;
            level_offsets
                .push(read_packed(input, offset, 4).ok_or(CrnUnityDecodeError::InputTooShort)?);
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
fn read_palette(input: &[u8], offset: usize) -> Result<PaletteDesc, CrnUnityDecodeError> {
    let ofs = read_packed(input, offset, 3).ok_or(CrnUnityDecodeError::InputTooShort)?;
    let size = read_packed(input, offset + 3, 3).ok_or(CrnUnityDecodeError::InputTooShort)?;
    let num = read_packed(input, offset + 6, 2).ok_or(CrnUnityDecodeError::InputTooShort)?;
    Ok(PaletteDesc { ofs, size, num })
}

/// Returns the byte slice for a palette/table section, validating its bounds.
fn section(input: &[u8], ofs: u32, size: u32) -> Result<&[u8], CrnUnityDecodeError> {
    let start = ofs as usize;
    let end = start
        .checked_add(size as usize)
        .ok_or(CrnUnityDecodeError::InvalidHeader)?;
    input
        .get(start..end)
        .ok_or(CrnUnityDecodeError::InputTooShort)
}

// ---------------------------------------------------------------------------
// MSB-first bit reader
// ---------------------------------------------------------------------------

/// An MSB-first bit reader over a byte slice.
///
/// Direct port of `symbol_codec`'s `get_bits` / `decode_bits` / refill loop
/// in `crn_decomp.h`. Bytes past the end of the buffer read as zero (the
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
            let c = self.next_byte();
            self.bit_count += 8;
            // Place the new byte at the top of the accumulator, after the
            // currently-valid bits. Matches `m_bit_buf |= (c << (32 - count))`.
            self.bit_buf |= u32::from(c) << (32 - self.bit_count);
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

    /// Refills the accumulator to at least 16 (or 24) valid bits, mirroring the
    /// inline refill at the top of `symbol_codec::decode`. Used before a Huffman
    /// lookup so the fast path can read directly from `bit_buf`.
    #[inline]
    fn refill_for_decode(&mut self) {
        if self.bit_count < 16 {
            let c0 = self.next_byte();
            let c1 = self.next_byte();
            self.bit_count += 16;
            let c = (u32::from(c0) << 8) | u32::from(c1);
            self.bit_buf |= c << (32 - self.bit_count);
        } else if self.bit_count < 24 {
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
    fn build(code_sizes: Vec<u8>) -> Result<HuffmanTable, CrnUnityDecodeError> {
        let num_syms = code_sizes.len();
        if num_syms == 0 {
            return Err(CrnUnityDecodeError::InvalidStream);
        }

        let table_bits = if num_syms > 16 {
            let log2 = ceil_log2i(num_syms as u32);
            core::cmp::min(log2 + 1, MAX_TABLE_BITS)
        } else {
            0
        };

        let mut num_codes = [0u32; MAX_EXPECTED_CODE_SIZE + 1];
        for &c in &code_sizes {
            if c != 0 {
                if c as usize > MAX_EXPECTED_CODE_SIZE {
                    return Err(CrnUnityDecodeError::InvalidStream);
                }
                num_codes[c as usize] += 1;
            }
        }

        // The reference's `decoder_tables::init` performs no Kraft/oversubscription
        // validation — it unconditionally builds the lookup table. Matching that
        // behaviour lets us decode the same (occasionally buggy) models the
        // reference accepts.

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
            return Err(CrnUnityDecodeError::InvalidStream);
        }

        let mut sorted_order = vec![0u16; total_used_syms as usize];
        for (sym, &c) in code_sizes.iter().enumerate() {
            if c != 0 {
                let pos = sorted_positions[c as usize];
                sorted_positions[c as usize] += 1;
                sorted_order[pos as usize] = sym as u16;
            }
        }

        let table_bits = if table_bits <= min_code_size {
            0
        } else {
            table_bits
        };

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

        for i in 0..MAX_EXPECTED_CODE_SIZE {
            val_ptrs[i] -= min_codes[i] as i32;
        }

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
    /// canonical code of a given length. See `HuffmanTable::build`.
    #[inline]
    fn unshifted_max_code(stored: u32, i: u32) -> u32 {
        (stored - 1) >> (16 - i)
    }

    /// Decode one symbol. Port of `symbol_codec::decode`.
    ///
    /// The slow path mirrors the reference decompressor's recovery: if the
    /// resolved symbol index falls outside the sorted table, the reference
    /// prints `CRND_ASSERT(0)` and `return 0` — recovering as symbol 0 without
    /// consuming any bits. Degenerate single-symbol delta models hit this path
    /// on real crunch-encoded files, so the recovery must be reproduced
    /// verbatim, including the zero-bit consumption, or the stream's bit
    /// alignment diverges.
    #[inline]
    fn decode(&self, reader: &mut BitReader) -> Result<u32, CrnUnityDecodeError> {
        reader.refill_for_decode();

        let k = (reader.bit_buf >> 16) + 1;

        let (sym, len) = if k <= self.table_max_code {
            let idx = (reader.bit_buf >> (32 - self.table_bits)) as usize;
            let t = self.lookup[idx];
            if t == u32::MAX {
                return Err(CrnUnityDecodeError::InvalidStream);
            }
            (t & 0xFFFF, t >> 16)
        } else {
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
fn receive_static_data_model(reader: &mut BitReader) -> Result<HuffmanTable, CrnUnityDecodeError> {
    let total_used_syms = reader.decode_bits(total_bits(MAX_SUPPORTED_SYMS));
    if total_used_syms == 0 {
        return Err(CrnUnityDecodeError::InvalidStream);
    }

    let mut code_sizes = vec![0u8; total_used_syms as usize];

    let num_cl_codes = reader.decode_bits(5);
    if !(1..=21).contains(&num_cl_codes) {
        return Err(CrnUnityDecodeError::InvalidStream);
    }

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
                return Err(CrnUnityDecodeError::InvalidStream);
            }
            ofs += len;
        } else if code == LARGE_ZERO_RUN_CODE {
            let len = reader.decode_bits(LARGE_ZERO_RUN_EXTRA_BITS) + MIN_LARGE_ZERO_RUN_SIZE;
            if len > num_remaining {
                return Err(CrnUnityDecodeError::InvalidStream);
            }
            ofs += len;
        } else if code == SMALL_REPEAT_CODE || code == LARGE_REPEAT_CODE {
            let len = if code == SMALL_REPEAT_CODE {
                reader.decode_bits(SMALL_NONZERO_RUN_EXTRA_BITS) + SMALL_MIN_NONZERO_RUN_SIZE
            } else {
                reader.decode_bits(LARGE_NONZERO_RUN_EXTRA_BITS) + LARGE_MIN_NONZERO_RUN_SIZE
            };
            if ofs == 0 || len > num_remaining {
                return Err(CrnUnityDecodeError::InvalidStream);
            }
            let prev = code_sizes[(ofs - 1) as usize];
            if prev == 0 {
                return Err(CrnUnityDecodeError::InvalidStream);
            }
            let end = ofs + len;
            while ofs < end {
                code_sizes[ofs as usize] = prev;
                ofs += 1;
            }
        } else {
            return Err(CrnUnityDecodeError::InvalidStream);
        }
    }

    if ofs != total_used_syms {
        return Err(CrnUnityDecodeError::InvalidStream);
    }
    HuffmanTable::build(code_sizes)
}

// ---------------------------------------------------------------------------
// Tables section
// ---------------------------------------------------------------------------

/// The static Huffman codebooks shared across every block of a level.
///
/// Port of `crn_unpacker::init_tables` (`crn_decomp.h:3087-3114`). Unity reads a
/// `reference_encoding_dm` plus, per present palette, an `endpoint_delta_dm`
/// and a `selector_delta_dm`.
struct Tables {
    reference_encoding: HuffmanTable,
    endpoint_delta: [HuffmanTable; 2], // 0 = color, 1 = alpha
    selector_delta: [HuffmanTable; 2], // 0 = color, 1 = alpha
}

/// Which palette channels are present for this format, mirroring the C++
/// `m_pHeader->m_color_endpoints.m_num` / `m_alpha_endpoints.m_num` guards.
#[derive(Clone, Copy)]
struct HasAlpha(bool);

impl Tables {
    fn decode(
        header: &Header,
        input: &[u8],
        has_alpha: HasAlpha,
    ) -> Result<Tables, CrnUnityDecodeError> {
        let data = section(input, header.tables_ofs, header.tables_size)?;
        let mut reader = BitReader::new(data);

        let reference_encoding = receive_static_data_model(&mut reader)?;

        if header.color_endpoints.num == 0 && header.alpha_endpoints.num == 0 {
            return Err(CrnUnityDecodeError::InvalidHeader);
        }

        // Placeholder degenerate model for unused channels (single one-bit
        // symbol); it decodes to 0 without needing real stream data, matching
        // the reference which simply skips absent models.
        let placeholder = || HuffmanTable::build(vec![1]).unwrap();
        let mut color_endpoint = placeholder();
        let mut color_selector = placeholder();
        let mut alpha_endpoint = placeholder();
        let mut alpha_selector = placeholder();

        if header.color_endpoints.num != 0 {
            color_endpoint = receive_static_data_model(&mut reader)?;
            color_selector = receive_static_data_model(&mut reader)?;
        }
        if has_alpha.0 && header.alpha_endpoints.num != 0 {
            alpha_endpoint = receive_static_data_model(&mut reader)?;
            alpha_selector = receive_static_data_model(&mut reader)?;
        }

        Ok(Tables {
            reference_encoding,
            endpoint_delta: [color_endpoint, alpha_endpoint],
            selector_delta: [color_selector, alpha_selector],
        })
    }
}

// ---------------------------------------------------------------------------
// Palettes
// ---------------------------------------------------------------------------

/// All decoded palette data needed to assemble blocks.
struct Palettes {
    color_endpoints: Vec<u32>,
    color_selectors: Vec<u32>,
    alpha_endpoints: Vec<u16>,
    /// DXT5: `3 * num` u16 (three packed u16 per selector entry).
    /// ETC2A: `6 * num` u16 (six packed u16 per selector entry).
    /// Unused for BC1/ETC1.
    alpha_selectors: Vec<u16>,
}

/// Per-format palette/selector reconstruction flags. Mirrors the two C++ flags
/// in `decode_color_endpoints` / `decode_color_selectors`.
struct FormatFlags {
    has_etc_color_blocks: bool,
    has_subblocks: bool,
}

impl FormatFlags {
    fn new(format: u8) -> FormatFlags {
        // `has_etc_color_blocks`: color endpoints stored as 32-bit ETC words
        // (single delta model) instead of DXT's two RGB565 models.
        let has_etc_color_blocks = matches!(format, FMT_ETC1 | FMT_ETC2A | FMT_ETC1S | FMT_ETC2AS);
        // `has_subblocks`: ETC1/ETC2/ETC2A blocks have two flip-defined
        // subblocks, so each selector entry expands to two words. The "S"
        // variants have a single subblock and reuse the DXT selector layout.
        let has_subblocks = matches!(format, FMT_ETC1 | FMT_ETC2A);
        FormatFlags {
            has_etc_color_blocks,
            has_subblocks,
        }
    }
}

/// Decode the color-endpoint palette.
///
/// Port of `crn_unpacker::decode_color_endpoints` (`crn_decomp.h:3134-3175`).
/// The ETC path reads a single delta model and reconstructs 32-bit packed ETC
/// endpoint words; the DXT path reads two models and packs two RGB565 colors.
fn decode_color_endpoints(
    data: &[u8],
    count: u32,
    flags: &FormatFlags,
) -> Result<Vec<u32>, CrnUnityDecodeError> {
    let mut reader = BitReader::new(data);

    let mut dm = [
        HuffmanTable::build(vec![1]).unwrap(),
        HuffmanTable::build(vec![1]).unwrap(),
    ];
    let num_models = if flags.has_etc_color_blocks { 1 } else { 2 };
    for slot in dm.iter_mut().take(num_models) {
        *slot = receive_static_data_model(&mut reader)?;
    }

    let mut out = vec![0u32; count as usize];
    let mut a = 0u32;

    if flags.has_etc_color_blocks {
        // ETC: four bytes accumulated (delta) per endpoint, masked to 5 bits
        // per channel, then repacked. With subblocks the raw word is kept; the
        // "S" variants remap into the ETC1S layout (3-bit high nibble mirrored
        // and a fixed diff bit).
        for out_slot in out.iter_mut() {
            for b in (0..32).step_by(8) {
                a = (a + (dm[0].decode(&mut reader)? << b)) & 0x1F1F1F1F;
            }
            *out_slot = if flags.has_subblocks {
                a
            } else {
                // ETC1S: replicate the 3 high bits of the red channel and set
                // the fixed diff/table bits, then shift the low channels up.
                ((a & 0x0700_0000) << 5)
                    | ((a & 0x0700_0000) << 2)
                    | 0x0200_0000
                    | ((a & 0x001F_1F1F) << 3)
            };
        }
    } else {
        // DXT: two delta models reconstruct two RGB565 colors packed into one
        // u32 (the first 4 bytes of a BC1 block, little-endian).
        let (mut b, mut c, mut d, mut e, mut f) = (0u32, 0u32, 0u32, 0u32, 0u32);
        for out_slot in out.iter_mut().take(count as usize) {
            a = (a + dm[0].decode(&mut reader)?) & 31;
            b = (b + dm[1].decode(&mut reader)?) & 63;
            c = (c + dm[0].decode(&mut reader)?) & 31;
            d = (d + dm[0].decode(&mut reader)?) & 31;
            e = (e + dm[1].decode(&mut reader)?) & 63;
            f = (f + dm[0].decode(&mut reader)?) & 31;
            *out_slot = c | (b << 5) | (a << 11) | (f << 16) | (e << 21) | (d << 27);
        }
    }
    Ok(out)
}

/// Decode the color-selector palette.
///
/// Port of `crn_unpacker::decode_color_selectors` (`crn_decomp.h:3177-3204`).
/// The raw 32-bit selector bitmap `s` persists across entries. For ETC each
/// entry expands into one (`!has_subblocks`) or two (`has_subblocks`) words
/// whose 2-bit selector pairs are bit-interleaved into the low/high 16-bit
/// halves at ETC pixel positions. For DXT the bitmap is XOR-rearranged in place.
fn decode_color_selectors(
    data: &[u8],
    count: u32,
    flags: &FormatFlags,
) -> Result<Vec<u32>, CrnUnityDecodeError> {
    let mut reader = BitReader::new(data);
    let dm = receive_static_data_model(&mut reader)?;

    let len = (count as usize) << if flags.has_subblocks { 1 } else { 0 };
    let mut out = vec![0u32; len];

    // `s` carries across entries, matching the C++ `for (uint32 s = 0, ...)`.
    let mut s = 0u32;
    for i in 0..count as usize {
        for j in (0..32).step_by(4) {
            s ^= dm.decode(&mut reader)? << j;
        }
        if flags.has_etc_color_blocks {
            // selector bit decode + ETC subblock expansion.
            let selector = (!s & 0xAAAA_AAAA) | (!(s ^ (s >> 1)) & 0x5555_5555);
            let mut t: u32 = 8;
            for h in 0..4u32 {
                for w in 0..4u32 {
                    if flags.has_subblocks {
                        let s0 = selector >> ((w << 3) | (h << 1));
                        out[i << 1] |= (((s0 >> 1) & 1) | ((s0 & 1) << 16)) << (t & 15);
                    }
                    let s1 = selector >> ((h << 3) | (w << 1));
                    let idx = if flags.has_subblocks { (i << 1) | 1 } else { i };
                    out[idx] |= (((s1 >> 1) & 1) | ((s1 & 1) << 16)) << (t & 15);
                    t = t.wrapping_add(4);
                }
                t = t.wrapping_sub(15);
            }
        } else {
            // DXT: bit-interleave rearrangement.
            out[i] = ((s ^ (s << 1)) & 0xAAAA_AAAA) | ((s >> 1) & 0x5555_5555);
        }
    }
    Ok(out)
}

/// Decode the alpha-endpoint palette.
///
/// Port of `decode_alpha_endpoints` (`crn_decomp.h:3206-3231`). Each entry is
/// two 8-bit alphas reconstructed via modulo-delta and packed as `a | (b << 8)`
/// (the first 2 bytes of a BC3/ETC2 alpha block).
fn decode_alpha_endpoints(data: &[u8], count: u32) -> Result<Vec<u16>, CrnUnityDecodeError> {
    let mut reader = BitReader::new(data);
    let dm = receive_static_data_model(&mut reader)?;

    let mut out = Vec::with_capacity(count as usize);
    let (mut a, mut b) = (0u32, 0u32);
    for _ in 0..count {
        a = (a + dm.decode(&mut reader)?) & 255;
        b = (b + dm.decode(&mut reader)?) & 255;
        out.push((a | (b << 8)) as u16);
    }
    Ok(out)
}

/// Decode the alpha-selector palette for DXT5 (BC3) output.
///
/// Port of `decode_alpha_selectors` (`crn_decomp.h:3233-3253`). Each entry
/// produces three packed u16 (the 6 selector bytes of a BC3 alpha block,
/// 16 × 3-bit), via a 64-entry `dxt5_from_linear` lookup and 6-bit group
/// deltas. Returns `3 * count` u16.
fn decode_alpha_selectors_dxt5(data: &[u8], count: u32) -> Result<Vec<u16>, CrnUnityDecodeError> {
    let mut reader = BitReader::new(data);
    let dm = receive_static_data_model(&mut reader)?;

    // dxt5_from_linear[64] = g_dxt5_from_linear[lo] | (g_dxt5_from_linear[hi] << 3)
    let mut dxt5_from_linear = [0u8; 64];
    for i in 0..64 {
        dxt5_from_linear[i] =
            (DXT5_FROM_LINEAR[i & 7] as u8) | ((DXT5_FROM_LINEAR[i >> 3] as u8) << 3);
    }

    let mut out = vec![0u16; (count as usize) * 3];
    let (mut s0_linear, mut s1_linear) = (0u32, 0u32);
    let mut idx = 0usize;
    while idx < out.len() {
        let (mut s0, mut s1) = (0u32, 0u32);
        for j in (0..24).step_by(6) {
            s0_linear ^= dm.decode(&mut reader)? << j;
            s0 |= (dxt5_from_linear[((s0_linear >> j) & 0x3F) as usize] as u32) << j;
        }
        for j in (0..24).step_by(6) {
            s1_linear ^= dm.decode(&mut reader)? << j;
            s1 |= (dxt5_from_linear[((s1_linear >> j) & 0x3F) as usize] as u32) << j;
        }
        out[idx] = s0 as u16;
        out[idx + 1] = ((s0 >> 16) | (s1 << 8)) as u16;
        out[idx + 2] = (s1 >> 8) as u16;
        idx += 3;
    }
    Ok(out)
}

/// Decode the alpha-selector palette for ETC2A output.
///
/// Port of `decode_alpha_selectors_etc` (`crn_decomp.h:3255-3284`). Each entry
/// produces six packed u16 (12 selector bytes) bit-packed into the rotated
/// ETC2 alpha block layout. Returns `6 * count` u16.
///
/// The C++ code casts `m_alpha_selectors.begin()` to `uint8*` and writes 12
/// bytes per entry byte-wise; `d` is a `uint8` that wraps mod 256. `s_linear`
/// is an 8-byte accumulator shared across all entries.
fn decode_alpha_selectors_etc(data: &[u8], count: u32) -> Result<Vec<u16>, CrnUnityDecodeError> {
    let mut reader = BitReader::new(data);
    let dm = receive_static_data_model(&mut reader)?;

    let total = (count as usize) * 6;
    // The reference indexes `data[byte_offset + 6]`, which for p=15 lands at
    // the first byte *past* the current 12-byte entry (into the next entry's
    // slot, or one byte past the buffer for the final entry). Over-allocate by
    // one extra entry so those trailing writes are in-bounds; the surplus is
    // dropped before packing into u16.
    let usable = (count as usize) * 12;
    let mut bytes = vec![0u8; usable + 12];
    let mut s_linear = [0u8; 8];

    let mut entry = 0usize;
    let mut data_ptr = 0usize;
    while entry < total {
        // `s_group` resets to 0 each entry; `p` runs 0..16.
        let mut s_group: u8 = 0;
        for p in 0..16u8 {
            s_group = if p & 1 != 0 {
                s_group >> 3
            } else {
                // s_linear[p >> 1] ^= decode(dm);  assignment yields the new value.
                let decoded = dm.decode(&mut reader)? as u8;
                s_linear[(p >> 1) as usize] ^= decoded;
                s_linear[(p >> 1) as usize]
            };
            let mut s = s_group & 7;
            if s <= 3 {
                s = 3 - s;
            }
            // C++ promotes `uint8 << n` to int before the shift, then truncates
            // back to uint8 via `|=`. Widen to u16 so a shift count of 8 (when
            // bit_offset == 0) is well-defined and truncates to zero, matching
            // the reference.
            let s16 = u16::from(s);
            // `d` is uint8 in C++ and wraps mod 256; keep it as u8 to match.
            let mut d: u8 = (3u8).wrapping_mul(p.wrapping_add(1));
            let mut byte_offset = (d >> 3) as usize;
            let bit_offset = d & 7;
            bytes[data_ptr + byte_offset] |= (s16 << (8 - bit_offset)) as u8;
            if bit_offset < 3 {
                bytes[data_ptr + byte_offset - 1] |= s >> bit_offset;
            }
            d = d.wrapping_add((9u8).wrapping_mul((p & 3).wrapping_sub(p >> 2)));
            byte_offset = (d >> 3) as usize;
            let bit_offset = d & 7;
            bytes[data_ptr + byte_offset + 6] |= (s16 << (8 - bit_offset)) as u8;
            if bit_offset < 3 {
                bytes[data_ptr + byte_offset + 5] |= s >> bit_offset;
            }
        }
        entry += 6;
        data_ptr += 12;
    }

    // Reinterpret the 12 bytes per entry as six little-endian u16.
    let mut out = vec![0u16; total];
    for i in 0..total {
        out[i] = u16::from_le_bytes([bytes[i * 2], bytes[i * 2 + 1]]);
    }
    Ok(out)
}

/// Decode the alpha-selector palette for ETC2AS (single-subblock) output.
///
/// Port of `decode_alpha_selectors_etcs` (`crn_decomp.h:3286-3309`). Like
/// [`decode_alpha_selectors_etc`] but produces only three packed u16 (6
/// selector bytes) per entry — the single-subblock variant. Returns
/// `3 * count` u16.
fn decode_alpha_selectors_etcs(data: &[u8], count: u32) -> Result<Vec<u16>, CrnUnityDecodeError> {
    let mut reader = BitReader::new(data);
    let dm = receive_static_data_model(&mut reader)?;

    let total = (count as usize) * 3;
    // Each entry occupies 6 bytes. The inner loop can index one byte past
    // the entry (a no-op write of zero when p=15); over-allocate to stay
    // in bounds.
    let usable = (count as usize) * 6;
    let mut bytes = vec![0u8; usable + 6];
    let mut s_linear = [0u8; 8];

    let mut data_ptr = 0usize;
    for _ in 0..count {
        let mut s_group: u8 = 0;
        for p in 0..16u8 {
            s_group = if p & 1 != 0 {
                s_group >> 3
            } else {
                let decoded = dm.decode(&mut reader)? as u8;
                s_linear[(p >> 1) as usize] ^= decoded;
                s_linear[(p >> 1) as usize]
            };
            let mut s = s_group & 7;
            if s <= 3 {
                s = 3 - s;
            }
            let s16 = u16::from(s);
            // The single-subblock layout uses the combined `d` formula that
            // matches the second subblock of the full ETC2A variant.
            let d: u8 = (3u8)
                .wrapping_mul(p.wrapping_add(1))
                .wrapping_add((9u8).wrapping_mul((p & 3).wrapping_sub(p >> 2)));
            let byte_offset = (d >> 3) as usize;
            let bit_offset = d & 7;
            bytes[data_ptr + byte_offset] |= (s16 << (8 - bit_offset)) as u8;
            if bit_offset < 3 {
                bytes[data_ptr + byte_offset - 1] |= s >> bit_offset;
            }
        }
        data_ptr += 6;
    }

    let mut out = vec![0u16; total];
    for i in 0..total {
        out[i] = u16::from_le_bytes([bytes[i * 2], bytes[i * 2 + 1]]);
    }
    Ok(out)
}

impl Palettes {
    fn decode(
        header: &Header,
        input: &[u8],
        format: u8,
        has_alpha: HasAlpha,
    ) -> Result<Palettes, CrnUnityDecodeError> {
        let flags = FormatFlags::new(format);
        let mut p = Palettes {
            color_endpoints: Vec::new(),
            color_selectors: Vec::new(),
            alpha_endpoints: Vec::new(),
            alpha_selectors: Vec::new(),
        };

        if header.color_endpoints.num == 0 || header.color_selectors.num == 0 {
            return Err(CrnUnityDecodeError::InvalidHeader);
        }
        p.color_endpoints = decode_color_endpoints(
            section(
                input,
                header.color_endpoints.ofs,
                header.color_endpoints.size,
            )?,
            header.color_endpoints.num,
            &flags,
        )?;
        p.color_selectors = decode_color_selectors(
            section(
                input,
                header.color_selectors.ofs,
                header.color_selectors.size,
            )?,
            header.color_selectors.num,
            &flags,
        )?;

        if has_alpha.0 {
            if header.alpha_endpoints.num == 0 || header.alpha_selectors.num == 0 {
                return Err(CrnUnityDecodeError::InvalidHeader);
            }
            p.alpha_endpoints = decode_alpha_endpoints(
                section(
                    input,
                    header.alpha_endpoints.ofs,
                    header.alpha_endpoints.size,
                )?,
                header.alpha_endpoints.num,
            )?;
            p.alpha_selectors = if format == FMT_ETC2AS {
                decode_alpha_selectors_etcs(
                    section(
                        input,
                        header.alpha_selectors.ofs,
                        header.alpha_selectors.size,
                    )?,
                    header.alpha_selectors.num,
                )?
            } else if format == FMT_ETC2A {
                decode_alpha_selectors_etc(
                    section(
                        input,
                        header.alpha_selectors.ofs,
                        header.alpha_selectors.size,
                    )?,
                    header.alpha_selectors.num,
                )?
            } else {
                decode_alpha_selectors_dxt5(
                    section(
                        input,
                        header.alpha_selectors.ofs,
                        header.alpha_selectors.size,
                    )?,
                    header.alpha_selectors.num,
                )?
            };
        }

        Ok(p)
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

/// Per-column predictor state retained across rows within one mip, mirroring
/// `block_buffer_element` (`crn_decomp.h:3079-3084`). Only the fields each
/// unpacker uses are meaningful; the others stay zero for non-alpha formats.
#[derive(Clone, Copy, Default)]
struct BlockBuffer {
    endpoint_reference: u8,
    color_endpoint_index: u16,
    alpha0_endpoint_index: u16,
    #[allow(dead_code)]
    alpha1_endpoint_index: u16,
}

/// Decodes one mip's continuous face stream into raw DXT1 (BC1) blocks.
///
/// Port of `crn_unpacker::unpack_dxt1` (`crn_decomp.h:3337-3386`). Unity walks
/// an evened-up 2×-multiple block grid in scanline order; every 2×2 block group
/// consumes one `reference_encoding` symbol whose low 2 bits drive the y=0 row
/// and whose next 2 bits (stashed in the per-column buffer) drive the y=1 row.
fn unpack_dxt1(
    data: &[u8],
    blocks_x: u32,
    blocks_y: u32,
    tables: &Tables,
    palettes: &Palettes,
    outputs: &mut [Vec<u8>],
) -> Result<(), CrnUnityDecodeError> {
    let block_bytes: usize = 8;
    // Even up to a multiple of two in block space.
    let width = (blocks_x + 1) & !1;
    let height = (blocks_y + 1) & !1;
    // Output buffers are tightly packed using the logical (un-padded) row
    // pitch. The reference achieves the same by advancing through the padded
    // grid but compensating with a negative `delta_pitch_in_dwords`.
    let row_pitch = (blocks_x as usize) * block_bytes;

    let num_color_endpoints = palettes.color_endpoints.len() as u32;

    let mut reader = BitReader::new(data);
    let mut buffer = vec![BlockBuffer::default(); width as usize];

    let mut color_endpoint_index: u32 = 0;
    let mut reference_group: u8 = 0;

    for output in outputs.iter_mut() {
        for y in 0..height {
            let visible_row = y < blocks_y;
            let row_base = (y as usize) * row_pitch;
            for x in 0..width {
                let visible = visible_row && x < blocks_x;
                if y & 1 == 0 && x & 1 == 0 {
                    reference_group = tables.reference_encoding.decode(&mut reader)? as u8;
                }
                let cell = &mut buffer[x as usize];
                let endpoint_reference: u8 = if y & 1 != 0 {
                    cell.endpoint_reference
                } else {
                    let er = reference_group & 3;
                    reference_group >>= 2;
                    cell.endpoint_reference = reference_group & 3;
                    reference_group >>= 2;
                    er
                };
                if endpoint_reference == 0 {
                    color_endpoint_index = color_endpoint_index
                        .wrapping_add(tables.endpoint_delta[0].decode(&mut reader)?);
                    limit(&mut color_endpoint_index, num_color_endpoints);
                    cell.color_endpoint_index = color_endpoint_index as u16;
                } else if endpoint_reference == 1 {
                    cell.color_endpoint_index = color_endpoint_index as u16;
                } else {
                    color_endpoint_index = cell.color_endpoint_index as u32;
                }
                let color_selector_index = tables.selector_delta[0].decode(&mut reader)?;
                if visible {
                    let off = row_base + (x as usize) * block_bytes;
                    let endpoints = palettes.color_endpoints[color_endpoint_index as usize];
                    let selectors = palettes.color_selectors[color_selector_index as usize];
                    output[off..off + 4].copy_from_slice(&endpoints.to_le_bytes());
                    output[off + 4..off + 8].copy_from_slice(&selectors.to_le_bytes());
                }
            }
        }
    }
    Ok(())
}

/// Decodes one mip's continuous face stream into raw DXT5 (BC3) blocks.
///
/// Port of `crn_unpacker::unpack_dxt5` (`crn_decomp.h:3388-3449`). Like
/// [`unpack_dxt1`] but also maintains an alpha0 endpoint predictor and writes
/// 16-byte blocks: alpha endpoints + selectors first, then color.
fn unpack_dxt5(
    data: &[u8],
    blocks_x: u32,
    blocks_y: u32,
    tables: &Tables,
    palettes: &Palettes,
    outputs: &mut [Vec<u8>],
) -> Result<(), CrnUnityDecodeError> {
    let block_bytes: usize = 16;
    let width = (blocks_x + 1) & !1;
    let height = (blocks_y + 1) & !1;
    let row_pitch = (blocks_x as usize) * block_bytes;

    let num_color_endpoints = palettes.color_endpoints.len() as u32;
    let num_alpha_endpoints = palettes.alpha_endpoints.len() as u32;

    let mut reader = BitReader::new(data);
    let mut buffer = vec![BlockBuffer::default(); width as usize];

    let mut color_endpoint_index: u32 = 0;
    let mut alpha0_endpoint_index: u32 = 0;
    let mut reference_group: u8 = 0;

    for output in outputs.iter_mut() {
        for y in 0..height {
            let visible_row = y < blocks_y;
            let row_base = (y as usize) * row_pitch;
            for x in 0..width {
                let visible = visible_row && x < blocks_x;
                if y & 1 == 0 && x & 1 == 0 {
                    reference_group = tables.reference_encoding.decode(&mut reader)? as u8;
                }
                let cell = &mut buffer[x as usize];
                let endpoint_reference: u8 = if y & 1 != 0 {
                    cell.endpoint_reference
                } else {
                    let er = reference_group & 3;
                    reference_group >>= 2;
                    cell.endpoint_reference = reference_group & 3;
                    reference_group >>= 2;
                    er
                };
                if endpoint_reference == 0 {
                    color_endpoint_index = color_endpoint_index
                        .wrapping_add(tables.endpoint_delta[0].decode(&mut reader)?);
                    limit(&mut color_endpoint_index, num_color_endpoints);
                    cell.color_endpoint_index = color_endpoint_index as u16;
                    alpha0_endpoint_index = alpha0_endpoint_index
                        .wrapping_add(tables.endpoint_delta[1].decode(&mut reader)?);
                    limit(&mut alpha0_endpoint_index, num_alpha_endpoints);
                    cell.alpha0_endpoint_index = alpha0_endpoint_index as u16;
                } else if endpoint_reference == 1 {
                    cell.color_endpoint_index = color_endpoint_index as u16;
                    cell.alpha0_endpoint_index = alpha0_endpoint_index as u16;
                } else {
                    color_endpoint_index = cell.color_endpoint_index as u32;
                    alpha0_endpoint_index = cell.alpha0_endpoint_index as u32;
                }
                let color_selector_index = tables.selector_delta[0].decode(&mut reader)?;
                let alpha0_selector_index = tables.selector_delta[1].decode(&mut reader)?;
                if visible {
                    let off = row_base + (x as usize) * block_bytes;
                    let sel_base = (alpha0_selector_index as usize) * 3;
                    let a0 = u32::from(palettes.alpha_selectors[sel_base]);
                    let a1 = u32::from(palettes.alpha_selectors[sel_base + 1]);
                    let a2 = u32::from(palettes.alpha_selectors[sel_base + 2]);

                    let dw0 = u32::from(palettes.alpha_endpoints[alpha0_endpoint_index as usize])
                        | (a0 << 16);
                    let dw1 = a1 | (a2 << 16);
                    let dw2 = palettes.color_endpoints[color_endpoint_index as usize];
                    let dw3 = palettes.color_selectors[color_selector_index as usize];

                    output[off..off + 4].copy_from_slice(&dw0.to_le_bytes());
                    output[off + 4..off + 8].copy_from_slice(&dw1.to_le_bytes());
                    output[off + 8..off + 12].copy_from_slice(&dw2.to_le_bytes());
                    output[off + 12..off + 16].copy_from_slice(&dw3.to_le_bytes());
                }
            }
        }
    }
    Ok(())
}

/// Decodes one mip's continuous face stream into raw ETC1 (ETC_RGB4) blocks.
///
/// Port of `crn_unpacker::unpack_etc1` (`crn_decomp.h:3566-3630`). ETC blocks
/// carry two subblocks, so each block group consumes a 4-bit reference encoding
/// (low 2 bits for subblock 0, high 2 bits selecting flip/reuse for subblock 1)
/// and maintains a diagonal endpoint predictor. The diff/individual bit is
/// decided from whether the two 5-bit base colors fit the signed-3-bit delta.
fn unpack_etc1(
    data: &[u8],
    blocks_x: u32,
    blocks_y: u32,
    tables: &Tables,
    palettes: &Palettes,
    outputs: &mut [Vec<u8>],
) -> Result<(), CrnUnityDecodeError> {
    let block_bytes: usize = 8;
    let width = (blocks_x + 1) & !1;
    let height = (blocks_y + 1) & !1;
    let row_pitch = (blocks_x as usize) * block_bytes;

    let num_color_endpoints = palettes.color_endpoints.len() as u32;

    let mut reader = BitReader::new(data);
    // Two slots per column (one per ETC subblock).
    let mut buffer = vec![BlockBuffer::default(); (width as usize) * 2];

    let mut color_endpoint_index: u32 = 0;
    let mut diagonal_color_endpoint_index: u32 = 0;
    let mut reference_group: u8;

    for output in outputs.iter_mut() {
        for y in 0..height {
            let visible_row = y < blocks_y;
            let row_base = (y as usize) * row_pitch;
            for x in 0..width {
                let visible = visible_row && x < blocks_x;
                let x2 = (x as usize) * 2;

                // Resolve the 4-bit endpoint_reference. On even rows a fresh
                // reference_group is decoded and split across this block and
                // the per-column buffer; on odd rows it is read back from the
                // buffer.
                let endpoint_reference: u8 = if y & 1 != 0 {
                    buffer[x2].endpoint_reference
                } else {
                    reference_group = tables.reference_encoding.decode(&mut reader)? as u8;
                    buffer[x2].endpoint_reference =
                        (reference_group >> 2 & 3) | (reference_group >> 4 & 12);
                    (reference_group & 3) | (reference_group >> 2 & 12)
                };

                // Subblock 0 color endpoint ladder.
                let er0 = endpoint_reference & 3;
                if er0 == 0 {
                    color_endpoint_index = color_endpoint_index
                        .wrapping_add(tables.endpoint_delta[0].decode(&mut reader)?);
                    limit(&mut color_endpoint_index, num_color_endpoints);
                    buffer[x2].color_endpoint_index = color_endpoint_index as u16;
                } else if er0 == 1 {
                    buffer[x2].color_endpoint_index = color_endpoint_index as u16;
                } else if er0 == 3 {
                    color_endpoint_index = diagonal_color_endpoint_index;
                    buffer[x2].color_endpoint_index = color_endpoint_index as u16;
                } else {
                    color_endpoint_index = buffer[x2].color_endpoint_index as u32;
                }
                let er1 = endpoint_reference >> 2;

                let e0 = palettes.color_endpoints[color_endpoint_index as usize];
                let selector_index = tables.selector_delta[0].decode(&mut reader)?;

                // Subblock 1 color endpoint: advance only if its nibble != 0.
                if er1 != 0 {
                    color_endpoint_index = color_endpoint_index
                        .wrapping_add(tables.endpoint_delta[0].decode(&mut reader)?);
                    limit(&mut color_endpoint_index, num_color_endpoints);
                }
                diagonal_color_endpoint_index = buffer[x2 + 1].color_endpoint_index as u32;
                buffer[x2 + 1].color_endpoint_index = color_endpoint_index as u16;
                let e1 = palettes.color_endpoints[color_endpoint_index as usize];

                if visible {
                    let (block_endpoint, selector_word) =
                        etc_pack_block(e0, e1, er1, selector_index as usize, palettes);
                    let off = row_base + (x as usize) * block_bytes;
                    output[off..off + 4].copy_from_slice(&block_endpoint.to_le_bytes());
                    output[off + 4..off + 8].copy_from_slice(&selector_word.to_le_bytes());
                }
            }
        }
    }
    Ok(())
}

/// Packs two 5-bit-per-channel ETC base colors into a 4-byte ETC color block
/// word and looks up the selector word. Port of the visible-block tail of
/// `unpack_etc1` (`crn_decomp.h:3616-3625`).
///
/// `e0`/`e1` are the decoded 32-bit endpoint words; their low three bytes hold
/// the R/G/B 5-bit base colors and byte 3 the per-subblock diff/table flags.
fn etc_pack_block(
    e0: u32,
    e1: u32,
    er1: u8,
    selector_index: usize,
    palettes: &Palettes,
) -> (u32, u32) {
    let e0b = e0.to_le_bytes();
    let e1b = e1.to_le_bytes();
    let flip: u8 = (er1 >> 1) ^ 1;
    let mut diff: u8 = 1;
    let mut c = 0;
    while diff != 0 && c < 3 {
        let lo = e0b[c] as i32;
        let hi = e1b[c] as i32;
        if !(lo + 3 >= hi && hi + 4 >= lo) {
            diff = 0;
        }
        c += 1;
    }
    let mut block_endpoint = [0u8; 4];
    for cc in 0..3 {
        block_endpoint[cc] = if diff != 0 {
            (e0b[cc] << 3) | (((e1b[cc] as i32 - e0b[cc] as i32) & 7) as u8)
        } else {
            ((e0b[cc] << 3) & 0xF0) | (e1b[cc] >> 1)
        };
    }
    block_endpoint[3] = (e0b[3] << 5) | (e1b[3] << 2) | (diff << 1) | flip;
    let block_word = u32::from_le_bytes(block_endpoint);
    let selector_word = palettes.color_selectors[(selector_index << 1) | flip as usize];
    (block_word, selector_word)
}

/// Decodes one mip's continuous face stream into raw ETC2_RGBA8 blocks.
///
/// Port of `crn_unpacker::unpack_etc2a` (`crn_decomp.h:3632-3710`). Combines the
/// ETC1 color path with an alpha0 predictor (endpoints + the six-u16 ETC alpha
/// selector entries), writing 16-byte blocks: 8 bytes alpha then 8 bytes color.
fn unpack_etc2a(
    data: &[u8],
    blocks_x: u32,
    blocks_y: u32,
    tables: &Tables,
    palettes: &Palettes,
    outputs: &mut [Vec<u8>],
) -> Result<(), CrnUnityDecodeError> {
    let block_bytes: usize = 16;
    let width = (blocks_x + 1) & !1;
    let height = (blocks_y + 1) & !1;
    let row_pitch = (blocks_x as usize) * block_bytes;

    let num_color_endpoints = palettes.color_endpoints.len() as u32;
    let num_alpha_endpoints = palettes.alpha_endpoints.len() as u32;

    let mut reader = BitReader::new(data);
    let mut buffer = vec![BlockBuffer::default(); (width as usize) * 2];

    let mut color_endpoint_index: u32 = 0;
    let mut alpha0_endpoint_index: u32 = 0;
    let mut diagonal_color_endpoint_index: u32 = 0;
    let mut diagonal_alpha0_endpoint_index: u32 = 0;
    let mut reference_group: u8;

    for output in outputs.iter_mut() {
        for y in 0..height {
            let visible_row = y < blocks_y;
            let row_base = (y as usize) * row_pitch;
            for x in 0..width {
                let visible = visible_row && x < blocks_x;
                let x2 = (x as usize) * 2;

                let endpoint_reference: u8 = if y & 1 != 0 {
                    buffer[x2].endpoint_reference
                } else {
                    reference_group = tables.reference_encoding.decode(&mut reader)? as u8;
                    buffer[x2].endpoint_reference =
                        (reference_group >> 2 & 3) | (reference_group >> 4 & 12);
                    (reference_group & 3) | (reference_group >> 2 & 12)
                };

                // Subblock 0 color + alpha endpoints.
                let er0 = endpoint_reference & 3;
                if er0 == 0 {
                    color_endpoint_index = color_endpoint_index
                        .wrapping_add(tables.endpoint_delta[0].decode(&mut reader)?);
                    limit(&mut color_endpoint_index, num_color_endpoints);
                    buffer[x2].color_endpoint_index = color_endpoint_index as u16;
                    alpha0_endpoint_index = alpha0_endpoint_index
                        .wrapping_add(tables.endpoint_delta[1].decode(&mut reader)?);
                    limit(&mut alpha0_endpoint_index, num_alpha_endpoints);
                    buffer[x2].alpha0_endpoint_index = alpha0_endpoint_index as u16;
                } else if er0 == 1 {
                    buffer[x2].color_endpoint_index = color_endpoint_index as u16;
                    buffer[x2].alpha0_endpoint_index = alpha0_endpoint_index as u16;
                } else if er0 == 3 {
                    color_endpoint_index = diagonal_color_endpoint_index;
                    alpha0_endpoint_index = diagonal_alpha0_endpoint_index;
                    buffer[x2].color_endpoint_index = color_endpoint_index as u16;
                    buffer[x2].alpha0_endpoint_index = alpha0_endpoint_index as u16;
                } else {
                    color_endpoint_index = buffer[x2].color_endpoint_index as u32;
                    alpha0_endpoint_index = buffer[x2].alpha0_endpoint_index as u32;
                }
                let er1 = endpoint_reference >> 2;

                let e0 = palettes.color_endpoints[color_endpoint_index as usize];
                let color_selector_index = tables.selector_delta[0].decode(&mut reader)?;
                let alpha0_selector_index = tables.selector_delta[1].decode(&mut reader)?;

                // Subblock 1 color endpoint (advance if er1 nonzero).
                if er1 != 0 {
                    color_endpoint_index = color_endpoint_index
                        .wrapping_add(tables.endpoint_delta[0].decode(&mut reader)?);
                    limit(&mut color_endpoint_index, num_color_endpoints);
                }
                let e1 = palettes.color_endpoints[color_endpoint_index as usize];

                diagonal_color_endpoint_index = buffer[x2 + 1].color_endpoint_index as u32;
                diagonal_alpha0_endpoint_index = buffer[x2 + 1].alpha0_endpoint_index as u32;
                buffer[x2 + 1].color_endpoint_index = color_endpoint_index as u16;
                buffer[x2 + 1].alpha0_endpoint_index = alpha0_endpoint_index as u16;

                if visible {
                    let (block_endpoint, selector_word) =
                        etc_pack_block(e0, e1, er1, color_selector_index as usize, palettes);

                    let flip = (er1 >> 1) ^ 1;
                    let sel_base =
                        (alpha0_selector_index as usize) * 6 + if flip != 0 { 3 } else { 0 };
                    let a0 = u32::from(palettes.alpha_selectors[sel_base]);
                    let a1 = u32::from(palettes.alpha_selectors[sel_base + 1]);
                    let a2 = u32::from(palettes.alpha_selectors[sel_base + 2]);

                    let dw0 = u32::from(palettes.alpha_endpoints[alpha0_endpoint_index as usize])
                        | (a0 << 16);
                    let dw1 = a1 | (a2 << 16);

                    let off = row_base + (x as usize) * block_bytes;
                    output[off..off + 4].copy_from_slice(&dw0.to_le_bytes());
                    output[off + 4..off + 8].copy_from_slice(&dw1.to_le_bytes());
                    output[off + 8..off + 12].copy_from_slice(&block_endpoint.to_le_bytes());
                    output[off + 12..off + 16].copy_from_slice(&selector_word.to_le_bytes());
                }
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Decompress every face and mip of a Unity-fork Crunch file into raw DXT1,
/// DXT5, ETC1, or ETC2_RGBA8 blocks.
///
/// The CRN format is read from the header: DXT1 → BC1, DXT5 (and swizzled
/// variants) → BC3, ETC1/ETC1S → ETC_RGB4, ETC2A → ETC2_RGBA8. ETC1S and
/// ETC2AS are single-subblock variants routed through the DXT1 and DXT5
/// unpackers respectively, matching the Unity reference dispatch. All other
/// formats return [`CrnUnityDecodeError::UnsupportedFormat`]. A 2D texture has one face;
/// a cubemap has six faces in +X, -X, +Y, -Y, +Z, -Z order.
///
/// Each returned [`CrnUnityMipLevel`] owns its standard physical block bytes,
/// laid out row-major in block space.
pub fn decode(input: &[u8]) -> Result<CrnUnityTexture, CrnUnityDecodeError> {
    let header = Header::parse(input)?;

    let (format, has_alpha) = match header.format {
        FMT_DXT1 => (CrnUnityFormat::Bc1, HasAlpha(false)),
        FMT_DXT5 | FMT_DXT5_CCXY | FMT_DXT5_XGXR | FMT_DXT5_XGBR | FMT_DXT5_AGBR | FMT_ETC2AS => {
            (CrnUnityFormat::Bc3, HasAlpha(true))
        }
        FMT_ETC1 | FMT_ETC1S => (CrnUnityFormat::Etc1Rgb4, HasAlpha(false)),
        FMT_ETC2A => (CrnUnityFormat::Etc2Rgba8, HasAlpha(true)),
        _ => return Err(CrnUnityDecodeError::UnsupportedFormat),
    };

    let block_bytes = format.block_bytes();
    let tables = Tables::decode(&header, input, has_alpha)?;
    let palettes = Palettes::decode(&header, input, header.format, has_alpha)?;

    let mut faces = Vec::with_capacity(header.faces as usize);
    for _ in 0..header.faces {
        faces.push(CrnUnityFace {
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
            .ok_or(CrnUnityDecodeError::SizeOverflow)?;

        let level_start = header.level_offsets[level_index] as usize;
        // The reference uses the caller-provided input length (m_data_size) as
        // the final mip's end, not the header's declared data_size.
        let level_end = header
            .level_offsets
            .get(level_index + 1)
            .copied()
            .unwrap_or(input.len() as u32) as usize;
        let level_data = input
            .get(level_start..level_end)
            .ok_or(CrnUnityDecodeError::InvalidHeader)?;

        let mut outputs = Vec::with_capacity(header.faces as usize);
        for _ in 0..header.faces {
            outputs.push(vec![0u8; total]);
        }
        match header.format {
            FMT_DXT1 | FMT_ETC1S => unpack_dxt1(
                level_data,
                blocks_x,
                blocks_y,
                &tables,
                &palettes,
                &mut outputs,
            )?,
            FMT_DXT5 | FMT_DXT5_CCXY | FMT_DXT5_XGXR | FMT_DXT5_XGBR | FMT_DXT5_AGBR
            | FMT_ETC2AS => unpack_dxt5(
                level_data,
                blocks_x,
                blocks_y,
                &tables,
                &palettes,
                &mut outputs,
            )?,
            FMT_ETC1 => unpack_etc1(
                level_data,
                blocks_x,
                blocks_y,
                &tables,
                &palettes,
                &mut outputs,
            )?,
            FMT_ETC2A => unpack_etc2a(
                level_data,
                blocks_x,
                blocks_y,
                &tables,
                &palettes,
                &mut outputs,
            )?,
            _ => unreachable!(),
        }

        for (face, data) in faces.iter_mut().zip(outputs) {
            face.levels.push(CrnUnityMipLevel {
                width,
                height,
                data,
            });
        }
    }

    // `crn_unpacker::unpack_level` clamps zero header dimensions to one.
    Ok(CrnUnityTexture {
        width: core::cmp::max(header.width, 1),
        height: core::cmp::max(header.height, 1),
        format,
        faces,
    })
}
