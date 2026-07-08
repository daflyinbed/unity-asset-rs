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
        let probe_occlusion_light_index = rdr.read_i32()?;
        let occlusion_mask_channel = rdr.read_i32()?;
        let lightmapping_mask = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let lightmap_bake_mode = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(LightmapBakeMode::parse(rdr, ver)?)
        } else {
            None
        };
        let is_baked = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(LightBakingOutput {
            probe_occlusion_light_index,
            occlusion_mask_channel,
            lightmapping_mask,
            lightmap_bake_mode,
            is_baked,
        })
    }
}
