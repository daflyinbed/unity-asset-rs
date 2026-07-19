//! 自动生成。请勿手动修改。

use crate::subclasses::vector2f::Vector2f;

/// 版本枚举：Polygon2DMPathsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Polygon2DMPathsVariant {
    Default(Vec<Vec<Vector2f>>),
    V2017_1(Vec<Vec<Vector2f>>),
}

impl crate::Parse for Polygon2DMPathsVariant {
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
                                .map(|_| Vector2f::parse(rdr, ver))
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
                                .map(|_| Vector2f::parse(rdr, ver))
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
pub struct Polygon2D {
    pub m_paths: Polygon2DMPathsVariant,
}

impl crate::Parse for Polygon2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_paths = Polygon2DMPathsVariant::parse(rdr, ver)?;

        Ok(Polygon2D { m_paths })
    }
}
