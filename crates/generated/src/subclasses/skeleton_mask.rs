//! 自动生成。请勿手动修改。

use crate::subclasses::skeleton_mask_element::SkeletonMaskElement;

#[derive(Debug, Clone)]
pub struct SkeletonMask {
    pub m_data: Vec<SkeletonMaskElement>,
}

impl crate::Parse for SkeletonMask {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_data = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| SkeletonMaskElement::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(SkeletonMask { m_data })
    }
}
