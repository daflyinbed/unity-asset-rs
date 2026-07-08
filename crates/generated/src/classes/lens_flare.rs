//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::color_rgb_af::ColorRgbAf;

#[derive(Debug, Clone)]
pub struct LensFlare {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_flare: PPtr,
    pub m_color: ColorRgbAf,
    pub m_brightness: f32,
    pub m_ignore_layers: BitField,
    pub m_fade_speed: Option<f32>,
    pub m_directional: bool,
}

impl crate::Parse for LensFlare {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_flare = None;
        let mut m_color = None;
        let mut m_brightness = None;
        let mut m_ignore_layers = None;
        let mut m_fade_speed = None;
        let mut m_directional = None;
            m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_enabled = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_flare = Some(crate::PPtr::parse(rdr, ver)?);
            m_color = Some(ColorRgbAf::parse(rdr, ver)?);
            m_brightness = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_fade_speed = Some(rdr.read_f32()?);
        } else {
        }
            m_ignore_layers = Some(BitField::parse(rdr, ver)?);
            m_directional = Some(rdr.read_bool()?);

        Ok(LensFlare {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_flare: m_flare.unwrap(),
            m_color: m_color.unwrap(),
            m_brightness: m_brightness.unwrap(),
            m_ignore_layers: m_ignore_layers.unwrap(),
            m_fade_speed,
            m_directional: m_directional.unwrap(),
        })
    }
}
