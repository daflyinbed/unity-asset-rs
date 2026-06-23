//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::transform_mask_element::TransformMaskElement;

#[derive(Debug, Clone)]
pub struct AvatarMask {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_mask: Vec<u32>,
    pub m_elements: Vec<TransformMaskElement>,
}

impl crate::Parse for AvatarMask {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_mask = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_elements = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TransformMaskElement::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(AvatarMask {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_mask,
            m_elements,
        })
    }
}
