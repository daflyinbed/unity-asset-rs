//! 自动生成。请勿手动修改。

use crate::subclasses::transform_mask_element::TransformMaskElement;

#[derive(Debug, Clone)]
pub struct AvatarBodyMask {
    pub m_name: String,
    pub m_mask: Vec<u32>,
    pub m_elements: Option<Vec<TransformMaskElement>>,
}

impl crate::Parse for AvatarBodyMask {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_mask = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_elements = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TransformMaskElement::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(AvatarBodyMask {
            m_name,
            m_mask,
            m_elements,
        })
    }
}
