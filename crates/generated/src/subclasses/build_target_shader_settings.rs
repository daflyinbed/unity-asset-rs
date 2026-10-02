//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetShaderSettings {
    pub m_build_target: String,
    pub m_chunk_size_in_mb: u32,
    pub m_max_chunks: u32,
    pub m_override_default_settings: bool,
}

impl crate::Parse for BuildTargetShaderSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_chunk_size_in_mb = rdr.read_u32()?;
        let m_max_chunks = rdr.read_u32()?;
        let m_override_default_settings = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(BuildTargetShaderSettings {
            m_build_target,
            m_chunk_size_in_mb,
            m_max_chunks,
            m_override_default_settings,
        })
    }
}

impl crate::ToValue for BuildTargetShaderSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BuildTarget".to_string(), self.m_build_target.to_value())),
            Some((
                "m_ChunkSizeInMB".to_string(),
                self.m_chunk_size_in_mb.to_value(),
            )),
            Some(("m_MaxChunks".to_string(), self.m_max_chunks.to_value())),
            Some((
                "m_OverrideDefaultSettings".to_string(),
                self.m_override_default_settings.to_value(),
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
                "m_ChunkSizeInMB" => (self.m_chunk_size_in_mb).inspect_value(path, offset, limit),
                "m_MaxChunks" => (self.m_max_chunks).inspect_value(path, offset, limit),
                "m_OverrideDefaultSettings" => {
                    (self.m_override_default_settings).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTarget".to_string(), v)),
            ((self.m_chunk_size_in_mb).inspect_value(path, offset, limit))
                .map(|v| ("m_ChunkSizeInMB".to_string(), v)),
            ((self.m_max_chunks).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxChunks".to_string(), v)),
            ((self.m_override_default_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_OverrideDefaultSettings".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
