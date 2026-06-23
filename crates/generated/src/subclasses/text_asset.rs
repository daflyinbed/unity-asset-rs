//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TextAsset {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_script: String,
    pub m_path_name: String,
}

impl crate::Parse for TextAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_script = r.read_aligned_string()?;
        r.align(4)?;
        let m_path_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(TextAsset {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_script,
            m_path_name,
        })
    }
}
