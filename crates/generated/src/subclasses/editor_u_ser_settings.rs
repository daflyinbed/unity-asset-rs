//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct EditorUSerSettings {
    pub m_vc_user_name: Option<String>,
    pub m_config_values: Option<Vec<(String, String)>>,
    pub m_vc_password: Option<String>,
    pub m_vc_automatic_add: bool,
    pub m_vc_server: Option<String>,
    pub m_vc_debug_com: bool,
    pub m_vc_workspace: Option<String>,
    pub m_vc_debug_cmd: bool,
    pub m_vc_debug_out: bool,
}

impl crate::Parse for EditorUSerSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_vc_user_name = None;
        let mut m_config_values = None;
        let mut m_vc_password = None;
        let mut m_vc_automatic_add = None;
        let mut m_vc_server = None;
        let mut m_vc_debug_com = None;
        let mut m_vc_workspace = None;
        let mut m_vc_debug_cmd = None;
        let mut m_vc_debug_out = None;
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_config_values = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
            m_vc_user_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_vc_password = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_vc_server = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_vc_workspace = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        }
            m_vc_automatic_add = Some(rdr.read_bool()?);
            m_vc_debug_com = Some(rdr.read_bool()?);
            m_vc_debug_cmd = Some(rdr.read_bool()?);
            m_vc_debug_out = Some(rdr.read_bool()?);
            rdr.align(4)?;

        Ok(EditorUSerSettings {
            m_vc_user_name,
            m_config_values,
            m_vc_password,
            m_vc_automatic_add: m_vc_automatic_add.unwrap(),
            m_vc_server,
            m_vc_debug_com: m_vc_debug_com.unwrap(),
            m_vc_workspace,
            m_vc_debug_cmd: m_vc_debug_cmd.unwrap(),
            m_vc_debug_out: m_vc_debug_out.unwrap(),
        })
    }
}
