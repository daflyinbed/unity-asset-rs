//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct LensFlare {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_flare: PPtr,
    pub m_color: ColorRgba,
    pub m_brightness: f32,
    pub m_ignore_layers: BitField,
    pub m_directional: bool,
}

impl crate::Parse for LensFlare {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_flare = crate::PPtr::parse(r, ver)?;
        let m_color = ColorRgba::parse(r, ver)?;
        let m_brightness = r.read_f32()?;
        let m_ignore_layers = BitField::parse(r, ver)?;
        let m_directional = r.read_bool()?;

        Ok(LensFlare {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_enabled,
            m_flare,
            m_color,
            m_brightness,
            m_ignore_layers,
            m_directional,
        })
    }
}
