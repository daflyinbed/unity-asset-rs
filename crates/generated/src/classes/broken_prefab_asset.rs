//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::default_asset::DefaultAsset;

#[derive(Debug, Clone)]
pub struct BrokenPrefabAsset {
    pub parent: DefaultAsset,
    pub m_name: String,
    pub m_message: String,
    pub m_error_code: Option<i32>,
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
        let parent = DefaultAsset::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_message = r.read_aligned_string()?;
        r.align(4)?;
        let m_error_code = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_is_warning = r.read_bool()?;
        let m_is_variant = r.read_bool()?;
        let m_is_prefab_file_valid = r.read_bool()?;
        r.align(4)?;
        let m_broken_parent_prefab = crate::PPtr::parse(r, ver)?;

        Ok(BrokenPrefabAsset {
            parent,
            m_name,
            m_message,
            m_error_code,
            m_is_warning,
            m_is_variant,
            m_is_prefab_file_valid,
            m_broken_parent_prefab,
        })
    }
}
