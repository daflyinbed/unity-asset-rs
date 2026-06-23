//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ExternalForcesModule {
    pub enabled: bool,
    pub multiplier: f32,
}

impl crate::Parse for ExternalForcesModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let multiplier = r.read_f32()?;

        Ok(ExternalForcesModule {
            enabled,
            multiplier,
        })
    }
}
