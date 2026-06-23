//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct WindZone {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_mode: i32,
    pub m_radius: f32,
    pub m_wind_main: f32,
    pub m_wind_turbulence: f32,
    pub m_wind_pulse_magnitude: f32,
    pub m_wind_pulse_frequency: f32,
}

impl crate::Parse for WindZone {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_mode = r.read_i32()?;
        let m_radius = r.read_f32()?;
        let m_wind_main = r.read_f32()?;
        let m_wind_turbulence = r.read_f32()?;
        let m_wind_pulse_magnitude = r.read_f32()?;
        let m_wind_pulse_frequency = r.read_f32()?;

        Ok(WindZone {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_enabled,
            m_mode,
            m_radius,
            m_wind_main,
            m_wind_turbulence,
            m_wind_pulse_magnitude,
            m_wind_pulse_frequency,
        })
    }
}
