//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2_long::Vector2Long;

/// 版本枚举：SubColliderMColliderPathsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SubColliderMColliderPathsVariant {
    Default(Vec<Vec<Vector2Long>>),
    V2017_1(Vec<Vec<Vector2Long>>),
}

impl crate::Parse for SubColliderMColliderPathsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let n = rdr.read_i32()? as usize;
                            (0..n)
                                .map(|_| Vector2Long::parse(rdr, ver))
                                .collect::<Result<Vec<_>, _>>()
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let n = rdr.read_i32()? as usize;
                            (0..n)
                                .map(|_| Vector2Long::parse(rdr, ver))
                                .collect::<Result<Vec<_>, _>>()
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SubColliderMColliderPathsVariant {
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
pub struct SubCollider {
    pub m_collider: PPtr,
    pub m_collider_paths: SubColliderMColliderPathsVariant,
}

impl crate::Parse for SubCollider {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_collider = crate::PPtr::parse(rdr, ver)?;
        let m_collider_paths = SubColliderMColliderPathsVariant::parse(rdr, ver)?;

        Ok(SubCollider {
            m_collider,
            m_collider_paths,
        })
    }
}

impl crate::ToValue for SubCollider {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Collider".to_string(), self.m_collider.to_value())),
            Some((
                "m_ColliderPaths".to_string(),
                self.m_collider_paths.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Collider" => (self.m_collider).inspect_value(path, offset, limit),
                "m_ColliderPaths" => (self.m_collider_paths).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_collider).inspect_value(path, offset, limit))
                .map(|v| ("m_Collider".to_string(), v)),
            ((self.m_collider_paths).inspect_value(path, offset, limit))
                .map(|v| ("m_ColliderPaths".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
