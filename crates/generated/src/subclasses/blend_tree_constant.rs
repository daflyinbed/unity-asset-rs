//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct BlendTreeConstant {
    pub m_node_array: Vec<OffsetPtr>,
    pub m_blend_event_array_constant: OffsetPtr,
}

impl crate::Parse for BlendTreeConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_node_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_blend_event_array_constant = OffsetPtr::parse(r, ver)?;

        Ok(BlendTreeConstant {
            m_node_array,
            m_blend_event_array_constant,
        })
    }
}
