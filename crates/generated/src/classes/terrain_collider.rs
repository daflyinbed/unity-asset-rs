//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

/// 版本枚举：TerrainColliderMMaterialVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TerrainColliderMMaterialVariant {
    Default(PPtr),
    V2023_3(PPtr),
}

impl crate::Parse for TerrainColliderMMaterialVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V2023_3(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TerrainColliderMMaterialVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TerrainCollider {
    pub m_game_object: PPtr,
    pub m_material: Option<TerrainColliderMMaterialVariant>,
    pub m_is_trigger: Option<bool>,
    pub m_enabled: bool,
    pub m_include_layers: Option<BitField>,
    pub m_terrain_data: PPtr,
    pub m_exclude_layers: Option<BitField>,
    pub m_create_tree_colliders: Option<bool>,
    pub m_enable_tree_colliders: Option<bool>,
    pub m_layer_override_priority: Option<i32>,
    pub m_provides_contacts: Option<bool>,
}

impl crate::Parse for TerrainCollider {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_material = None;
        let mut m_is_trigger = None;
        let mut m_enabled = None;
        let mut m_include_layers = None;
        let mut m_terrain_data = None;
        let mut m_exclude_layers = None;
        let mut m_create_tree_colliders = None;
        let mut m_enable_tree_colliders = None;
        let mut m_layer_override_priority = None;
        let mut m_provides_contacts = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_material = Some(TerrainColliderMMaterialVariant::parse(rdr, ver)?);
            m_include_layers = Some(BitField::parse(rdr, ver)?);
            m_exclude_layers = Some(BitField::parse(rdr, ver)?);
            m_layer_override_priority = Some(rdr.read_i32()?);
            m_provides_contacts = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_material = Some(TerrainColliderMMaterialVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_is_trigger = Some(rdr.read_bool()?);
        } else {
            m_material = Some(TerrainColliderMMaterialVariant::parse(rdr, ver)?);
            m_is_trigger = Some(rdr.read_bool()?);
        }
        m_enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_terrain_data = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enable_tree_colliders = Some(rdr.read_bool()?);
        } else {
            m_create_tree_colliders = Some(rdr.read_bool()?);
        }

        Ok(TerrainCollider {
            m_game_object: m_game_object.unwrap(),
            m_material,
            m_is_trigger,
            m_enabled: m_enabled.unwrap(),
            m_include_layers,
            m_terrain_data: m_terrain_data.unwrap(),
            m_exclude_layers,
            m_create_tree_colliders,
            m_enable_tree_colliders,
            m_layer_override_priority,
            m_provides_contacts,
        })
    }
}

impl crate::ToValue for TerrainCollider {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            crate::opt_field("m_Material", &self.m_material),
            crate::opt_field("m_IsTrigger", &self.m_is_trigger),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            crate::opt_field("m_IncludeLayers", &self.m_include_layers),
            Some(("m_TerrainData".to_string(), self.m_terrain_data.to_value())),
            crate::opt_field("m_ExcludeLayers", &self.m_exclude_layers),
            crate::opt_field("m_CreateTreeColliders", &self.m_create_tree_colliders),
            crate::opt_field("m_EnableTreeColliders", &self.m_enable_tree_colliders),
            crate::opt_field("m_LayerOverridePriority", &self.m_layer_override_priority),
            crate::opt_field("m_ProvidesContacts", &self.m_provides_contacts),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Material" => (self.m_material).inspect_value(path, offset, limit),
                "m_IsTrigger" => (self.m_is_trigger).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_IncludeLayers" => (self.m_include_layers).inspect_value(path, offset, limit),
                "m_TerrainData" => (self.m_terrain_data).inspect_value(path, offset, limit),
                "m_ExcludeLayers" => (self.m_exclude_layers).inspect_value(path, offset, limit),
                "m_CreateTreeColliders" => {
                    (self.m_create_tree_colliders).inspect_value(path, offset, limit)
                }
                "m_EnableTreeColliders" => {
                    (self.m_enable_tree_colliders).inspect_value(path, offset, limit)
                }
                "m_LayerOverridePriority" => {
                    (self.m_layer_override_priority).inspect_value(path, offset, limit)
                }
                "m_ProvidesContacts" => {
                    (self.m_provides_contacts).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_material).inspect_value(path, offset, limit))
                .map(|v| ("m_Material".to_string(), v)),
            ((self.m_is_trigger).inspect_value(path, offset, limit))
                .map(|v| ("m_IsTrigger".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_include_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_IncludeLayers".to_string(), v)),
            ((self.m_terrain_data).inspect_value(path, offset, limit))
                .map(|v| ("m_TerrainData".to_string(), v)),
            ((self.m_exclude_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_ExcludeLayers".to_string(), v)),
            ((self.m_create_tree_colliders).inspect_value(path, offset, limit))
                .map(|v| ("m_CreateTreeColliders".to_string(), v)),
            ((self.m_enable_tree_colliders).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableTreeColliders".to_string(), v)),
            ((self.m_layer_override_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_LayerOverridePriority".to_string(), v)),
            ((self.m_provides_contacts).inspect_value(path, offset, limit))
                .map(|v| ("m_ProvidesContacts".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
