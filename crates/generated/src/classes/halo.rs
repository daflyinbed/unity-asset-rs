//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba32::ColorRgba32;

#[derive(Debug, Clone)]
pub struct Halo {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_color: ColorRgba32,
    pub m_size: f32,
}

impl crate::Parse for Halo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_color = ColorRgba32::parse(rdr, ver)?;
        let m_size = rdr.read_f32()?;

        Ok(Halo {
            m_game_object,
            m_enabled,
            m_color,
            m_size,
        })
    }
}
