//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::prefab_modification::PrefabModification;

#[derive(Debug, Clone)]
pub struct DataTemplate {
    pub m_last_merge_identifier: Option<Guid>,
    pub m_modification: Option<PrefabModification>,
    pub m_root_game_object: Option<PPtr>,
    pub m_objects: Option<Vec<PPtr>>,
    pub m_parent_prefab: Option<PPtr>,
    pub m_source_prefab: Option<PPtr>,
    pub m_father: Option<PPtr>,
    pub m_is_data_template: Option<bool>,
    pub m_is_prefab_parent: Option<bool>,
    pub m_is_prefab_asset: Option<bool>,
    pub m_name: Option<String>,
    pub m_is_exploded: Option<bool>,
}

impl crate::Parse for DataTemplate {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_last_merge_identifier = None;
        let mut m_modification = None;
        let mut m_root_game_object = None;
        let mut m_objects = None;
        let mut m_parent_prefab = None;
        let mut m_source_prefab = None;
        let mut m_father = None;
        let mut m_is_data_template = None;
        let mut m_is_prefab_parent = None;
        let mut m_is_prefab_asset = None;
        let mut m_name = None;
        let mut m_is_exploded = None;
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_root_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_modification = Some(PrefabModification::parse(r, ver)?);
            r.align(4)?;
            m_source_prefab = Some(crate::PPtr::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_modification = Some(PrefabModification::parse(r, ver)?);
            r.align(4)?;
            m_source_prefab = Some(crate::PPtr::parse(r, ver)?);
            m_root_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_is_prefab_asset = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_modification = Some(PrefabModification::parse(r, ver)?);
            r.align(4)?;
            m_parent_prefab = Some(crate::PPtr::parse(r, ver)?);
            m_root_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_is_prefab_parent = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_modification = Some(PrefabModification::parse(r, ver)?);
            r.align(4)?;
            m_parent_prefab = Some(crate::PPtr::parse(r, ver)?);
            m_root_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_is_prefab_parent = Some(r.read_bool()?);
            r.align(4)?;
            m_is_exploded = Some(r.read_bool()?);
            r.align(4)?;
        } else {
            m_last_merge_identifier = Some(Guid::parse(r, ver)?);
            m_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_father = Some(crate::PPtr::parse(r, ver)?);
            m_is_data_template = Some(r.read_bool()?);
            r.align(4)?;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        }

        Ok(DataTemplate {
            m_last_merge_identifier,
            m_modification,
            m_root_game_object,
            m_objects,
            m_parent_prefab,
            m_source_prefab,
            m_father,
            m_is_data_template,
            m_is_prefab_parent,
            m_is_prefab_asset,
            m_name,
            m_is_exploded,
        })
    }
}
