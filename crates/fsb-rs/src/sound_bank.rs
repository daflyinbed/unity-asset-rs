//! The top-level container (`FmodSoundBank` in the C# source).

use crate::header::Header;
use crate::sample::Sample;

/// A parsed FSB5 sound bank: its header and the samples it contains.
#[derive(Debug)]
pub struct SoundBank<'a> {
    /// The bank header (version, codec, etc.).
    pub header: Header,
    /// Samples in declaration order.
    pub samples: Vec<Sample<'a>>,
}
