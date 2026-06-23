//! 自动生成。请勿手动修改。

use crate::subclasses::float4::float4;
use crate::subclasses::human_pose::HumanPose;
use crate::subclasses::offset_ptr::OffsetPtr;
use crate::subclasses::value_delta::ValueDelta;
use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct ClipMuscleConstant {
    pub m_delta_pose: HumanPose,
    pub m_start_x: xform,
    pub m_left_foot_start_x: xform,
    pub m_right_foot_start_x: xform,
    pub m_motion_start_x: xform,
    pub m_motion_stop_x: xform,
    pub m_average_speed: float4,
    pub m_clip: OffsetPtr,
    pub m_start_time: f32,
    pub m_stop_time: f32,
    pub m_orientation_offset_y: f32,
    pub m_level: f32,
    pub m_cycle_offset: f32,
    pub m_average_angular_speed: f32,
    pub m_index_array: Vec<i32>,
    pub m_additional_curve_index_array: Vec<i32>,
    pub m_value_array_delta: Vec<ValueDelta>,
    pub m_mirror: bool,
    pub m_loop_blend: bool,
    pub m_loop_blend_orientation: bool,
    pub m_loop_blend_position_y: bool,
    pub m_loop_blend_position_xz: bool,
    pub m_keep_original_orientation: bool,
    pub m_keep_original_position_y: bool,
    pub m_keep_original_position_xz: bool,
    pub m_height_from_feet: bool,
}

impl crate::Parse for ClipMuscleConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_delta_pose = HumanPose::parse(r, ver)?;
        let m_start_x = xform::parse(r, ver)?;
        let m_left_foot_start_x = xform::parse(r, ver)?;
        let m_right_foot_start_x = xform::parse(r, ver)?;
        let m_motion_start_x = xform::parse(r, ver)?;
        let m_motion_stop_x = xform::parse(r, ver)?;
        let m_average_speed = float4::parse(r, ver)?;
        let m_clip = OffsetPtr::parse(r, ver)?;
        let m_start_time = r.read_f32()?;
        let m_stop_time = r.read_f32()?;
        let m_orientation_offset_y = r.read_f32()?;
        let m_level = r.read_f32()?;
        let m_cycle_offset = r.read_f32()?;
        let m_average_angular_speed = r.read_f32()?;
        let m_index_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_additional_curve_index_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_value_array_delta = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ValueDelta::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_mirror = r.read_bool()?;
        let m_loop_blend = r.read_bool()?;
        let m_loop_blend_orientation = r.read_bool()?;
        let m_loop_blend_position_y = r.read_bool()?;
        let m_loop_blend_position_xz = r.read_bool()?;
        let m_keep_original_orientation = r.read_bool()?;
        let m_keep_original_position_y = r.read_bool()?;
        let m_keep_original_position_xz = r.read_bool()?;
        let m_height_from_feet = r.read_bool()?;
        r.align(4)?;

        Ok(ClipMuscleConstant {
            m_delta_pose,
            m_start_x,
            m_left_foot_start_x,
            m_right_foot_start_x,
            m_motion_start_x,
            m_motion_stop_x,
            m_average_speed,
            m_clip,
            m_start_time,
            m_stop_time,
            m_orientation_offset_y,
            m_level,
            m_cycle_offset,
            m_average_angular_speed,
            m_index_array,
            m_additional_curve_index_array,
            m_value_array_delta,
            m_mirror,
            m_loop_blend,
            m_loop_blend_orientation,
            m_loop_blend_position_y,
            m_loop_blend_position_xz,
            m_keep_original_orientation,
            m_keep_original_position_y,
            m_keep_original_position_xz,
            m_height_from_feet,
        })
    }
}
