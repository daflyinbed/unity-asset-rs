//! 自动生成。请勿手动修改。

use crate::subclasses::transform_mask_element::TransformMaskElement;

#[derive(Debug, Clone)]
pub struct AvatarMask_1011 {
    pub m_name: String,
    pub m_mask: Vec<u32>,
    pub m_elements: Option<Vec<TransformMaskElement>>,
}

impl crate::Parse for AvatarMask_1011 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_mask = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_elements = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TransformMaskElement::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };

        Ok(AvatarMask_1011 {
            m_name,
            m_mask,
            m_elements,
        })
    }
}
