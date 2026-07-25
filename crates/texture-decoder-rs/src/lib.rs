#![no_std]
//! Small, allocation-free ASTC LDR, ETC2/EAC, and BC1–BC7 decoders.
//!
//! The public API is organized by texture family:
//!
//! - [`astc`] for ASTC LDR.
//! - [`etc`] for ETC1, ETC2, and EAC.
//! - [`bcn`] for BC1–BC7.
//! - `crn` for Crunch containers when the `crn` feature is enabled.
//!
//! All image decoders write tightly packed RGBA8 in row-major order.
//! ETC RGB formats fill alpha with 255. EAC R11 writes `[R, 0, 0, 255]`,
//! and RG11 writes `[R, G, 0, 255]`. Unsigned 11-bit values are normalized
//! to 0 through 255; signed values use SNORM mapping, with zero mapping to
//! 128. The ETC entry points consume raw block payloads rather than PKM or
//! KTX containers. sRGB blocks use the matching RGB entry point because
//! decoding preserves code values and performs no color-space conversion.
//!
//! The BC1–BC7 decoders follow the same conventions. BC1 outputs RGBA with the
//! three-color selector 3 decoded as transparent black; BC2 expands 4-bit alpha
//! to `value * 17`; BC3 uses the 8-entry alpha interpolation. BC4 writes
//! `[R, 0, 0, 255]` and BC5 writes `[R, G, 0, 255]`. BC6H writes `[R, G, B, 255]`
//! using a clamped `f16` → 8-bit mapping (`<= 0 -> 0`, `>= 1 -> 255`, else
//! `floor(value * 255 + 0.5)`); SF16 negatives clip to 0 and HDR values above 1
//! clip to 255. BC7 preserves the decoded RGBA. No format performs sRGB,
//! gamma, exposure, or tone mapping, and BC4/BC5 SNORM is not implemented.
//!
//! With the optional `crn` cargo feature enabled, the `crn` module decompresses
//! every face and mip level of Crunch (CRN v1.04) 2D textures and cubemaps into
//! raw BC1/BC3 blocks. Unlike the other families it requires `alloc` for palette
//! and output storage. `crn::decode` returns a `crn::CrnTexture` organized as
//! `texture.faces[face].levels[level]`; cubemap faces are ordered `+X, -X, +Y,
//! -Y, +Z, -Z`. The output is physical BC block data, not RGBA8. Truly segmented
//! files with out-of-line level payloads are not supported.
//!
//! With the optional `crn-unity` cargo feature, the `crn_unity` module ports the
//! Unity fork of Crunch. The Unity fork replaced upstream Crunch's "chunk
//! encoding" bitstream with an incompatible "reference encoding" scheme and
//! added ETC1/ETC2A support, so its files are not readable by the `crn` module.
//! `crn_unity::decode` supports DXT1 (BC1), DXT5 (BC3), ETC_RGB4 (ETC1), and
//! ETC2_RGBA8, and returns raw physical blocks organized like `crn::CrnTexture`.

#[cfg(any(feature = "crn", feature = "crn-unity"))]
extern crate alloc;

mod error;

pub mod astc;
pub mod bcn;
#[cfg(feature = "crn")]
pub mod crn;
#[cfg(feature = "crn-unity")]
pub mod crn_unity;
pub mod etc;

pub use error::DecodeError;
