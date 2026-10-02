//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：TransformMLocalScaleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TransformMLocalScaleVariant {
    Default(Box<Vector3f>),
    V2021_2(Box<Vector3f>),
}

impl crate::Parse for TransformMLocalScaleVariant {
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

impl crate::ToValue for TransformMLocalScaleVariant {
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

/// 版本枚举：TransformMChildrenVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TransformMChildrenVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for TransformMChildrenVariant {
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

impl crate::ToValue for TransformMChildrenVariant {
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
pub struct Transform {
    pub m_game_object: PPtr,
    pub m_local_rotation: Quaternionf,
    pub m_local_position: Vector3f,
    pub m_local_scale: TransformMLocalScaleVariant,
    pub m_children: TransformMChildrenVariant,
    pub m_father: PPtr,
}

impl crate::Parse for Transform {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_local_rotation = Quaternionf::parse(rdr, ver)?;
        let m_local_position = Vector3f::parse(rdr, ver)?;
        let m_local_scale = TransformMLocalScaleVariant::parse(rdr, ver)?;
        let m_children = TransformMChildrenVariant::parse(rdr, ver)?;
        let m_father = crate::PPtr::parse(rdr, ver)?;

        Ok(Transform {
            m_game_object,
            m_local_rotation,
            m_local_position,
            m_local_scale,
            m_children,
            m_father,
        })
    }
}

impl crate::ToValue for Transform {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some((
                "m_LocalRotation".to_string(),
                self.m_local_rotation.to_value(),
            )),
            Some((
                "m_LocalPosition".to_string(),
                self.m_local_position.to_value(),
            )),
            Some(("m_LocalScale".to_string(), self.m_local_scale.to_value())),
            Some(("m_Children".to_string(), self.m_children.to_value())),
            Some(("m_Father".to_string(), self.m_father.to_value())),
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
                "m_LocalRotation" => (self.m_local_rotation).inspect_value(path, offset, limit),
                "m_LocalPosition" => (self.m_local_position).inspect_value(path, offset, limit),
                "m_LocalScale" => (self.m_local_scale).inspect_value(path, offset, limit),
                "m_Children" => (self.m_children).inspect_value(path, offset, limit),
                "m_Father" => (self.m_father).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_local_rotation).inspect_value(path, offset, limit))
                .map(|v| ("m_LocalRotation".to_string(), v)),
            ((self.m_local_position).inspect_value(path, offset, limit))
                .map(|v| ("m_LocalPosition".to_string(), v)),
            ((self.m_local_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_LocalScale".to_string(), v)),
            ((self.m_children).inspect_value(path, offset, limit))
                .map(|v| ("m_Children".to_string(), v)),
            ((self.m_father).inspect_value(path, offset, limit))
                .map(|v| ("m_Father".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
