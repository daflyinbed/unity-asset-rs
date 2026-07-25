//! Unity-fork Crunch container decoding to raw DXT1/DXT5/ETC1/ETC2A blocks.
//!
//! Requires the `crn-unity` cargo feature, which pulls in `extern crate alloc`.
//!
//! This is a port of the decompression path in the Unity fork of Crunch
//! (`vendor/unity-crunch/inc/crn_decomp.h`). The Unity fork replaced upstream
//! Crunch's "chunk encoding" bitstream with an incompatible "reference encoding"
//! scheme and added ETC1/ETC2A support, so files produced by the Unity tools are
//! **not** decodable by the [`crate::crn`] module (and vice versa).
//!
//! Like [`crate::crn`], this module reads a compressed Crunch file and writes the
//! **raw physical block bytes** (the layout the GPU or the [`crate::bcn`] /
//! [`crate::etc`] decoders expect), not RGBA8. [`CrnUnityTexture`] contains every
//! face and mip level, with each [`CrnUnityMipLevel`] laid out row-major in block
//! space.
//!
//! Supported formats: DXT1 (BC1, 8 bytes/block), DXT5 (BC3, 16 bytes/block),
//! ETC_RGB4 (ETC1, 8 bytes/block), and ETC2_RGBA8 (16 bytes/block). Cubemaps use
//! +X, -X, +Y, -Y, +Z, -Z face order. Other Crunch formats are rejected. A
//! segmented flag is accepted when every mip remains in-buffer; truly out-of-line
//! level data returns [`CrnUnityDecodeError::UnsupportedSegmented`].

mod decoder;

pub use decoder::{
    CrnUnityDecodeError, CrnUnityFace, CrnUnityFormat, CrnUnityMipLevel, CrnUnityTexture, decode,
};
