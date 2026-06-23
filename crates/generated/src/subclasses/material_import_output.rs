//! 自动生成。请勿手动修改。

use crate::subclasses::build_target_settings::BuildTargetSettings;

#[derive(Debug, Clone)]
pub struct MaterialImportOutput {
    pub current_settings: BuildTargetSettings,
    pub baked: i32,
}

impl crate::Parse for MaterialImportOutput {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let current_settings = BuildTargetSettings::parse(r, ver)?;
        r.align(4)?;
        let baked = r.read_i32()?;

        Ok(MaterialImportOutput {
            current_settings,
            baked,
        })
    }
}
