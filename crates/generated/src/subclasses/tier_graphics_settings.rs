//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TierGraphicsSettings {
    pub rendering_path: i32,
    pub use_cascaded_shadow_maps: bool,
}

impl crate::Parse for TierGraphicsSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let rendering_path = r.read_i32()?;
        let use_cascaded_shadow_maps = r.read_bool()?;
        r.align(4)?;

        Ok(TierGraphicsSettings {
            rendering_path,
            use_cascaded_shadow_maps,
        })
    }
}
