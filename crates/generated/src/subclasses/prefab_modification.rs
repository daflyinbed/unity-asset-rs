//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::added_component::AddedComponent;
use crate::subclasses::added_game_object::AddedGameObject;
use crate::subclasses::property_modification::PropertyModification;

/// 版本枚举：PrefabModificationMModificationsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PrefabModificationMModificationsVariant {
    Default(Vec<PropertyModification>),
    V2017_1(Vec<PropertyModification>),
}

impl crate::Parse for PrefabModificationMModificationsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| PropertyModification::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| PropertyModification::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PrefabModificationMModificationsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PrefabModificationMRemovedComponentsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PrefabModificationMRemovedComponentsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
    V2018_3(Vec<PPtr>),
}

impl crate::Parse for PrefabModificationMRemovedComponentsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2018_3(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PrefabModificationMRemovedComponentsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PrefabModification {
    pub m_transform_parent: PPtr,
    pub m_modifications: PrefabModificationMModificationsVariant,
    pub m_removed_components: PrefabModificationMRemovedComponentsVariant,
    pub m_added_game_objects: Option<Vec<AddedGameObject>>,
    pub m_removed_game_objects: Option<Vec<PPtr>>,
    pub m_added_components: Option<Vec<AddedComponent>>,
}

impl crate::Parse for PrefabModification {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_transform_parent = None;
        let mut m_modifications = None;
        let mut m_removed_components = None;
        let mut m_added_game_objects = None;
        let mut m_removed_game_objects = None;
        let mut m_added_components = None;
        m_transform_parent = Some(crate::PPtr::parse(rdr, ver)?);
        m_modifications = Some(PrefabModificationMModificationsVariant::parse(rdr, ver)?);
        m_removed_components = Some(PrefabModificationMRemovedComponentsVariant::parse(
            rdr, ver,
        )?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_removed_game_objects = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_added_game_objects = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AddedGameObject::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_added_components = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AddedComponent::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_added_game_objects = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AddedGameObject::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }

        Ok(PrefabModification {
            m_transform_parent: m_transform_parent.unwrap(),
            m_modifications: m_modifications.unwrap(),
            m_removed_components: m_removed_components.unwrap(),
            m_added_game_objects,
            m_removed_game_objects,
            m_added_components,
        })
    }
}

impl crate::ToValue for PrefabModification {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_TransformParent".to_string(),
                self.m_transform_parent.to_value(),
            )),
            Some((
                "m_Modifications".to_string(),
                self.m_modifications.to_value(),
            )),
            Some((
                "m_RemovedComponents".to_string(),
                self.m_removed_components.to_value(),
            )),
            crate::opt_field("m_AddedGameObjects", &self.m_added_game_objects),
            crate::opt_field("m_RemovedGameObjects", &self.m_removed_game_objects),
            crate::opt_field("m_AddedComponents", &self.m_added_components),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_TransformParent" => (self.m_transform_parent).inspect_value(path, offset, limit),
                "m_Modifications" => (self.m_modifications).inspect_value(path, offset, limit),
                "m_RemovedComponents" => {
                    (self.m_removed_components).inspect_value(path, offset, limit)
                }
                "m_AddedGameObjects" => {
                    (self.m_added_game_objects).inspect_value(path, offset, limit)
                }
                "m_RemovedGameObjects" => {
                    (self.m_removed_game_objects).inspect_value(path, offset, limit)
                }
                "m_AddedComponents" => (self.m_added_components).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_transform_parent).inspect_value(path, offset, limit))
                .map(|v| ("m_TransformParent".to_string(), v)),
            ((self.m_modifications).inspect_value(path, offset, limit))
                .map(|v| ("m_Modifications".to_string(), v)),
            ((self.m_removed_components).inspect_value(path, offset, limit))
                .map(|v| ("m_RemovedComponents".to_string(), v)),
            ((self.m_added_game_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_AddedGameObjects".to_string(), v)),
            ((self.m_removed_game_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_RemovedGameObjects".to_string(), v)),
            ((self.m_added_components).inspect_value(path, offset, limit))
                .map(|v| ("m_AddedComponents".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
