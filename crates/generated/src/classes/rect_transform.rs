//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：RectTransformMLocalScaleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RectTransformMLocalScaleVariant {
    Default(Vector3f),
    V2021_2(Vector3f),
}

impl crate::Parse for RectTransformMLocalScaleVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = Vector3f::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V2021_2(v))
        } else {
            let v = Vector3f::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：RectTransformMChildrenVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RectTransformMChildrenVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for RectTransformMChildrenVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct RectTransform {
    pub m_game_object: PPtr,
    pub m_anchor_min: Vector2f,
    pub m_local_rotation: Option<Quaternionf>,
    pub m_anchor_max: Vector2f,
    pub m_local_position: Option<Vector3f>,
    pub m_position: Option<Vector2f>,
    pub m_local_scale: Option<RectTransformMLocalScaleVariant>,
    pub m_size_delta: Vector2f,
    pub m_children: Option<RectTransformMChildrenVariant>,
    pub m_pivot: Vector2f,
    pub m_father: Option<PPtr>,
    pub m_anchored_position: Option<Vector2f>,
}

impl crate::Parse for RectTransform {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_anchor_min = None;
        let mut m_local_rotation = None;
        let mut m_anchor_max = None;
        let mut m_local_position = None;
        let mut m_position = None;
        let mut m_local_scale = None;
        let mut m_size_delta = None;
        let mut m_children = None;
        let mut m_pivot = None;
        let mut m_father = None;
        let mut m_anchored_position = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(4, 6, 0) {
            m_local_rotation = Some(Quaternionf::parse(r, ver)?);
            m_local_position = Some(Vector3f::parse(r, ver)?);
            m_local_scale = Some(RectTransformMLocalScaleVariant::parse(r, ver)?);
            m_children = Some(RectTransformMChildrenVariant::parse(r, ver)?);
            m_father = Some(crate::PPtr::parse(r, ver)?);
        } else {
        }
            m_anchor_min = Some(Vector2f::parse(r, ver)?);
            m_anchor_max = Some(Vector2f::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(4, 6, 0) {
            m_anchored_position = Some(Vector2f::parse(r, ver)?);
        } else {
            m_position = Some(Vector2f::parse(r, ver)?);
        }
            m_size_delta = Some(Vector2f::parse(r, ver)?);
            m_pivot = Some(Vector2f::parse(r, ver)?);

        Ok(RectTransform {
            m_game_object: m_game_object.unwrap(),
            m_anchor_min: m_anchor_min.unwrap(),
            m_local_rotation,
            m_anchor_max: m_anchor_max.unwrap(),
            m_local_position,
            m_position,
            m_local_scale,
            m_size_delta: m_size_delta.unwrap(),
            m_children,
            m_pivot: m_pivot.unwrap(),
            m_father,
            m_anchored_position,
        })
    }
}
