//! 自动生成。请勿手动修改。

use crate::subclasses::material_build_target_settings::MaterialBuildTargetSettings;

#[derive(Debug, Clone)]
pub struct MaterialImportOutput {
    pub current_settings: MaterialBuildTargetSettings,
    pub baked: i32,
}

impl crate::Parse for MaterialImportOutput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let current_settings = MaterialBuildTargetSettings::parse(rdr, ver)?;
        let baked = rdr.read_i32()?;

        Ok(MaterialImportOutput {
            current_settings,
            baked,
        })
    }
}
