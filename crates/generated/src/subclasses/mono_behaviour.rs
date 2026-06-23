//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct MonoBehaviour {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_editor_hide_flags: u32,
    pub m_script: PPtr,
    pub m_name: String,
}

impl crate::Parse for MonoBehaviour {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_editor_hide_flags = r.read_u32()?;
        let m_script = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(MonoBehaviour {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_enabled,
            m_editor_hide_flags,
            m_script,
            m_name,
        })
    }
}
