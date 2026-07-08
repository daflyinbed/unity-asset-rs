//! 自动生成。请勿手动修改。

use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct SkeletonPose {
    pub m_x: Vec<xform>,
}

impl crate::Parse for SkeletonPose {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_x = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { xform::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(SkeletonPose {
            m_x,
        })
    }
}
