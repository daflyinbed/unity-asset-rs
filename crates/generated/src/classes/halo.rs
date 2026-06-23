//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct Halo {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_color: ColorRgba,
    pub m_size: f32,
}

impl crate::Parse for Halo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_color = ColorRgba::parse(r, ver)?;
        let m_size = r.read_f32()?;

        Ok(Halo {
            parent,
            m_game_object,
            m_enabled,
            m_color,
            m_size,
        })
    }
}
