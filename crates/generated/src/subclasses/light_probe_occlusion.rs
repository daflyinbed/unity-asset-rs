//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LightProbeOcclusion {
    pub m_baked_light_index: Option<Vec<i32>>,
    pub m_probe_occlusion_light_index: Option<Vec<i32>>,
    pub m_occlusion: Vec<f32>,
    pub m_occlusion_mask_channel: Option<Vec<i8>>,
}

impl crate::Parse for LightProbeOcclusion {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_baked_light_index = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_probe_occlusion_light_index = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_occlusion = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_occlusion_mask_channel = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i8() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };

        Ok(LightProbeOcclusion {
            m_baked_light_index,
            m_probe_occlusion_light_index,
            m_occlusion,
            m_occlusion_mask_channel,
        })
    }
}
