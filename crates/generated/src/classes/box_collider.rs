//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：BoxColliderMMaterialVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BoxColliderMMaterialVariant {
    Default(PPtr),
    V2023_3(PPtr),
}

impl crate::Parse for BoxColliderMMaterialVariant {
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

impl crate::ToValue for BoxColliderMMaterialVariant {
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
pub struct BoxCollider {
    pub m_game_object: PPtr,
    pub m_material: BoxColliderMMaterialVariant,
    pub m_is_trigger: bool,
    pub m_include_layers: Option<BitField>,
    pub m_enabled: bool,
    pub m_exclude_layers: Option<BitField>,
    pub m_size: Vector3f,
    pub m_layer_override_priority: Option<i32>,
    pub m_center: Vector3f,
    pub m_provides_contacts: Option<bool>,
}

impl crate::Parse for BoxCollider {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_material = None;
        let mut m_is_trigger = None;
        let mut m_include_layers = None;
        let mut m_enabled = None;
        let mut m_exclude_layers = None;
        let mut m_size = None;
        let mut m_layer_override_priority = None;
        let mut m_center = None;
        let mut m_provides_contacts = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_material = Some(BoxColliderMMaterialVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_include_layers = Some(BitField::parse(rdr, ver)?);
            m_exclude_layers = Some(BitField::parse(rdr, ver)?);
            m_layer_override_priority = Some(rdr.read_i32()?);
        } else {
        }
        m_is_trigger = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_provides_contacts = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_size = Some(Vector3f::parse(rdr, ver)?);
        m_center = Some(Vector3f::parse(rdr, ver)?);

        Ok(BoxCollider {
            m_game_object: m_game_object.unwrap(),
            m_material: m_material.unwrap(),
            m_is_trigger: m_is_trigger.unwrap(),
            m_include_layers,
            m_enabled: m_enabled.unwrap(),
            m_exclude_layers,
            m_size: m_size.unwrap(),
            m_layer_override_priority,
            m_center: m_center.unwrap(),
            m_provides_contacts,
        })
    }
}

impl crate::ToValue for BoxCollider {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Material".to_string(), self.m_material.to_value())),
            Some(("m_IsTrigger".to_string(), self.m_is_trigger.to_value())),
            crate::opt_field("m_IncludeLayers", &self.m_include_layers),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            crate::opt_field("m_ExcludeLayers", &self.m_exclude_layers),
            Some(("m_Size".to_string(), self.m_size.to_value())),
            crate::opt_field("m_LayerOverridePriority", &self.m_layer_override_priority),
            Some(("m_Center".to_string(), self.m_center.to_value())),
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
                "m_IncludeLayers" => (self.m_include_layers).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_ExcludeLayers" => (self.m_exclude_layers).inspect_value(path, offset, limit),
                "m_Size" => (self.m_size).inspect_value(path, offset, limit),
                "m_LayerOverridePriority" => {
                    (self.m_layer_override_priority).inspect_value(path, offset, limit)
                }
                "m_Center" => (self.m_center).inspect_value(path, offset, limit),
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
            ((self.m_include_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_IncludeLayers".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_exclude_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_ExcludeLayers".to_string(), v)),
            ((self.m_size).inspect_value(path, offset, limit)).map(|v| ("m_Size".to_string(), v)),
            ((self.m_layer_override_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_LayerOverridePriority".to_string(), v)),
            ((self.m_center).inspect_value(path, offset, limit))
                .map(|v| ("m_Center".to_string(), v)),
            ((self.m_provides_contacts).inspect_value(path, offset, limit))
                .map(|v| ("m_ProvidesContacts".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
