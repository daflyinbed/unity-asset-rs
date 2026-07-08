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
