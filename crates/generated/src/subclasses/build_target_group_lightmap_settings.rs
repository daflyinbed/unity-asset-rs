//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGroupLightmapSettings {
    pub m_build_target: String,
    pub m_texture_streaming_enabled: bool,
    pub m_texture_streaming_priority: i32,
}

impl crate::Parse for BuildTargetGroupLightmapSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = r.read_aligned_string()?;
        r.align(4)?;
        let m_texture_streaming_enabled = r.read_bool()?;
        r.align(4)?;
        let m_texture_streaming_priority = r.read_i32()?;

        Ok(BuildTargetGroupLightmapSettings {
            m_build_target,
            m_texture_streaming_enabled,
            m_texture_streaming_priority,
        })
    }
}
