//! 自动生成。请勿手动修改。

use crate::subclasses::axes::Axes;
use crate::subclasses::node::Node;

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub m_node: Vec<Node>,
    pub m_id: Vec<u32>,
    pub m_axes_array: Vec<Axes>,
}

impl crate::Parse for Skeleton {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_node = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Node::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_id = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_axes_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Axes::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(Skeleton {
            m_node,
            m_id,
            m_axes_array,
        })
    }
}
