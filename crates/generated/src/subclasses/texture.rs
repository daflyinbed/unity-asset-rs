//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct Texture {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_image_contents_hash: Hash128,
}

impl crate::Parse for Texture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_image_contents_hash = Hash128::parse(r, ver)?;

        Ok(Texture {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_image_contents_hash,
        })
    }
}
