/// A failure while decoding compressed texture data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeError {
    /// The compressed input does not contain all required blocks.
    InputTooShort,
    /// The destination does not contain all required RGBA8 texels.
    OutputTooShort,
    /// A dimension, block count, or byte-count calculation overflowed `usize`.
    SizeOverflow,
    /// The compressed block is not legal for the selected format.
    InvalidBlock,
}
