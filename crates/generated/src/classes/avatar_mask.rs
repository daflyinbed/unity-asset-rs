//! 自动生成。请勿手动修改。

use crate::subclasses::transform_mask_element::TransformMaskElement;


/// 版本枚举：AvatarMaskMMaskVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AvatarMaskMMaskVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
}

impl crate::Parse for AvatarMaskMMaskVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AvatarMaskMElementsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AvatarMaskMElementsVariant {
    Default(Vec<TransformMaskElement>),
    V2017_1(Vec<TransformMaskElement>),
}

impl crate::Parse for AvatarMaskMElementsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TransformMaskElement::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TransformMaskElement::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AvatarMask {
    pub m_name: String,
    pub m_mask: AvatarMaskMMaskVariant,
    pub m_elements: Option<AvatarMaskMElementsVariant>,
}

impl crate::Parse for AvatarMask {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_mask = AvatarMaskMMaskVariant::parse(rdr, ver)?;
        let m_elements = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            Some(AvatarMaskMElementsVariant::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(AvatarMask {
            m_name,
            m_mask,
            m_elements,
        })
    }
}
