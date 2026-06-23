//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LightBakingOutput {
    pub probe_occlusion_light_index: i32,
    pub shadow_mask_channel: i32,
    pub lightmapping_mask: i32,
}

impl crate::Parse for LightBakingOutput {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let probe_occlusion_light_index = r.read_i32()?;
        let shadow_mask_channel = r.read_i32()?;
        let lightmapping_mask = r.read_i32()?;

        Ok(LightBakingOutput {
            probe_occlusion_light_index,
            shadow_mask_channel,
            lightmapping_mask,
        })
    }
}
