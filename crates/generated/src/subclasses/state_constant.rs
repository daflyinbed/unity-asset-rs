//! 自动生成。请勿手动修改。

use crate::subclasses::leaf_info_constant::LeafInfoConstant;
use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct StateConstant {
    pub m_transition_constant_array: Vec<OffsetPtr>,
    pub m_blend_tree_constant_index_array: Vec<i32>,
    pub m_leaf_info_array: Vec<LeafInfoConstant>,
    pub m_blend_tree_constant_array: Vec<OffsetPtr>,
    pub m_id: u32,
    pub m_tag_id: u32,
    pub m_speed: f32,
    pub m_ik_on_feet: bool,
    pub m_loop: bool,
}

impl crate::Parse for StateConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_transition_constant_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_blend_tree_constant_index_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_leaf_info_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LeafInfoConstant::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_blend_tree_constant_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_id = r.read_u32()?;
        let m_tag_id = r.read_u32()?;
        let m_speed = r.read_f32()?;
        let m_ik_on_feet = r.read_bool()?;
        let m_loop = r.read_bool()?;
        r.align(4)?;

        Ok(StateConstant {
            m_transition_constant_array,
            m_blend_tree_constant_index_array,
            m_leaf_info_array,
            m_blend_tree_constant_array,
            m_id,
            m_tag_id,
            m_speed,
            m_ik_on_feet,
            m_loop,
        })
    }
}
