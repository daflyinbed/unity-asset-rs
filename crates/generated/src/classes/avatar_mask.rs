//! 自动生成。请勿手动修改。

use crate::subclasses::transform_mask_element::TransformMaskElement;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct AvatarMask {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_mask: Vec<u32>,
    pub m_elements: Vec<TransformMaskElement>,
}

impl crate::Parse for AvatarMask {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_mask = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_elements = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TransformMaskElement::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(AvatarMask {
            parent,
            m_name,
            m_mask,
            m_elements,
        })
    }
}
