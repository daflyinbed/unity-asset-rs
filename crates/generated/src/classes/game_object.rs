//! 自动生成。请勿手动修改。

use crate::subclasses::component_pair::ComponentPair;

#[derive(Debug, Clone)]
pub struct GameObject {
    pub m_component: Vec<ComponentPair>,
    pub m_layer: u32,
    pub m_name: String,
    pub m_tag: u16,
    pub m_is_active: bool,
}

impl crate::Parse for GameObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_component = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ComponentPair::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_layer = r.read_u32()?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_tag = r.read_u16()?;
        let m_is_active = r.read_bool()?;

        Ok(GameObject {
            m_component,
            m_layer,
            m_name,
            m_tag,
            m_is_active,
        })
    }
}
