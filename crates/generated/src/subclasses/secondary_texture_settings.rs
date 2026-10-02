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
        let platform_settings = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| TextureImporterPlatformSettings::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let s_rgb = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(SecondaryTextureSettings {
            platform_settings,
            s_rgb,
        })
    }
}

impl crate::ToValue for SecondaryTextureSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "platformSettings".to_string(),
                self.platform_settings.to_value(),
            )),
            Some(("sRGB".to_string(), self.s_rgb.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "platformSettings" => (self.platform_settings).inspect_value(path, offset, limit),
                "sRGB" => (self.s_rgb).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.platform_settings).inspect_value(path, offset, limit))
                .map(|v| ("platformSettings".to_string(), v)),
            ((self.s_rgb).inspect_value(path, offset, limit)).map(|v| ("sRGB".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
