//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PluginImportOutput {
    pub plugin_type: Option<i32>,
    pub dll_type: Option<i32>,
    pub scripting_runtime_version: Option<i32>,
}

impl crate::Parse for PluginImportOutput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut plugin_type = None;
        let mut dll_type = None;
        let mut scripting_runtime_version = None;
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            dll_type = Some(rdr.read_i32()?);
            scripting_runtime_version = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            dll_type = Some(rdr.read_i32()?);
        } else {
            plugin_type = Some(rdr.read_i32()?);
        }

        Ok(PluginImportOutput {
            plugin_type,
            dll_type,
            scripting_runtime_version,
        })
    }
}

impl crate::ToValue for PluginImportOutput {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("pluginType", &self.plugin_type),
            crate::opt_field("dllType", &self.dll_type),
            crate::opt_field("scriptingRuntimeVersion", &self.scripting_runtime_version),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "pluginType" => (self.plugin_type).inspect_value(path, offset, limit),
                "dllType" => (self.dll_type).inspect_value(path, offset, limit),
                "scriptingRuntimeVersion" => {
                    (self.scripting_runtime_version).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.plugin_type).inspect_value(path, offset, limit))
                .map(|v| ("pluginType".to_string(), v)),
            ((self.dll_type).inspect_value(path, offset, limit))
                .map(|v| ("dllType".to_string(), v)),
            ((self.scripting_runtime_version).inspect_value(path, offset, limit))
                .map(|v| ("scriptingRuntimeVersion".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
