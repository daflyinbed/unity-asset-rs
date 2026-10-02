//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGroupLoadStoreDebugMode {
    pub m_build_target: String,
    pub m_debug_mode_enabled: bool,
    pub m_is_editor_only: bool,
}

impl crate::Parse for BuildTargetGroupLoadStoreDebugMode {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_debug_mode_enabled = rdr.read_bool()?;
        let m_is_editor_only = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(BuildTargetGroupLoadStoreDebugMode {
            m_build_target,
            m_debug_mode_enabled,
            m_is_editor_only,
        })
    }
}

impl crate::ToValue for BuildTargetGroupLoadStoreDebugMode {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BuildTarget".to_string(), self.m_build_target.to_value())),
            Some((
                "m_DebugModeEnabled".to_string(),
                self.m_debug_mode_enabled.to_value(),
            )),
            Some((
                "m_IsEditorOnly".to_string(),
                self.m_is_editor_only.to_value(),
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
                "m_BuildTarget" => (self.m_build_target).inspect_value(path, offset, limit),
                "m_DebugModeEnabled" => {
                    (self.m_debug_mode_enabled).inspect_value(path, offset, limit)
                }
                "m_IsEditorOnly" => (self.m_is_editor_only).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTarget".to_string(), v)),
            ((self.m_debug_mode_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_DebugModeEnabled".to_string(), v)),
            ((self.m_is_editor_only).inspect_value(path, offset, limit))
                .map(|v| ("m_IsEditorOnly".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
