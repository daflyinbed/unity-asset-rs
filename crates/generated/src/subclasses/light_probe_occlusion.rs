//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LightProbeOcclusion {
    pub m_baked_light_index: Vec<i32>,
    pub m_occlusion: Vec<f32>,
}

impl crate::Parse for LightProbeOcclusion {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_baked_light_index = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_occlusion = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(LightProbeOcclusion {
            m_baked_light_index,
            m_occlusion,
        })
    }
}
