//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Annotation {
    pub m_icon_enabled: bool,
    pub m_gizmo_enabled: bool,
    pub m_class_id: i32,
    pub m_script_class: String,
    pub m_flags: i32,
}

impl crate::Parse for Annotation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_icon_enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let m_gizmo_enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let m_class_id = rdr.read_i32()?;
        let m_script_class = rdr.read_aligned_string()?;
        let m_flags = rdr.read_i32()?;

        Ok(Annotation {
            m_icon_enabled,
            m_gizmo_enabled,
            m_class_id,
            m_script_class,
            m_flags,
        })
    }
}

impl crate::ToValue for Annotation {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_IconEnabled".to_string(), self.m_icon_enabled.to_value())),
            Some((
                "m_GizmoEnabled".to_string(),
                self.m_gizmo_enabled.to_value(),
            )),
            Some(("m_ClassID".to_string(), self.m_class_id.to_value())),
            Some(("m_ScriptClass".to_string(), self.m_script_class.to_value())),
            Some(("m_Flags".to_string(), self.m_flags.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_IconEnabled" => (self.m_icon_enabled).inspect_value(path, offset, limit),
                "m_GizmoEnabled" => (self.m_gizmo_enabled).inspect_value(path, offset, limit),
                "m_ClassID" => (self.m_class_id).inspect_value(path, offset, limit),
                "m_ScriptClass" => (self.m_script_class).inspect_value(path, offset, limit),
                "m_Flags" => (self.m_flags).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_icon_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_IconEnabled".to_string(), v)),
            ((self.m_gizmo_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_GizmoEnabled".to_string(), v)),
            ((self.m_class_id).inspect_value(path, offset, limit))
                .map(|v| ("m_ClassID".to_string(), v)),
            ((self.m_script_class).inspect_value(path, offset, limit))
                .map(|v| ("m_ScriptClass".to_string(), v)),
            ((self.m_flags).inspect_value(path, offset, limit)).map(|v| ("m_Flags".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
