//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedTextureProperty {
    pub m_default_name: String,
    pub m_tex_dim: i32,
}

impl crate::Parse for SerializedTextureProperty {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_default_name = rdr.read_aligned_string()?;
        let m_tex_dim = rdr.read_i32()?;

        Ok(SerializedTextureProperty {
            m_default_name,
            m_tex_dim,
        })
    }
}

impl crate::ToValue for SerializedTextureProperty {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_DefaultName".to_string(), self.m_default_name.to_value())),
            Some(("m_TexDim".to_string(), self.m_tex_dim.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_DefaultName" => (self.m_default_name).inspect_value(path, offset, limit),
                "m_TexDim" => (self.m_tex_dim).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_default_name).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultName".to_string(), v)),
            ((self.m_tex_dim).inspect_value(path, offset, limit))
                .map(|v| ("m_TexDim".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
