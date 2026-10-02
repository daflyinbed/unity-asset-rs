//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct BuiltinShaderSettings {
    pub m_mode: i32,
    pub m_shader: PPtr,
}

impl crate::Parse for BuiltinShaderSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_mode = rdr.read_i32()?;
        let m_shader = crate::PPtr::parse(rdr, ver)?;

        Ok(BuiltinShaderSettings { m_mode, m_shader })
    }
}

impl crate::ToValue for BuiltinShaderSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Mode".to_string(), self.m_mode.to_value())),
            Some(("m_Shader".to_string(), self.m_shader.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Mode" => (self.m_mode).inspect_value(path, offset, limit),
                "m_Shader" => (self.m_shader).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_mode).inspect_value(path, offset, limit)).map(|v| ("m_Mode".to_string(), v)),
            ((self.m_shader).inspect_value(path, offset, limit))
                .map(|v| ("m_Shader".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
