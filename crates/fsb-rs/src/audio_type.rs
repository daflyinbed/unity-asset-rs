//! FMOD audio codec identifiers (`FmodAudioType` in the C# source).
//!
//! Values match the wire format exactly (the enum is `#[repr(u32)]`). Unknown
//! discriminants round-trip through [`AudioType::Unknown`] so parsing never fails
//! on a codec this crate doesn't recognise yet.

/// An FMOD codec kind, as stored in the FSB5 header.
///
/// Only the container layer is parsed here; actual decoding is out of scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AudioType {
    None = 0,
    Pcm8 = 1,
    Pcm16 = 2,
    Pcm24 = 3,
    Pcm32 = 4,
    PcmFloat = 5,
    Gcadpcm = 6,
    ImaAdpcm = 7,
    Vag = 8,
    Hevag = 9,
    Xma = 10,
    Mpeg = 11,
    Celt = 12,
    At9 = 13,
    Xwma = 14,
    Vorbis = 15,
    Fadpcm = 16,
    Opus = 17,
    /// An audio type not in the known list. Carries the raw discriminant.
    Unknown(u32),
}

impl AudioType {
    pub(crate) fn from_u32(raw: u32) -> Self {
        match raw {
            0 => Self::None,
            1 => Self::Pcm8,
            2 => Self::Pcm16,
            3 => Self::Pcm24,
            4 => Self::Pcm32,
            5 => Self::PcmFloat,
            6 => Self::Gcadpcm,
            7 => Self::ImaAdpcm,
            8 => Self::Vag,
            9 => Self::Hevag,
            10 => Self::Xma,
            11 => Self::Mpeg,
            12 => Self::Celt,
            13 => Self::At9,
            14 => Self::Xwma,
            15 => Self::Vorbis,
            16 => Self::Fadpcm,
            17 => Self::Opus,
            other => Self::Unknown(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_values_round_trip() {
        assert_eq!(AudioType::from_u32(2), AudioType::Pcm16);
        assert_eq!(AudioType::from_u32(15), AudioType::Vorbis);
        assert_eq!(AudioType::from_u32(17), AudioType::Opus);
        assert_eq!(AudioType::from_u32(0), AudioType::None);
    }

    #[test]
    fn unknown_value_preserved() {
        assert_eq!(AudioType::from_u32(99), AudioType::Unknown(99));
    }
}
