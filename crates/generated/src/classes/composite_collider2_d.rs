//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::polygon2_d::Polygon2D;
use crate::subclasses::sub_collider::SubCollider;
use crate::subclasses::vector2f::Vector2f;

/// 版本枚举：CompositeCollider2DMUsedByEffectorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CompositeCollider2DMUsedByEffectorVariant {
    Default(bool),
    V2023_1(bool),
}

impl crate::Parse for CompositeCollider2DMUsedByEffectorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CompositeCollider2DMUsedByEffectorVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：CompositeCollider2DMColliderPathsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CompositeCollider2DMColliderPathsVariant {
    Default(Vec<SubCollider>),
    V2017_1(Vec<SubCollider>),
}

impl crate::Parse for CompositeCollider2DMColliderPathsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubCollider::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubCollider::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for CompositeCollider2DMColliderPathsVariant {
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
#[derive(Debug, Clone)]
pub struct CompositeCollider2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_density: f32,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_include_layers: Option<BitField>,
    pub m_used_by_effector: CompositeCollider2DMUsedByEffectorVariant,
    pub m_exclude_layers: Option<BitField>,
    pub m_used_by_composite: Option<bool>,
    pub m_layer_override_priority: Option<i32>,
    pub m_offset: Vector2f,
    pub m_force_send_layers: Option<BitField>,
    pub m_geometry_type: i32,
    pub m_force_receive_layers: Option<BitField>,
    pub m_generation_type: i32,
    pub m_contact_capture_layers: Option<BitField>,
    pub m_edge_radius: f32,
    pub m_callback_layers: Option<BitField>,
    pub m_collider_paths: CompositeCollider2DMColliderPathsVariant,
    pub m_composite_paths: Polygon2D,
    pub m_vertex_distance: f32,
    pub m_composite_operation: Option<i32>,
    pub m_offset_distance: Option<f32>,
    pub m_composite_order: Option<i32>,
    pub m_use_delaunay_mesh: Option<bool>,
    pub m_composite_game_object: Option<PPtr>,
}

impl crate::Parse for CompositeCollider2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_density = None;
        let mut m_material = None;
        let mut m_is_trigger = None;
        let mut m_include_layers = None;
        let mut m_used_by_effector = None;
        let mut m_exclude_layers = None;
        let mut m_used_by_composite = None;
        let mut m_layer_override_priority = None;
        let mut m_offset = None;
        let mut m_force_send_layers = None;
        let mut m_geometry_type = None;
        let mut m_force_receive_layers = None;
        let mut m_generation_type = None;
        let mut m_contact_capture_layers = None;
        let mut m_edge_radius = None;
        let mut m_callback_layers = None;
        let mut m_collider_paths = None;
        let mut m_composite_paths = None;
        let mut m_vertex_distance = None;
        let mut m_composite_operation = None;
        let mut m_offset_distance = None;
        let mut m_composite_order = None;
        let mut m_use_delaunay_mesh = None;
        let mut m_composite_game_object = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_density = Some(rdr.read_f32()?);
        m_material = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_include_layers = Some(BitField::parse(rdr, ver)?);
            m_exclude_layers = Some(BitField::parse(rdr, ver)?);
            m_layer_override_priority = Some(rdr.read_i32()?);
            m_force_send_layers = Some(BitField::parse(rdr, ver)?);
            m_force_receive_layers = Some(BitField::parse(rdr, ver)?);
            m_contact_capture_layers = Some(BitField::parse(rdr, ver)?);
            m_callback_layers = Some(BitField::parse(rdr, ver)?);
        } else {
        }
        m_is_trigger = Some(rdr.read_bool()?);
        m_used_by_effector = Some(CompositeCollider2DMUsedByEffectorVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_composite_operation = Some(rdr.read_i32()?);
            m_composite_order = Some(rdr.read_i32()?);
        } else {
            m_used_by_composite = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_offset = Some(Vector2f::parse(rdr, ver)?);
        m_geometry_type = Some(rdr.read_i32()?);
        m_generation_type = Some(rdr.read_i32()?);
        m_edge_radius = Some(rdr.read_f32()?);
        m_collider_paths = Some(CompositeCollider2DMColliderPathsVariant::parse(rdr, ver)?);
        m_composite_paths = Some(Polygon2D::parse(rdr, ver)?);
        m_vertex_distance = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 1, 18) {
            m_offset_distance = Some(rdr.read_f32()?);
            m_use_delaunay_mesh = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_composite_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_offset_distance = Some(rdr.read_f32()?);
            m_use_delaunay_mesh = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 3) {
            m_offset_distance = Some(rdr.read_f32()?);
        } else {
        }

        Ok(CompositeCollider2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_density: m_density.unwrap(),
            m_material: m_material.unwrap(),
            m_is_trigger: m_is_trigger.unwrap(),
            m_include_layers,
            m_used_by_effector: m_used_by_effector.unwrap(),
            m_exclude_layers,
            m_used_by_composite,
            m_layer_override_priority,
            m_offset: m_offset.unwrap(),
            m_force_send_layers,
            m_geometry_type: m_geometry_type.unwrap(),
            m_force_receive_layers,
            m_generation_type: m_generation_type.unwrap(),
            m_contact_capture_layers,
            m_edge_radius: m_edge_radius.unwrap(),
            m_callback_layers,
            m_collider_paths: m_collider_paths.unwrap(),
            m_composite_paths: m_composite_paths.unwrap(),
            m_vertex_distance: m_vertex_distance.unwrap(),
            m_composite_operation,
            m_offset_distance,
            m_composite_order,
            m_use_delaunay_mesh,
            m_composite_game_object,
        })
    }
}

