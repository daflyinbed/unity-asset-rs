//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::p_ptr_prefab_instance::PPtr_PrefabInstance;
use crate::subclasses::prefab_modification::PrefabModification;


/// 版本枚举：DataTemplateMSourcePrefabVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum DataTemplateMSourcePrefabVariant {
    Default(PPtr),
    V2018_2(Box<PPtr_PrefabInstance>),
}

impl crate::Parse for DataTemplateMSourcePrefabVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = ( { Ok(Box::new(PPtr_PrefabInstance::parse(rdr, ver)?)) } )?;
            Ok(Self::V2018_2(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：DataTemplateMIsPrefabAssetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum DataTemplateMIsPrefabAssetVariant {
    Default(bool),
    V5_0(bool),
}

impl crate::Parse for DataTemplateMIsPrefabAssetVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct DataTemplate {
    pub m_last_merge_identifier: Option<Guid>,
    pub m_modification: Option<PrefabModification>,
    pub m_root_game_object: Option<PPtr>,
    pub m_objects: Option<Vec<PPtr>>,
    pub m_source_prefab: Option<DataTemplateMSourcePrefabVariant>,
    pub m_father: Option<PPtr>,
    pub m_is_data_template: Option<bool>,
    pub m_is_prefab_asset: Option<DataTemplateMIsPrefabAssetVariant>,
    pub m_name: Option<String>,
    pub m_is_exploded: Option<bool>,
}

impl crate::Parse for DataTemplate {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_last_merge_identifier = None;
        let mut m_modification = None;
        let mut m_root_game_object = None;
        let mut m_objects = None;
        let mut m_source_prefab = None;
        let mut m_father = None;
        let mut m_is_data_template = None;
        let mut m_is_prefab_asset = None;
        let mut m_name = None;
        let mut m_is_exploded = None;
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_root_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_modification = Some(PrefabModification::parse(rdr, ver)?);
            m_source_prefab = Some(DataTemplateMSourcePrefabVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_modification = Some(PrefabModification::parse(rdr, ver)?);
            m_source_prefab = Some(DataTemplateMSourcePrefabVariant::parse(rdr, ver)?);
            m_root_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_is_prefab_asset = Some(DataTemplateMIsPrefabAssetVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_modification = Some(PrefabModification::parse(rdr, ver)?);
            m_source_prefab = Some(DataTemplateMSourcePrefabVariant::parse(rdr, ver)?);
            m_root_game_object = Some(crate::PPtr::parse(rdr, ver)?);
            m_is_prefab_asset = Some(DataTemplateMIsPrefabAssetVariant::parse(rdr, ver)?);
            m_is_exploded = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_last_merge_identifier = Some(Guid::parse(rdr, ver)?);
            m_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_father = Some(crate::PPtr::parse(rdr, ver)?);
            m_is_data_template = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_name = Some(rdr.read_aligned_string()?);
        }

        Ok(DataTemplate {
            m_last_merge_identifier,
            m_modification,
            m_root_game_object,
            m_objects,
            m_source_prefab,
            m_father,
            m_is_data_template,
            m_is_prefab_asset,
            m_name,
            m_is_exploded,
        })
    }
}
