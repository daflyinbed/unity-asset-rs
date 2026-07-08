//! 自动生成。请勿手动修改。

use crate::subclasses::float3::float3;
use crate::subclasses::float4::float4;
use crate::subclasses::human_pose::HumanPose;
use crate::subclasses::offset_ptr::OffsetPtr;
use crate::subclasses::value_delta::ValueDelta;
use crate::subclasses::xform::xform;


/// 版本枚举：ClipMuscleConstantMAverageSpeedVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClipMuscleConstantMAverageSpeedVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for ClipMuscleConstantMAverageSpeedVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ( { Ok(Box::new(float3::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_4(v))
        } else {
            let v = ( { Ok(Box::new(float4::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClipMuscleConstant {
    pub m_delta_pose: HumanPose,
    pub m_start_x: xform,
    pub m_left_foot_start_x: xform,
    pub m_stop_x: Option<xform>,
    pub m_right_foot_start_x: xform,
    pub m_motion_start_x: Option<xform>,
    pub m_average_speed: ClipMuscleConstantMAverageSpeedVariant,
    pub m_motion_stop_x: Option<xform>,
    pub m_clip: OffsetPtr,
    pub m_start_time: f32,
    pub m_stop_time: f32,
    pub m_orientation_offset_y: f32,
    pub m_level: f32,
    pub m_cycle_offset: f32,
    pub m_average_angular_speed: f32,
    pub m_index_array: Vec<i32>,
    pub m_value_array_delta: Vec<ValueDelta>,
    pub m_mirror: bool,
    pub m_value_array_reference_pose: Option<Vec<f32>>,
    pub m_additional_curve_index_array: Option<Vec<i32>>,
    pub m_loop_time: Option<bool>,
    pub m_loop_blend: bool,
    pub m_loop_blend_orientation: bool,
    pub m_loop_blend_position_y: bool,
    pub m_loop_blend_position_xz: bool,
    pub m_keep_original_orientation: bool,
    pub m_keep_original_position_y: bool,
    pub m_keep_original_position_xz: bool,
    pub m_start_at_origin: Option<bool>,
    pub m_height_from_feet: bool,
}

impl crate::Parse for ClipMuscleConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_delta_pose = None;
        let mut m_start_x = None;
        let mut m_left_foot_start_x = None;
        let mut m_stop_x = None;
        let mut m_right_foot_start_x = None;
        let mut m_motion_start_x = None;
        let mut m_average_speed = None;
        let mut m_motion_stop_x = None;
        let mut m_clip = None;
        let mut m_start_time = None;
        let mut m_stop_time = None;
        let mut m_orientation_offset_y = None;
        let mut m_level = None;
        let mut m_cycle_offset = None;
        let mut m_average_angular_speed = None;
        let mut m_index_array = None;
        let mut m_value_array_delta = None;
        let mut m_mirror = None;
        let mut m_value_array_reference_pose = None;
        let mut m_additional_curve_index_array = None;
        let mut m_loop_time = None;
        let mut m_loop_blend = None;
        let mut m_loop_blend_orientation = None;
        let mut m_loop_blend_position_y = None;
        let mut m_loop_blend_position_xz = None;
        let mut m_keep_original_orientation = None;
        let mut m_keep_original_position_y = None;
        let mut m_keep_original_position_xz = None;
        let mut m_start_at_origin = None;
        let mut m_height_from_feet = None;
            m_delta_pose = Some(HumanPose::parse(rdr, ver)?);
            m_start_x = Some(xform::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_stop_x = Some(xform::parse(rdr, ver)?);
        } else {
        }
            m_left_foot_start_x = Some(xform::parse(rdr, ver)?);
            m_right_foot_start_x = Some(xform::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_motion_start_x = Some(xform::parse(rdr, ver)?);
            m_motion_stop_x = Some(xform::parse(rdr, ver)?);
        }
            m_average_speed = Some(ClipMuscleConstantMAverageSpeedVariant::parse(rdr, ver)?);
            m_clip = Some(OffsetPtr::parse(rdr, ver)?);
            m_start_time = Some(rdr.read_f32()?);
            m_stop_time = Some(rdr.read_f32()?);
            m_orientation_offset_y = Some(rdr.read_f32()?);
            m_level = Some(rdr.read_f32()?);
            m_cycle_offset = Some(rdr.read_f32()?);
            m_average_angular_speed = Some(rdr.read_f32()?);
            m_index_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
        } else {
            m_additional_curve_index_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_value_array_delta = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ValueDelta::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_value_array_reference_pose = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
        }
            m_mirror = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_loop_time = Some(rdr.read_bool()?);
        } else {
        }
            m_loop_blend = Some(rdr.read_bool()?);
            m_loop_blend_orientation = Some(rdr.read_bool()?);
            m_loop_blend_position_y = Some(rdr.read_bool()?);
            m_loop_blend_position_xz = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_start_at_origin = Some(rdr.read_bool()?);
        } else {
        }
            m_keep_original_orientation = Some(rdr.read_bool()?);
            m_keep_original_position_y = Some(rdr.read_bool()?);
            m_keep_original_position_xz = Some(rdr.read_bool()?);
            m_height_from_feet = Some(rdr.read_bool()?);
            rdr.align(4)?;

        Ok(ClipMuscleConstant {
            m_delta_pose: m_delta_pose.unwrap(),
            m_start_x: m_start_x.unwrap(),
            m_left_foot_start_x: m_left_foot_start_x.unwrap(),
            m_stop_x,
            m_right_foot_start_x: m_right_foot_start_x.unwrap(),
            m_motion_start_x,
            m_average_speed: m_average_speed.unwrap(),
            m_motion_stop_x,
            m_clip: m_clip.unwrap(),
            m_start_time: m_start_time.unwrap(),
            m_stop_time: m_stop_time.unwrap(),
            m_orientation_offset_y: m_orientation_offset_y.unwrap(),
            m_level: m_level.unwrap(),
            m_cycle_offset: m_cycle_offset.unwrap(),
            m_average_angular_speed: m_average_angular_speed.unwrap(),
            m_index_array: m_index_array.unwrap(),
            m_value_array_delta: m_value_array_delta.unwrap(),
            m_mirror: m_mirror.unwrap(),
            m_value_array_reference_pose,
            m_additional_curve_index_array,
            m_loop_time,
            m_loop_blend: m_loop_blend.unwrap(),
            m_loop_blend_orientation: m_loop_blend_orientation.unwrap(),
            m_loop_blend_position_y: m_loop_blend_position_y.unwrap(),
            m_loop_blend_position_xz: m_loop_blend_position_xz.unwrap(),
            m_keep_original_orientation: m_keep_original_orientation.unwrap(),
            m_keep_original_position_y: m_keep_original_position_y.unwrap(),
            m_keep_original_position_xz: m_keep_original_position_xz.unwrap(),
            m_start_at_origin,
            m_height_from_feet: m_height_from_feet.unwrap(),
        })
    }
}
