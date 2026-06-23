//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PlatformShaderSettings {
    pub use_screen_space_shadows: bool,
}

impl crate::Parse for PlatformShaderSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let use_screen_space_shadows = r.read_bool()?;
        r.align(4)?;

        Ok(PlatformShaderSettings {
            use_screen_space_shadows,
        })
    }
}
