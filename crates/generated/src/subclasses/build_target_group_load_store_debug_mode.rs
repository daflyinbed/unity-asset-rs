//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGroupLoadStoreDebugMode {
    pub m_build_target: String,
    pub m_debug_mode_enabled: bool,
    pub m_is_editor_only: bool,
}

impl crate::Parse for BuildTargetGroupLoadStoreDebugMode {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = r.read_aligned_string()?;
        r.align(4)?;
        let m_debug_mode_enabled = r.read_bool()?;
        let m_is_editor_only = r.read_bool()?;
        r.align(4)?;

        Ok(BuildTargetGroupLoadStoreDebugMode {
            m_build_target,
            m_debug_mode_enabled,
            m_is_editor_only,
        })
    }
}
