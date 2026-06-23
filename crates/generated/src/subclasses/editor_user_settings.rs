//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct EditorUserSettings {
    pub m_object_hide_flags: u32,
    pub m_config_values: Vec<(String, String)>,
    pub m_vc_automatic_add: bool,
    pub m_vc_debug_com: bool,
    pub m_vc_debug_cmd: bool,
    pub m_vc_debug_out: bool,
}

impl crate::Parse for EditorUserSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_config_values = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_vc_automatic_add = r.read_bool()?;
        let m_vc_debug_com = r.read_bool()?;
        let m_vc_debug_cmd = r.read_bool()?;
        let m_vc_debug_out = r.read_bool()?;
        r.align(4)?;

        Ok(EditorUserSettings {
            m_object_hide_flags,
            m_config_values,
            m_vc_automatic_add,
            m_vc_debug_com,
            m_vc_debug_cmd,
            m_vc_debug_out,
        })
    }
}
