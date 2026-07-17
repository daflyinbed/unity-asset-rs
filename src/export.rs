//! 导出层门面:纹理 + Sprite → PNG(高级用户用)。
//!
//! 默认入口(阶段 7b 后)是 [`crate::Environment::decode_image`] / [`DecodedImage`]。
//! 本模块提供给需要直接调底层解码原语(`decode_astc` / `decode_bcn` / `decode_crunch` /
//! `decode_etc` / `decode_uncompressed`)的高级用户。
//!
//! 受 `export` feature 门控(默认开)。

pub use crate::sprite_export::{export_sprite, sprite_pixel_bytes, SpriteExportError};
pub use crate::texture_export::{
    decode_astc, decode_bcn, decode_crunch, decode_etc, decode_uncompressed, texture_pixel_bytes,
    write_png, TextureExportError, TextureFormat,
};
