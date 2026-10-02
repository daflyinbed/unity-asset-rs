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
            let v = ({ Ok(Box::new(PPtr_PrefabInstance::parse(rdr, ver)?)) })?;
            Ok(Self::V2018_2(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for DataTemplateMSourcePrefabVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_2(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for DataTemplateMIsPrefabAssetVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
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
            m_objects = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
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

impl crate::ToValue for DataTemplate {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_LastMergeIdentifier", &self.m_last_merge_identifier),
            crate::opt_field("m_Modification", &self.m_modification),
            crate::opt_field("m_RootGameObject", &self.m_root_game_object),
            crate::opt_field("m_Objects", &self.m_objects),
            crate::opt_field("m_SourcePrefab", &self.m_source_prefab),
            crate::opt_field("m_Father", &self.m_father),
            crate::opt_field("m_IsDataTemplate", &self.m_is_data_template),
            crate::opt_field("m_IsPrefabAsset", &self.m_is_prefab_asset),
            crate::opt_field("m_Name", &self.m_name),
            crate::opt_field("m_IsExploded", &self.m_is_exploded),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_LastMergeIdentifier" => {
                    (self.m_last_merge_identifier).inspect_value(path, offset, limit)
                }
                "m_Modification" => (self.m_modification).inspect_value(path, offset, limit),
                "m_RootGameObject" => (self.m_root_game_object).inspect_value(path, offset, limit),
                "m_Objects" => (self.m_objects).inspect_value(path, offset, limit),
                "m_SourcePrefab" => (self.m_source_prefab).inspect_value(path, offset, limit),
                "m_Father" => (self.m_father).inspect_value(path, offset, limit),
                "m_IsDataTemplate" => (self.m_is_data_template).inspect_value(path, offset, limit),
                "m_IsPrefabAsset" => (self.m_is_prefab_asset).inspect_value(path, offset, limit),
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_IsExploded" => (self.m_is_exploded).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_last_merge_identifier).inspect_value(path, offset, limit))
                .map(|v| ("m_LastMergeIdentifier".to_string(), v)),
            ((self.m_modification).inspect_value(path, offset, limit))
                .map(|v| ("m_Modification".to_string(), v)),
            ((self.m_root_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_RootGameObject".to_string(), v)),
            ((self.m_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_Objects".to_string(), v)),
            ((self.m_source_prefab).inspect_value(path, offset, limit))
                .map(|v| ("m_SourcePrefab".to_string(), v)),
            ((self.m_father).inspect_value(path, offset, limit))
                .map(|v| ("m_Father".to_string(), v)),
            ((self.m_is_data_template).inspect_value(path, offset, limit))
                .map(|v| ("m_IsDataTemplate".to_string(), v)),
            ((self.m_is_prefab_asset).inspect_value(path, offset, limit))
                .map(|v| ("m_IsPrefabAsset".to_string(), v)),
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_is_exploded).inspect_value(path, offset, limit))
                .map(|v| ("m_IsExploded".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
