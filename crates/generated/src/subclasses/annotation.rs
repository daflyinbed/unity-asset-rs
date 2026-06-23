//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Annotation {
    pub m_icon_enabled: bool,
    pub m_gizmo_enabled: bool,
    pub m_class_id: i32,
    pub m_script_class: String,
    pub m_flags: i32,
}

impl crate::Parse for Annotation {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_icon_enabled = r.read_bool()?;
        r.align(4)?;
        let m_gizmo_enabled = r.read_bool()?;
        r.align(4)?;
        let m_class_id = r.read_i32()?;
        let m_script_class = r.read_aligned_string()?;
        r.align(4)?;
        let m_flags = r.read_i32()?;

        Ok(Annotation {
            m_icon_enabled,
            m_gizmo_enabled,
            m_class_id,
            m_script_class,
            m_flags,
        })
    }
}
