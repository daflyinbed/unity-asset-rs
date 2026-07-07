//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::flare_element::FlareElement;

#[derive(Debug, Clone)]
pub struct Flare {
    pub m_name: String,
    pub m_flare_texture: PPtr,
    pub m_texture_layout: i32,
    pub m_elements: Vec<FlareElement>,
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
        let m_elements = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { FlareElement::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
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
