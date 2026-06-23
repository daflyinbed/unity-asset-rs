//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::prefab_modification::PrefabModification;
use super::named_object::NamedObject;
use super::object::Object;

#[derive(Debug, Clone)]
pub enum DataTemplateParent {
    NamedObject(NamedObject),
    Object(Object),
}

#[derive(Debug, Clone)]
pub struct DataTemplate {
    pub parent: DataTemplateParent,
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
        let parent = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            DataTemplateParent::NamedObject(NamedObject::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            DataTemplateParent::Object(Object::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_last_merge_identifier = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(Guid::parse(r, ver)?)
        } else {
            None
        };
        let m_modification = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = PrefabModification::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_root_game_object = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_objects = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_parent_prefab = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_source_prefab = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_father = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_is_data_template = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_prefab_parent = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_is_prefab_asset = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_name = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_exploded = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(DataTemplate {
            parent,
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
