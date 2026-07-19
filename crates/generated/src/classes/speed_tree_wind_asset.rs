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
