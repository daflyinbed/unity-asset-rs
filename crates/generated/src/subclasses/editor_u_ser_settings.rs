//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct EditorUSerSettings {
    pub m_object_hide_flags: u32,
    pub m_vc_user_name: String,
    pub m_vc_password: String,
    pub m_vc_server: String,
    pub m_vc_workspace: String,
    pub m_vc_automatic_add: bool,
    pub m_vc_debug_com: bool,
    pub m_vc_debug_cmd: bool,
    pub m_vc_debug_out: bool,
}

impl crate::Parse for EditorUSerSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_vc_user_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_vc_password = r.read_aligned_string()?;
        r.align(4)?;
        let m_vc_server = r.read_aligned_string()?;
        r.align(4)?;
        let m_vc_workspace = r.read_aligned_string()?;
        r.align(4)?;
        let m_vc_automatic_add = r.read_bool()?;
        let m_vc_debug_com = r.read_bool()?;
        let m_vc_debug_cmd = r.read_bool()?;
        let m_vc_debug_out = r.read_bool()?;
        r.align(4)?;

        Ok(EditorUSerSettings {
            m_object_hide_flags,
            m_vc_user_name,
            m_vc_password,
            m_vc_server,
            m_vc_workspace,
            m_vc_automatic_add,
            m_vc_debug_com,
            m_vc_debug_cmd,
            m_vc_debug_out,
        })
    }
}
