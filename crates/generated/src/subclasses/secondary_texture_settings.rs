//! 自动生成。请勿手动修改。

use crate::subclasses::texture_importer_platform_settings::TextureImporterPlatformSettings;

#[derive(Debug, Clone)]
pub struct SecondaryTextureSettings {
    pub platform_settings: Vec<TextureImporterPlatformSettings>,
}

impl crate::Parse for SecondaryTextureSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let platform_settings = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TextureImporterPlatformSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(SecondaryTextureSettings {
            platform_settings,
        })
    }
}
