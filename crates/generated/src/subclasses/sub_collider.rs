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
