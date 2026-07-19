//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LightProbeOcclusion {
    pub m_baked_light_index: Option<Vec<i32>>,
    pub m_probe_occlusion_light_index: Option<Vec<i32>>,
    pub m_occlusion: Vec<f32>,
    pub m_shadow_mask_channel: Option<Vec<i8>>,
    pub m_occlusion_mask_channel: Option<Vec<i8>>,
}

impl crate::Parse for LightProbeOcclusion {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_baked_light_index = None;
        let mut m_probe_occlusion_light_index = None;
        let mut m_occlusion = None;
        let mut m_shadow_mask_channel = None;
        let mut m_occlusion_mask_channel = None;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_probe_occlusion_light_index = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
            m_baked_light_index = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }
        m_occlusion = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_occlusion_mask_channel = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_i8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_shadow_mask_channel = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_i8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
        }

        Ok(LightProbeOcclusion {
            m_baked_light_index,
            m_probe_occlusion_light_index,
            m_occlusion: m_occlusion.unwrap(),
            m_shadow_mask_channel,
            m_occlusion_mask_channel,
        })
    }
}
