//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

/// 版本枚举：MeshColliderMMaterialVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshColliderMMaterialVariant {
    Default(PPtr),
    V2023_3(PPtr),
}

impl crate::Parse for MeshColliderMMaterialVariant {
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

impl crate::ToValue for MeshColliderMMaterialVariant {
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

/// 版本枚举：MeshColliderMConvexVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshColliderMConvexVariant {
    Default(bool),
    V5_5(bool),
    V2017_3(bool),
}

impl crate::Parse for MeshColliderMConvexVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2017_3(v))
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MeshColliderMConvexVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
            Self::V2017_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MeshCollider {
    pub m_game_object: PPtr,
    pub m_material: MeshColliderMMaterialVariant,
    pub m_is_trigger: bool,
    pub m_include_layers: Option<BitField>,
    pub m_enabled: bool,
    pub m_exclude_layers: Option<BitField>,
    pub m_smooth_sphere_collisions: Option<bool>,
    pub m_convex: MeshColliderMConvexVariant,
    pub m_layer_override_priority: Option<i32>,
    pub m_mesh: PPtr,
    pub m_inflate_mesh: Option<bool>,
    pub m_cooking_options: Option<i32>,
    pub m_skin_width: Option<f32>,
    pub m_provides_contacts: Option<bool>,
}

impl crate::Parse for MeshCollider {
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
        let mut m_smooth_sphere_collisions = None;
        let mut m_convex = None;
        let mut m_layer_override_priority = None;
        let mut m_mesh = None;
        let mut m_inflate_mesh = None;
        let mut m_cooking_options = None;
        let mut m_skin_width = None;
        let mut m_provides_contacts = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_material = Some(MeshColliderMMaterialVariant::parse(rdr, ver)?);
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
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_smooth_sphere_collisions = Some(rdr.read_bool()?);
        }
        m_convex = Some(MeshColliderMConvexVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_cooking_options = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_cooking_options = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_skin_width = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_inflate_mesh = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_skin_width = Some(rdr.read_f32()?);
        } else {
        }
        m_mesh = Some(crate::PPtr::parse(rdr, ver)?);

        Ok(MeshCollider {
            m_game_object: m_game_object.unwrap(),
            m_material: m_material.unwrap(),
            m_is_trigger: m_is_trigger.unwrap(),
            m_include_layers,
            m_enabled: m_enabled.unwrap(),
            m_exclude_layers,
            m_smooth_sphere_collisions,
            m_convex: m_convex.unwrap(),
            m_layer_override_priority,
            m_mesh: m_mesh.unwrap(),
            m_inflate_mesh,
            m_cooking_options,
            m_skin_width,
            m_provides_contacts,
        })
    }
}

impl crate::ToValue for MeshCollider {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Material".to_string(), self.m_material.to_value())),
            Some(("m_IsTrigger".to_string(), self.m_is_trigger.to_value())),
            crate::opt_field("m_IncludeLayers", &self.m_include_layers),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            crate::opt_field("m_ExcludeLayers", &self.m_exclude_layers),
            crate::opt_field("m_SmoothSphereCollisions", &self.m_smooth_sphere_collisions),
            Some(("m_Convex".to_string(), self.m_convex.to_value())),
            crate::opt_field("m_LayerOverridePriority", &self.m_layer_override_priority),
            Some(("m_Mesh".to_string(), self.m_mesh.to_value())),
            crate::opt_field("m_InflateMesh", &self.m_inflate_mesh),
            crate::opt_field("m_CookingOptions", &self.m_cooking_options),
            crate::opt_field("m_SkinWidth", &self.m_skin_width),
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
                "m_SmoothSphereCollisions" => {
                    (self.m_smooth_sphere_collisions).inspect_value(path, offset, limit)
                }
                "m_Convex" => (self.m_convex).inspect_value(path, offset, limit),
                "m_LayerOverridePriority" => {
                    (self.m_layer_override_priority).inspect_value(path, offset, limit)
                }
                "m_Mesh" => (self.m_mesh).inspect_value(path, offset, limit),
                "m_InflateMesh" => (self.m_inflate_mesh).inspect_value(path, offset, limit),
                "m_CookingOptions" => (self.m_cooking_options).inspect_value(path, offset, limit),
                "m_SkinWidth" => (self.m_skin_width).inspect_value(path, offset, limit),
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
            ((self.m_smooth_sphere_collisions).inspect_value(path, offset, limit))
                .map(|v| ("m_SmoothSphereCollisions".to_string(), v)),
            ((self.m_convex).inspect_value(path, offset, limit))
                .map(|v| ("m_Convex".to_string(), v)),
            ((self.m_layer_override_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_LayerOverridePriority".to_string(), v)),
            ((self.m_mesh).inspect_value(path, offset, limit)).map(|v| ("m_Mesh".to_string(), v)),
            ((self.m_inflate_mesh).inspect_value(path, offset, limit))
                .map(|v| ("m_InflateMesh".to_string(), v)),
            ((self.m_cooking_options).inspect_value(path, offset, limit))
                .map(|v| ("m_CookingOptions".to_string(), v)),
            ((self.m_skin_width).inspect_value(path, offset, limit))
                .map(|v| ("m_SkinWidth".to_string(), v)),
            ((self.m_provides_contacts).inspect_value(path, offset, limit))
                .map(|v| ("m_ProvidesContacts".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
