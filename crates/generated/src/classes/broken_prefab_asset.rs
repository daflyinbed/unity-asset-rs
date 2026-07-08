//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct BrokenPrefabAsset {
    pub m_name: String,
    pub m_message: String,
    pub m_is_warning: bool,
    pub m_is_variant: bool,
    pub m_is_prefab_file_valid: bool,
    pub m_broken_parent_prefab: PPtr,
}

impl crate::Parse for BrokenPrefabAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_message = rdr.read_aligned_string()?;
        let m_is_warning = rdr.read_bool()?;
        let m_is_variant = rdr.read_bool()?;
        let m_is_prefab_file_valid = rdr.read_bool()?;
        rdr.align(4)?;
        let m_broken_parent_prefab = crate::PPtr::parse(rdr, ver)?;

        Ok(BrokenPrefabAsset {
            m_name,
            m_message,
            m_is_warning,
            m_is_variant,
            m_is_prefab_file_valid,
            m_broken_parent_prefab,
        })
    }
}
