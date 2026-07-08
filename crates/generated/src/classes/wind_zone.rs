//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct WindZone {
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_mode = rdr.read_i32()?;
        let m_radius = rdr.read_f32()?;
        let m_wind_main = rdr.read_f32()?;
        let m_wind_turbulence = rdr.read_f32()?;
        let m_wind_pulse_magnitude = rdr.read_f32()?;
        let m_wind_pulse_frequency = rdr.read_f32()?;

        Ok(WindZone {
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
