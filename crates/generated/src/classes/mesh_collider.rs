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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::V2023_3(v))
        } else {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::Default(v))
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V2017_3(v))
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
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
        r: &mut R,
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
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_material = Some(MeshColliderMMaterialVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_include_layers = Some(BitField::parse(r, ver)?);
            m_exclude_layers = Some(BitField::parse(r, ver)?);
            m_layer_override_priority = Some(r.read_i32()?);
        } else {
        }
            m_is_trigger = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_provides_contacts = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }
            m_enabled = Some(r.read_bool()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_smooth_sphere_collisions = Some(r.read_bool()?);
        }
            m_convex = Some(MeshColliderMConvexVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_cooking_options = Some(r.read_i32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_cooking_options = Some(r.read_i32()?);
            r.align(4)?;
            m_skin_width = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_inflate_mesh = Some(r.read_bool()?);
            r.align(4)?;
            m_skin_width = Some(r.read_f32()?);
        } else {
        }
            m_mesh = Some(crate::PPtr::parse(r, ver)?);

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
