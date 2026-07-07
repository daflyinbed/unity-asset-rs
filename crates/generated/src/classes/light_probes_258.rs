//! 自动生成。请勿手动修改。

use crate::subclasses::light_probe_data::LightProbeData;
use crate::subclasses::light_probe_occlusion::LightProbeOcclusion;
use crate::subclasses::spherical_harmonics_l2::SphericalHarmonicsL2;

#[derive(Debug, Clone)]
pub struct LightProbes_258 {
    pub m_name: String,
    pub m_data: LightProbeData,
    pub m_baked_coefficients: Vec<SphericalHarmonicsL2>,
    pub m_baked_light_occlusion: Option<Vec<LightProbeOcclusion>>,
    pub m_has_been_edited: Option<bool>,
}

impl crate::Parse for LightProbes_258 {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_data = LightProbeData::parse(r, ver)?;
        r.align(4)?;
        let m_baked_coefficients = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SphericalHarmonicsL2::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_baked_light_occlusion = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightProbeOcclusion::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_has_been_edited = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(LightProbes_258 {
            m_name,
            m_data,
            m_baked_coefficients,
            m_baked_light_occlusion,
            m_has_been_edited,
        })
    }
}
