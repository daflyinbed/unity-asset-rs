//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct EditorSettings {
    pub m_object_hide_flags: u32,
    pub m_external_version_control_support: i32,
    pub m_web_security_emulation_enabled: i32,
    pub m_web_security_emulation_host_url: String,
}

impl crate::Parse for EditorSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_external_version_control_support = r.read_i32()?;
        let m_web_security_emulation_enabled = r.read_i32()?;
        let m_web_security_emulation_host_url = r.read_aligned_string()?;
        r.align(4)?;

        Ok(EditorSettings {
            m_object_hide_flags,
            m_external_version_control_support,
            m_web_security_emulation_enabled,
            m_web_security_emulation_host_url,
        })
    }
}
