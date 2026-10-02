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

impl crate::ToValue for WindZone {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Mode".to_string(), self.m_mode.to_value())),
            Some(("m_Radius".to_string(), self.m_radius.to_value())),
            Some(("m_WindMain".to_string(), self.m_wind_main.to_value())),
            Some((
                "m_WindTurbulence".to_string(),
                self.m_wind_turbulence.to_value(),
            )),
            Some((
                "m_WindPulseMagnitude".to_string(),
                self.m_wind_pulse_magnitude.to_value(),
            )),
            Some((
                "m_WindPulseFrequency".to_string(),
                self.m_wind_pulse_frequency.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Mode" => (self.m_mode).inspect_value(path, offset, limit),
                "m_Radius" => (self.m_radius).inspect_value(path, offset, limit),
                "m_WindMain" => (self.m_wind_main).inspect_value(path, offset, limit),
                "m_WindTurbulence" => (self.m_wind_turbulence).inspect_value(path, offset, limit),
                "m_WindPulseMagnitude" => {
                    (self.m_wind_pulse_magnitude).inspect_value(path, offset, limit)
                }
                "m_WindPulseFrequency" => {
                    (self.m_wind_pulse_frequency).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_mode).inspect_value(path, offset, limit)).map(|v| ("m_Mode".to_string(), v)),
            ((self.m_radius).inspect_value(path, offset, limit))
                .map(|v| ("m_Radius".to_string(), v)),
            ((self.m_wind_main).inspect_value(path, offset, limit))
                .map(|v| ("m_WindMain".to_string(), v)),
            ((self.m_wind_turbulence).inspect_value(path, offset, limit))
                .map(|v| ("m_WindTurbulence".to_string(), v)),
            ((self.m_wind_pulse_magnitude).inspect_value(path, offset, limit))
                .map(|v| ("m_WindPulseMagnitude".to_string(), v)),
            ((self.m_wind_pulse_frequency).inspect_value(path, offset, limit))
                .map(|v| ("m_WindPulseFrequency".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
