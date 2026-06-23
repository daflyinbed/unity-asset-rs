//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::int_point::IntPoint;

#[derive(Debug, Clone)]
pub struct SubCollider {
    pub m_collider: PPtr,
    pub m_collider_paths: Vec<Vec<IntPoint>>,
}

impl crate::Parse for SubCollider {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_collider = crate::PPtr::parse(r, ver)?;
        let m_collider_paths = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { IntPoint::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(SubCollider {
            m_collider,
            m_collider_paths,
        })
    }
}
