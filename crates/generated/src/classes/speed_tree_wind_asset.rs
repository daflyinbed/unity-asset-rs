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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_wind = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(SpeedTreeWind::parse(r, ver)?)
        } else {
            None
        };
        let m_e_version = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_config8 = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(SpeedTreeWindConfig8::parse(r, ver)?)
        } else {
            None
        };
        let m_config9 = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(SpeedTreeWindConfig9::parse(r, ver)?)
        } else {
            None
        };

        Ok(SpeedTreeWindAsset {
            m_name,
            m_wind,
            m_e_version,
            m_config8,
            m_config9,
        })
    }
}
