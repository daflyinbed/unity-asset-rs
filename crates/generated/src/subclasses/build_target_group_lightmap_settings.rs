//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGroupLightmapSettings {
    pub m_build_target: String,
    pub m_texture_streaming_enabled: bool,
    pub m_texture_streaming_priority: i32,
}

impl crate::Parse for BuildTargetGroupLightmapSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_texture_streaming_enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let m_texture_streaming_priority = rdr.read_i32()?;

        Ok(BuildTargetGroupLightmapSettings {
            m_build_target,
            m_texture_streaming_enabled,
            m_texture_streaming_priority,
        })
    }
}

impl crate::ToValue for BuildTargetGroupLightmapSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BuildTarget".to_string(), self.m_build_target.to_value())),
            Some((
                "m_TextureStreamingEnabled".to_string(),
                self.m_texture_streaming_enabled.to_value(),
            )),
            Some((
                "m_TextureStreamingPriority".to_string(),
                self.m_texture_streaming_priority.to_value(),
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
                "m_TextureStreamingEnabled" => {
                    (self.m_texture_streaming_enabled).inspect_value(path, offset, limit)
                }
                "m_TextureStreamingPriority" => {
                    (self.m_texture_streaming_priority).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTarget".to_string(), v)),
            ((self.m_texture_streaming_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureStreamingEnabled".to_string(), v)),
            ((self.m_texture_streaming_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureStreamingPriority".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
