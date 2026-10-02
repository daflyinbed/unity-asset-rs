//! 自动生成。请勿手动修改。

use crate::subclasses::lightmap_bake_mode::LightmapBakeMode;

#[derive(Debug, Clone)]
pub struct LightBakingOutput {
    pub probe_occlusion_light_index: i32,
    pub occlusion_mask_channel: i32,
    pub lightmapping_mask: Option<i32>,
    pub lightmap_bake_mode: Option<LightmapBakeMode>,
    pub is_baked: Option<bool>,
}

impl crate::Parse for LightBakingOutput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut probe_occlusion_light_index = None;
        let mut occlusion_mask_channel = None;
        let mut lightmapping_mask = None;
        let mut lightmap_bake_mode = None;
        let mut is_baked = None;
        probe_occlusion_light_index = Some(rdr.read_i32()?);
        occlusion_mask_channel = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            lightmap_bake_mode = Some(LightmapBakeMode::parse(rdr, ver)?);
            is_baked = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            lightmapping_mask = Some(rdr.read_i32()?);
        }

        Ok(LightBakingOutput {
            probe_occlusion_light_index: probe_occlusion_light_index.unwrap(),
            occlusion_mask_channel: occlusion_mask_channel.unwrap(),
            lightmapping_mask,
            lightmap_bake_mode,
            is_baked,
        })
    }
}

impl crate::ToValue for LightBakingOutput {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "probeOcclusionLightIndex".to_string(),
                self.probe_occlusion_light_index.to_value(),
            )),
            Some((
                "occlusionMaskChannel".to_string(),
                self.occlusion_mask_channel.to_value(),
            )),
            crate::opt_field("lightmappingMask", &self.lightmapping_mask),
            crate::opt_field("lightmapBakeMode", &self.lightmap_bake_mode),
            crate::opt_field("isBaked", &self.is_baked),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "probeOcclusionLightIndex" => {
                    (self.probe_occlusion_light_index).inspect_value(path, offset, limit)
                }
                "occlusionMaskChannel" => {
                    (self.occlusion_mask_channel).inspect_value(path, offset, limit)
                }
                "lightmappingMask" => (self.lightmapping_mask).inspect_value(path, offset, limit),
                "lightmapBakeMode" => (self.lightmap_bake_mode).inspect_value(path, offset, limit),
                "isBaked" => (self.is_baked).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.probe_occlusion_light_index).inspect_value(path, offset, limit))
                .map(|v| ("probeOcclusionLightIndex".to_string(), v)),
            ((self.occlusion_mask_channel).inspect_value(path, offset, limit))
                .map(|v| ("occlusionMaskChannel".to_string(), v)),
            ((self.lightmapping_mask).inspect_value(path, offset, limit))
                .map(|v| ("lightmappingMask".to_string(), v)),
            ((self.lightmap_bake_mode).inspect_value(path, offset, limit))
                .map(|v| ("lightmapBakeMode".to_string(), v)),
            ((self.is_baked).inspect_value(path, offset, limit))
                .map(|v| ("isBaked".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
