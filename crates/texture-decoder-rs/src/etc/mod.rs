//! ETC1, ETC2, and EAC block decoding.
//!
//! Functions decode raw ETC/EAC blocks to tightly packed RGBA8.

mod decoder;

pub use decoder::{
    decode_eac_r11, decode_eac_r11_block, decode_eac_rg11, decode_eac_rg11_block,
    decode_eac_signed_r11, decode_eac_signed_r11_block, decode_eac_signed_rg11,
    decode_eac_signed_rg11_block, decode_etc1_rgb8, decode_etc1_rgb8_block, decode_etc2_rgb8,
    decode_etc2_rgb8_block, decode_etc2_rgb8a1, decode_etc2_rgb8a1_block, decode_etc2_rgba8,
    decode_etc2_rgba8_block,
};
