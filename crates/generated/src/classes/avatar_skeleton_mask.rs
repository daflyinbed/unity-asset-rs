//! 自动生成。请勿手动修改。

use crate::subclasses::avatar_skeleton_mask_element::AvatarSkeletonMaskElement;

#[derive(Debug, Clone)]
pub struct AvatarSkeletonMask {
    pub m_name: String,
    pub elements: Vec<AvatarSkeletonMaskElement>,
}

impl crate::Parse for AvatarSkeletonMask {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let elements = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AvatarSkeletonMaskElement::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(AvatarSkeletonMask {
            m_name,
            elements,
        })
    }
}
