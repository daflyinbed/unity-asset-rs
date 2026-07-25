//! Crunch (CRN v1.04) to raw BC1/BC3 block decoding.
//!
//! Requires the `crn` cargo feature, which pulls in `extern crate alloc`.
//!
//! Unlike the [`crate::bcn`] family, which consumes already-uncompressed BC
//! blocks and writes RGBA8, this module reads a compressed Crunch file and
//! writes the **raw physical BC1/BC3 block bytes** (the same layout the GPU or
//! the `bcn` decoders expect), not RGBA8. [`CrnTexture`] contains every face
//! and mip level, with each [`CrnMipLevel`] laid out row-major in block space.
//!
//! DXT1 (BC1) and DXT5-family (BC3) formats are supported. Swizzled DXT5
//! variants return their raw physical BC3 blocks. Cubemaps use +X, -X, +Y,
//! -Y, +Z, -Z face order. Other Crunch formats are rejected. A segmented flag
//! is accepted when every mip remains in-buffer; truly out-of-line level data
//! returns
//! [`CrnDecodeError::UnsupportedSegmented`].

mod decoder;

pub use decoder::{CrnDecodeError, CrnFace, CrnFormat, CrnMipLevel, CrnTexture, decode};
