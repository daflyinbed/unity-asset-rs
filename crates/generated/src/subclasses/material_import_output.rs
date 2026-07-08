//! 自动生成。请勿手动修改。

use crate::subclasses::build_target_settings::BuildTargetSettings;

#[derive(Debug, Clone)]
pub struct MaterialImportOutput {
    pub current_settings: BuildTargetSettings,
    pub baked: i32,
}

impl crate::Parse for MaterialImportOutput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let current_settings = BuildTargetSettings::parse(rdr, ver)?;
        rdr.align(4)?;
        let baked = rdr.read_i32()?;

        Ok(MaterialImportOutput {
            current_settings,
            baked,
        })
    }
}
