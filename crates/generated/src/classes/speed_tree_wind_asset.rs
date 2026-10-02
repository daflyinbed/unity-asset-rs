//! 自动生成。请勿手动修改。

use crate::subclasses::speed_tree_wind::SpeedTreeWind;
use crate::subclasses::speed_tree_wind_config8::SpeedTreeWindConfig8;
use crate::subclasses::speed_tree_wind_config9::SpeedTreeWindConfig9;

#[derive(Debug, Clone)]
pub struct SpeedTreeWindAsset {
    pub m_name: String,
    pub m_wind: Option<SpeedTreeWind>,
    pub m_e_version: Option<i32>,
    pub m_config8: Option<SpeedTreeWindConfig8>,
    pub m_config9: Option<SpeedTreeWindConfig9>,
}

impl crate::Parse for SpeedTreeWindAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_wind = None;
        let mut m_e_version = None;
        let mut m_config8 = None;
        let mut m_config9 = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_e_version = Some(rdr.read_i32()?);
            m_config8 = Some(SpeedTreeWindConfig8::parse(rdr, ver)?);
            m_config9 = Some(SpeedTreeWindConfig9::parse(rdr, ver)?);
        } else {
            m_wind = Some(SpeedTreeWind::parse(rdr, ver)?);
        }

        Ok(SpeedTreeWindAsset {
            m_name: m_name.unwrap(),
            m_wind,
            m_e_version,
            m_config8,
            m_config9,
        })
    }
}

impl crate::ToValue for SpeedTreeWindAsset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_Wind", &self.m_wind),
            crate::opt_field("m_eVersion", &self.m_e_version),
            crate::opt_field("m_Config8", &self.m_config8),
            crate::opt_field("m_Config9", &self.m_config9),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Wind" => (self.m_wind).inspect_value(path, offset, limit),
                "m_eVersion" => (self.m_e_version).inspect_value(path, offset, limit),
                "m_Config8" => (self.m_config8).inspect_value(path, offset, limit),
                "m_Config9" => (self.m_config9).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_wind).inspect_value(path, offset, limit)).map(|v| ("m_Wind".to_string(), v)),
            ((self.m_e_version).inspect_value(path, offset, limit))
                .map(|v| ("m_eVersion".to_string(), v)),
            ((self.m_config8).inspect_value(path, offset, limit))
                .map(|v| ("m_Config8".to_string(), v)),
            ((self.m_config9).inspect_value(path, offset, limit))
                .map(|v| ("m_Config9".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
