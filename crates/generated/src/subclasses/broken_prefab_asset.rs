//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct BrokenPrefabAsset {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_name: String,
    pub m_message: String,
    pub m_is_warning: bool,
    pub m_is_variant: bool,
    pub m_is_prefab_file_valid: bool,
    pub m_broken_parent_prefab: PPtr,
}

impl crate::Parse for BrokenPrefabAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_message = r.read_aligned_string()?;
        r.align(4)?;
        let m_is_warning = r.read_bool()?;
        let m_is_variant = r.read_bool()?;
        let m_is_prefab_file_valid = r.read_bool()?;
        r.align(4)?;
        let m_broken_parent_prefab = crate::PPtr::parse(r, ver)?;

        Ok(BrokenPrefabAsset {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_name,
            m_message,
            m_is_warning,
            m_is_variant,
            m_is_prefab_file_valid,
            m_broken_parent_prefab,
        })
    }
}
