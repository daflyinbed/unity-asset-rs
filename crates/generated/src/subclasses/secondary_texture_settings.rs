//! 自动生成。请勿手动修改。

use crate::subclasses::texture_importer_platform_settings::TextureImporterPlatformSettings;

#[derive(Debug, Clone)]
pub struct SecondaryTextureSettings {
    pub platform_settings: Vec<TextureImporterPlatformSettings>,
    pub s_rgb: bool,
}

impl crate::Parse for SecondaryTextureSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let platform_settings = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TextureImporterPlatformSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let s_rgb = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(SecondaryTextureSettings {
            platform_settings,
            s_rgb,
        })
    }
}
