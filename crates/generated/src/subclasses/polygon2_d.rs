//! 自动生成。请勿手动修改。

use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct Polygon2D {
    pub m_paths: Vec<Vec<Vector2f>>,
}

impl crate::Parse for Polygon2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_paths = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(Polygon2D {
            m_paths,
        })
    }
}
