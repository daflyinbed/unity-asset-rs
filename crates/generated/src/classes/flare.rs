//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::flare_element::FlareElement;


/// 版本枚举：FlareMElementsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FlareMElementsVariant {
    Default(Vec<FlareElement>),
    V2017_1(Vec<FlareElement>),
}

impl crate::Parse for FlareMElementsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { FlareElement::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { FlareElement::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Flare {
    pub m_name: String,
    pub m_flare_texture: PPtr,
    pub m_texture_layout: i32,
    pub m_elements: FlareMElementsVariant,
    pub m_use_fog: bool,
}

impl crate::Parse for Flare {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_flare_texture = crate::PPtr::parse(r, ver)?;
        let m_texture_layout = r.read_i32()?;
        let m_elements = FlareMElementsVariant::parse(r, ver)?;
        let m_use_fog = r.read_bool()?;

        Ok(Flare {
            m_name,
            m_flare_texture,
            m_texture_layout,
            m_elements,
            m_use_fog,
        })
    }
}
