//! 自动生成。请勿手动修改。

use crate::subclasses::leaf_info_constant::LeafInfoConstant;
use crate::subclasses::offset_ptr::OffsetPtr;


/// 版本枚举：StateConstantMLoopVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StateConstantMLoopVariant {
    Default(bool),
    V4_1(bool),
}

impl crate::Parse for StateConstantMLoopVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct StateConstant {
    pub m_transition_constant_array: Vec<OffsetPtr>,
    pub m_blend_tree_constant_index_array: Vec<i32>,
    pub m_leaf_info_array: Option<Vec<LeafInfoConstant>>,
    pub m_blend_tree_constant_array: Vec<OffsetPtr>,
    pub m_name_id: Option<u32>,
    pub m_id: Option<u32>,
    pub m_path_id: Option<u32>,
    pub m_tag_id: u32,
    pub m_full_path_id: Option<u32>,
    pub m_speed: f32,
    pub m_ik_on_feet: bool,
    pub m_cycle_offset: Option<f32>,
    pub m_speed_param_id: Option<u32>,
    pub m_loop: StateConstantMLoopVariant,
    pub m_mirror_param_id: Option<u32>,
    pub m_cycle_offset_param_id: Option<u32>,
    pub m_mirror: Option<bool>,
    pub m_time_param_id: Option<u32>,
    pub m_write_default_values: Option<bool>,
}

impl crate::Parse for StateConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_transition_constant_array = None;
        let mut m_blend_tree_constant_index_array = None;
        let mut m_leaf_info_array = None;
        let mut m_blend_tree_constant_array = None;
        let mut m_name_id = None;
        let mut m_id = None;
        let mut m_path_id = None;
        let mut m_tag_id = None;
        let mut m_full_path_id = None;
        let mut m_speed = None;
        let mut m_ik_on_feet = None;
        let mut m_cycle_offset = None;
        let mut m_speed_param_id = None;
        let mut m_loop = None;
        let mut m_mirror_param_id = None;
        let mut m_cycle_offset_param_id = None;
        let mut m_mirror = None;
        let mut m_time_param_id = None;
        let mut m_write_default_values = None;
            m_transition_constant_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_blend_tree_constant_index_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
        } else {
            m_leaf_info_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { LeafInfoConstant::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_blend_tree_constant_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_name_id = Some(rdr.read_u32()?);
            m_path_id = Some(rdr.read_u32()?);
            m_full_path_id = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_name_id = Some(rdr.read_u32()?);
            m_path_id = Some(rdr.read_u32()?);
        } else {
            m_id = Some(rdr.read_u32()?);
        }
            m_tag_id = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_speed_param_id = Some(rdr.read_u32()?);
            m_mirror_param_id = Some(rdr.read_u32()?);
            m_cycle_offset_param_id = Some(rdr.read_u32()?);
            m_time_param_id = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_speed_param_id = Some(rdr.read_u32()?);
            m_mirror_param_id = Some(rdr.read_u32()?);
            m_cycle_offset_param_id = Some(rdr.read_u32()?);
        } else {
        }
            m_speed = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_cycle_offset = Some(rdr.read_f32()?);
        } else {
        }
            m_ik_on_feet = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_write_default_values = Some(rdr.read_bool()?);
        } else {
        }
            m_loop = Some(StateConstantMLoopVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_mirror = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(StateConstant {
            m_transition_constant_array: m_transition_constant_array.unwrap(),
            m_blend_tree_constant_index_array: m_blend_tree_constant_index_array.unwrap(),
            m_leaf_info_array,
            m_blend_tree_constant_array: m_blend_tree_constant_array.unwrap(),
            m_name_id,
            m_id,
            m_path_id,
            m_tag_id: m_tag_id.unwrap(),
            m_full_path_id,
            m_speed: m_speed.unwrap(),
            m_ik_on_feet: m_ik_on_feet.unwrap(),
            m_cycle_offset,
            m_speed_param_id,
            m_loop: m_loop.unwrap(),
            m_mirror_param_id,
            m_cycle_offset_param_id,
            m_mirror,
            m_time_param_id,
            m_write_default_values,
        })
    }
}
