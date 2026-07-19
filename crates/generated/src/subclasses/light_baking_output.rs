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
