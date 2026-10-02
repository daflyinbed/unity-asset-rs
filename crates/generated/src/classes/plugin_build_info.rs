//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PluginBuildInfo {
    pub m_runtime_plugins: Vec<String>,
    pub m_editor_plugins: Vec<String>,
}

impl crate::Parse for PluginBuildInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_runtime_plugins = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_editor_plugins = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(PluginBuildInfo {
            m_runtime_plugins,
            m_editor_plugins,
        })
    }
}

impl crate::ToValue for PluginBuildInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_RuntimePlugins".to_string(),
                self.m_runtime_plugins.to_value(),
            )),
            Some((
                "m_EditorPlugins".to_string(),
                self.m_editor_plugins.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_RuntimePlugins" => (self.m_runtime_plugins).inspect_value(path, offset, limit),
                "m_EditorPlugins" => (self.m_editor_plugins).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_runtime_plugins).inspect_value(path, offset, limit))
                .map(|v| ("m_RuntimePlugins".to_string(), v)),
            ((self.m_editor_plugins).inspect_value(path, offset, limit))
                .map(|v| ("m_EditorPlugins".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
