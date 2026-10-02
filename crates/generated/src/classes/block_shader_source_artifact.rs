//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BlockShaderSourceArtifact {
    pub shader_name: String,
    pub shader_source: String,
}

impl crate::Parse for BlockShaderSourceArtifact {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let shader_name = rdr.read_aligned_string()?;
        let shader_source = rdr.read_aligned_string()?;

        Ok(BlockShaderSourceArtifact {
            shader_name,
            shader_source,
        })
    }
}

impl crate::ToValue for BlockShaderSourceArtifact {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("shaderName".to_string(), self.shader_name.to_value())),
            Some(("shaderSource".to_string(), self.shader_source.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "shaderName" => (self.shader_name).inspect_value(path, offset, limit),
                "shaderSource" => (self.shader_source).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.shader_name).inspect_value(path, offset, limit))
                .map(|v| ("shaderName".to_string(), v)),
            ((self.shader_source).inspect_value(path, offset, limit))
                .map(|v| ("shaderSource".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
