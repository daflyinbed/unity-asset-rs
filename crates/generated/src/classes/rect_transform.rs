//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：RectTransformMLocalScaleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RectTransformMLocalScaleVariant {
    Default(Box<Vector3f>),
    V2021_2(Box<Vector3f>),
}

impl crate::Parse for RectTransformMLocalScaleVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({ Ok(Box::new(Vector3f::parse(rdr, ver)?)) })?;
            rdr.align(4)?;
            Ok(Self::V2021_2(v))
        } else {
            let v = ({ Ok(Box::new(Vector3f::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RectTransformMLocalScaleVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
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

impl crate::ToValue for RectTransformMChildrenVariant {
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
        rdr: &mut R,
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
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 6, 0) {
            m_local_rotation = Some(Quaternionf::parse(rdr, ver)?);
            m_local_position = Some(Vector3f::parse(rdr, ver)?);
            m_local_scale = Some(RectTransformMLocalScaleVariant::parse(rdr, ver)?);
            m_children = Some(RectTransformMChildrenVariant::parse(rdr, ver)?);
            m_father = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
        m_anchor_min = Some(Vector2f::parse(rdr, ver)?);
        m_anchor_max = Some(Vector2f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 6, 0) {
            m_anchored_position = Some(Vector2f::parse(rdr, ver)?);
        } else {
            m_position = Some(Vector2f::parse(rdr, ver)?);
        }
        m_size_delta = Some(Vector2f::parse(rdr, ver)?);
        m_pivot = Some(Vector2f::parse(rdr, ver)?);

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

impl crate::ToValue for RectTransform {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_AnchorMin".to_string(), self.m_anchor_min.to_value())),
            crate::opt_field("m_LocalRotation", &self.m_local_rotation),
            Some(("m_AnchorMax".to_string(), self.m_anchor_max.to_value())),
            crate::opt_field("m_LocalPosition", &self.m_local_position),
            crate::opt_field("m_Position", &self.m_position),
            crate::opt_field("m_LocalScale", &self.m_local_scale),
            Some(("m_SizeDelta".to_string(), self.m_size_delta.to_value())),
            crate::opt_field("m_Children", &self.m_children),
            Some(("m_Pivot".to_string(), self.m_pivot.to_value())),
            crate::opt_field("m_Father", &self.m_father),
            crate::opt_field("m_AnchoredPosition", &self.m_anchored_position),
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
                "m_AnchorMin" => (self.m_anchor_min).inspect_value(path, offset, limit),
                "m_LocalRotation" => (self.m_local_rotation).inspect_value(path, offset, limit),
                "m_AnchorMax" => (self.m_anchor_max).inspect_value(path, offset, limit),
                "m_LocalPosition" => (self.m_local_position).inspect_value(path, offset, limit),
                "m_Position" => (self.m_position).inspect_value(path, offset, limit),
                "m_LocalScale" => (self.m_local_scale).inspect_value(path, offset, limit),
                "m_SizeDelta" => (self.m_size_delta).inspect_value(path, offset, limit),
                "m_Children" => (self.m_children).inspect_value(path, offset, limit),
                "m_Pivot" => (self.m_pivot).inspect_value(path, offset, limit),
                "m_Father" => (self.m_father).inspect_value(path, offset, limit),
                "m_AnchoredPosition" => {
                    (self.m_anchored_position).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_anchor_min).inspect_value(path, offset, limit))
                .map(|v| ("m_AnchorMin".to_string(), v)),
            ((self.m_local_rotation).inspect_value(path, offset, limit))
                .map(|v| ("m_LocalRotation".to_string(), v)),
            ((self.m_anchor_max).inspect_value(path, offset, limit))
                .map(|v| ("m_AnchorMax".to_string(), v)),
            ((self.m_local_position).inspect_value(path, offset, limit))
                .map(|v| ("m_LocalPosition".to_string(), v)),
            ((self.m_position).inspect_value(path, offset, limit))
                .map(|v| ("m_Position".to_string(), v)),
            ((self.m_local_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_LocalScale".to_string(), v)),
            ((self.m_size_delta).inspect_value(path, offset, limit))
                .map(|v| ("m_SizeDelta".to_string(), v)),
            ((self.m_children).inspect_value(path, offset, limit))
                .map(|v| ("m_Children".to_string(), v)),
            ((self.m_pivot).inspect_value(path, offset, limit)).map(|v| ("m_Pivot".to_string(), v)),
            ((self.m_father).inspect_value(path, offset, limit))
                .map(|v| ("m_Father".to_string(), v)),
            ((self.m_anchored_position).inspect_value(path, offset, limit))
                .map(|v| ("m_AnchoredPosition".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
