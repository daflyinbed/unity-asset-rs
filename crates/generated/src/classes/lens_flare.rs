//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgba::ColorRgba;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct LensFlare {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_flare: PPtr,
    pub m_color: ColorRgba,
    pub m_brightness: f32,
    pub m_ignore_layers: BitField,
    pub m_fade_speed: Option<f32>,
    pub m_directional: bool,
}

impl crate::Parse for LensFlare {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_flare = crate::PPtr::parse(r, ver)?;
        let m_color = ColorRgba::parse(r, ver)?;
        let m_brightness = r.read_f32()?;
        let m_ignore_layers = BitField::parse(r, ver)?;
        let m_fade_speed = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_directional = r.read_bool()?;

        Ok(LensFlare {
            parent,
            m_game_object,
            m_enabled,
            m_flare,
            m_color,
            m_brightness,
            m_ignore_layers,
            m_fade_speed,
            m_directional,
        })
    }
}
