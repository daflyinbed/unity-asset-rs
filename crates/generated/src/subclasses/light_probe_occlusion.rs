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

impl crate::ToValue for LightProbeOcclusion {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_BakedLightIndex", &self.m_baked_light_index),
            crate::opt_field(
                "m_ProbeOcclusionLightIndex",
                &self.m_probe_occlusion_light_index,
            ),
            Some(("m_Occlusion".to_string(), self.m_occlusion.to_value())),
            crate::opt_field("m_ShadowMaskChannel", &self.m_shadow_mask_channel),
            crate::opt_field("m_OcclusionMaskChannel", &self.m_occlusion_mask_channel),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BakedLightIndex" => {
                    (self.m_baked_light_index).inspect_value(path, offset, limit)
                }
                "m_ProbeOcclusionLightIndex" => {
                    (self.m_probe_occlusion_light_index).inspect_value(path, offset, limit)
                }
                "m_Occlusion" => (self.m_occlusion).inspect_value(path, offset, limit),
                "m_ShadowMaskChannel" => {
                    (self.m_shadow_mask_channel).inspect_value(path, offset, limit)
                }
                "m_OcclusionMaskChannel" => {
                    (self.m_occlusion_mask_channel).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_baked_light_index).inspect_value(path, offset, limit))
                .map(|v| ("m_BakedLightIndex".to_string(), v)),
            ((self.m_probe_occlusion_light_index).inspect_value(path, offset, limit))
                .map(|v| ("m_ProbeOcclusionLightIndex".to_string(), v)),
            ((self.m_occlusion).inspect_value(path, offset, limit))
                .map(|v| ("m_Occlusion".to_string(), v)),
            ((self.m_shadow_mask_channel).inspect_value(path, offset, limit))
                .map(|v| ("m_ShadowMaskChannel".to_string(), v)),
            ((self.m_occlusion_mask_channel).inspect_value(path, offset, limit))
                .map(|v| ("m_OcclusionMaskChannel".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
