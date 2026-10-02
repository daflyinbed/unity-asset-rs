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

impl crate::ToValue for MaterialImportOutput {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "currentSettings".to_string(),
                self.current_settings.to_value(),
            )),
            Some(("baked".to_string(), self.baked.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "currentSettings" => (self.current_settings).inspect_value(path, offset, limit),
                "baked" => (self.baked).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.current_settings).inspect_value(path, offset, limit))
                .map(|v| ("currentSettings".to_string(), v)),
            ((self.baked).inspect_value(path, offset, limit)).map(|v| ("baked".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
