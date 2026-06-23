//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector3f::Vector3f;
use super::component::Component;
use super::transform::Transform;

#[derive(Debug, Clone)]
pub enum RectTransformParent {
    Component(Component),
    Transform(Transform),
}

#[derive(Debug, Clone)]
pub struct RectTransform {
    pub parent: RectTransformParent,
    pub m_game_object: PPtr,
    pub m_anchor_min: Vector2f,
    pub m_local_rotation: Option<Quaternionf>,
    pub m_anchor_max: Vector2f,
    pub m_local_position: Option<Vector3f>,
    pub m_position: Option<Vector2f>,
    pub m_local_scale: Option<Vector3f>,
    pub m_size_delta: Vector2f,
    pub m_children: Option<Vec<PPtr>>,
    pub m_pivot: Vector2f,
    pub m_father: Option<PPtr>,
    pub m_anchored_position: Option<Vector2f>,
}

impl crate::Parse for RectTransform {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            RectTransformParent::Component(Component::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(4, 6, 0) {
            RectTransformParent::Transform(Transform::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_anchor_min = Vector2f::parse(r, ver)?;
        let m_local_rotation = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            Some(Quaternionf::parse(r, ver)?)
        } else {
            None
        };
        let m_anchor_max = Vector2f::parse(r, ver)?;
        let m_local_position = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_position = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let m_local_scale = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let m_size_delta = Vector2f::parse(r, ver)?;
        let m_children = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_pivot = Vector2f::parse(r, ver)?;
        let m_father = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_anchored_position = if *ver >= crate::UnityVersion::new(4, 6, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };

        Ok(RectTransform {
            parent,
            m_game_object,
            m_anchor_min,
            m_local_rotation,
            m_anchor_max,
            m_local_position,
            m_position,
            m_local_scale,
            m_size_delta,
            m_children,
            m_pivot,
            m_father,
            m_anchored_position,
        })
    }
}