impl crate::ToValue for CompositeCollider2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Density".to_string(), self.m_density.to_value())),
            Some(("m_Material".to_string(), self.m_material.to_value())),
            Some(("m_IsTrigger".to_string(), self.m_is_trigger.to_value())),
            crate::opt_field("m_IncludeLayers", &self.m_include_layers),
            Some((
                "m_UsedByEffector".to_string(),
                self.m_used_by_effector.to_value(),
            )),
            crate::opt_field("m_ExcludeLayers", &self.m_exclude_layers),
            crate::opt_field("m_UsedByComposite", &self.m_used_by_composite),
            crate::opt_field("m_LayerOverridePriority", &self.m_layer_override_priority),
            Some(("m_Offset".to_string(), self.m_offset.to_value())),
            crate::opt_field("m_ForceSendLayers", &self.m_force_send_layers),
            Some((
                "m_GeometryType".to_string(),
                self.m_geometry_type.to_value(),
            )),
            crate::opt_field("m_ForceReceiveLayers", &self.m_force_receive_layers),
            Some((
                "m_GenerationType".to_string(),
                self.m_generation_type.to_value(),
            )),
            crate::opt_field("m_ContactCaptureLayers", &self.m_contact_capture_layers),
            Some(("m_EdgeRadius".to_string(), self.m_edge_radius.to_value())),
            crate::opt_field("m_CallbackLayers", &self.m_callback_layers),
            Some((
                "m_ColliderPaths".to_string(),
                self.m_collider_paths.to_value(),
            )),
            Some((
                "m_CompositePaths".to_string(),
                self.m_composite_paths.to_value(),
            )),
            Some((
                "m_VertexDistance".to_string(),
                self.m_vertex_distance.to_value(),
            )),
            crate::opt_field("m_CompositeOperation", &self.m_composite_operation),
            crate::opt_field("m_OffsetDistance", &self.m_offset_distance),
            crate::opt_field("m_CompositeOrder", &self.m_composite_order),
            crate::opt_field("m_UseDelaunayMesh", &self.m_use_delaunay_mesh),
            crate::opt_field("m_CompositeGameObject", &self.m_composite_game_object),
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
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Density" => (self.m_density).inspect_value(path, offset, limit),
                "m_Material" => (self.m_material).inspect_value(path, offset, limit),
                "m_IsTrigger" => (self.m_is_trigger).inspect_value(path, offset, limit),
                "m_IncludeLayers" => (self.m_include_layers).inspect_value(path, offset, limit),
                "m_UsedByEffector" => (self.m_used_by_effector).inspect_value(path, offset, limit),
                "m_ExcludeLayers" => (self.m_exclude_layers).inspect_value(path, offset, limit),
                "m_UsedByComposite" => {
                    (self.m_used_by_composite).inspect_value(path, offset, limit)
                }
                "m_LayerOverridePriority" => {
                    (self.m_layer_override_priority).inspect_value(path, offset, limit)
                }
                "m_Offset" => (self.m_offset).inspect_value(path, offset, limit),
                "m_ForceSendLayers" => {
                    (self.m_force_send_layers).inspect_value(path, offset, limit)
                }
                "m_GeometryType" => (self.m_geometry_type).inspect_value(path, offset, limit),
                "m_ForceReceiveLayers" => {
                    (self.m_force_receive_layers).inspect_value(path, offset, limit)
                }
                "m_GenerationType" => (self.m_generation_type).inspect_value(path, offset, limit),
                "m_ContactCaptureLayers" => {
                    (self.m_contact_capture_layers).inspect_value(path, offset, limit)
                }
                "m_EdgeRadius" => (self.m_edge_radius).inspect_value(path, offset, limit),
                "m_CallbackLayers" => (self.m_callback_layers).inspect_value(path, offset, limit),
                "m_ColliderPaths" => (self.m_collider_paths).inspect_value(path, offset, limit),
                "m_CompositePaths" => (self.m_composite_paths).inspect_value(path, offset, limit),
                "m_VertexDistance" => (self.m_vertex_distance).inspect_value(path, offset, limit),
                "m_CompositeOperation" => {
                    (self.m_composite_operation).inspect_value(path, offset, limit)
                }
                "m_OffsetDistance" => (self.m_offset_distance).inspect_value(path, offset, limit),
                "m_CompositeOrder" => (self.m_composite_order).inspect_value(path, offset, limit),
                "m_UseDelaunayMesh" => {
                    (self.m_use_delaunay_mesh).inspect_value(path, offset, limit)
                }
                "m_CompositeGameObject" => {
                    (self.m_composite_game_object).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_density).inspect_value(path, offset, limit))
                .map(|v| ("m_Density".to_string(), v)),
            ((self.m_material).inspect_value(path, offset, limit))
                .map(|v| ("m_Material".to_string(), v)),
            ((self.m_is_trigger).inspect_value(path, offset, limit))
                .map(|v| ("m_IsTrigger".to_string(), v)),
            ((self.m_include_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_IncludeLayers".to_string(), v)),
            ((self.m_used_by_effector).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedByEffector".to_string(), v)),
            ((self.m_exclude_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_ExcludeLayers".to_string(), v)),
            ((self.m_used_by_composite).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedByComposite".to_string(), v)),
            ((self.m_layer_override_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_LayerOverridePriority".to_string(), v)),
            ((self.m_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_Offset".to_string(), v)),
            ((self.m_force_send_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceSendLayers".to_string(), v)),
            ((self.m_geometry_type).inspect_value(path, offset, limit))
                .map(|v| ("m_GeometryType".to_string(), v)),
            ((self.m_force_receive_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceReceiveLayers".to_string(), v)),
            ((self.m_generation_type).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerationType".to_string(), v)),
            ((self.m_contact_capture_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_ContactCaptureLayers".to_string(), v)),
            ((self.m_edge_radius).inspect_value(path, offset, limit))
                .map(|v| ("m_EdgeRadius".to_string(), v)),
            ((self.m_callback_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_CallbackLayers".to_string(), v)),
            ((self.m_collider_paths).inspect_value(path, offset, limit))
                .map(|v| ("m_ColliderPaths".to_string(), v)),
            ((self.m_composite_paths).inspect_value(path, offset, limit))
                .map(|v| ("m_CompositePaths".to_string(), v)),
            ((self.m_vertex_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_VertexDistance".to_string(), v)),
            ((self.m_composite_operation).inspect_value(path, offset, limit))
                .map(|v| ("m_CompositeOperation".to_string(), v)),
            ((self.m_offset_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_OffsetDistance".to_string(), v)),
            ((self.m_composite_order).inspect_value(path, offset, limit))
                .map(|v| ("m_CompositeOrder".to_string(), v)),
            ((self.m_use_delaunay_mesh).inspect_value(path, offset, limit))
                .map(|v| ("m_UseDelaunayMesh".to_string(), v)),
            ((self.m_composite_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_CompositeGameObject".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
