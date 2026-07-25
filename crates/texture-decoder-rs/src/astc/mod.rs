//! ASTC LDR block decoding.
//!
//! This module supports the legal 2D LDR footprints from 4×4 through 12×12.
//! Each footprint has a `decode_ldr_*` whole-image function and a
//! `decode_ldr_block_*` single-block function.

mod decoder;
mod tables;

pub use decoder::{
    decode_ldr_4_4, decode_ldr_5_4, decode_ldr_5_5, decode_ldr_6_5, decode_ldr_6_6, decode_ldr_8_5,
    decode_ldr_8_6, decode_ldr_8_8, decode_ldr_10_5, decode_ldr_10_6, decode_ldr_10_8,
    decode_ldr_10_10, decode_ldr_12_10, decode_ldr_12_12, decode_ldr_block_4_4,
    decode_ldr_block_5_4, decode_ldr_block_5_5, decode_ldr_block_6_5, decode_ldr_block_6_6,
    decode_ldr_block_8_5, decode_ldr_block_8_6, decode_ldr_block_8_8, decode_ldr_block_10_5,
    decode_ldr_block_10_6, decode_ldr_block_10_8, decode_ldr_block_10_10, decode_ldr_block_12_10,
    decode_ldr_block_12_12,
};
