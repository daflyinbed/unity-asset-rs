//! BC1–BC7 block decoding.
//!
//! Functions decode raw BCn blocks to tightly packed RGBA8.

mod decoder;
mod tables;

pub use decoder::{
    decode_bc1, decode_bc1_block, decode_bc2, decode_bc2_block, decode_bc3, decode_bc3_block,
    decode_bc4, decode_bc4_block, decode_bc5, decode_bc5_block, decode_bc6h_sf16,
    decode_bc6h_sf16_block, decode_bc6h_uf16, decode_bc6h_uf16_block, decode_bc7, decode_bc7_block,
};
