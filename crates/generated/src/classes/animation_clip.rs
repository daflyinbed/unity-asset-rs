//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;
use crate::subclasses::animation_clip_binding_constant::AnimationClipBindingConstant;
use crate::subclasses::animation_event::AnimationEvent;
use crate::subclasses::clip_muscle_constant::ClipMuscleConstant;
use crate::subclasses::compressed_animation_curve::CompressedAnimationCurve;
use crate::subclasses::float_curve::FloatCurve;
use crate::subclasses::p_ptr_curve::PPtrCurve;
use crate::subclasses::quaternion_curve::QuaternionCurve;
use crate::subclasses::vector3_curve::Vector3Curve;


/// 版本枚举：AnimationClipMCompressedVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipMCompressedVariant {
    Default(bool),
    V4_3(bool),
}

impl crate::Parse for AnimationClipMCompressedVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimationClipMClipBindingConstantVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipMClipBindingConstantVariant {
    Default(Box<AnimationClipBindingConstant>),
    V5_6(Box<AnimationClipBindingConstant>),
}

impl crate::Parse for AnimationClipMClipBindingConstantVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { Ok(Box::new(AnimationClipBindingConstant::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V5_6(v))
        } else {
            let v = ( { Ok(Box::new(AnimationClipBindingConstant::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimationClip {
    pub m_name: String,
    pub m_compressed: AnimationClipMCompressedVariant,
    pub m_animation_type: Option<i32>,
    pub m_legacy: Option<bool>,
    pub m_rotation_curves: Vec<QuaternionCurve>,
    pub m_compressed_rotation_curves: Vec<CompressedAnimationCurve>,
    pub m_use_high_quality_curve: Option<bool>,
    pub m_position_curves: Vec<Vector3Curve>,
    pub m_scale_curves: Vec<Vector3Curve>,
    pub m_float_curves: Vec<FloatCurve>,
    pub m_euler_curves: Option<Vec<Vector3Curve>>,
    pub m_sample_rate: f32,
    pub m_wrap_mode: i32,
    pub m_bounds: Aabb,
    pub m_p_ptr_curves: Option<Vec<PPtrCurve>>,
    pub m_events: Vec<AnimationEvent>,
    pub m_muscle_clip_size: Option<u32>,
    pub m_muscle_clip: Option<ClipMuscleConstant>,
    pub m_clip_binding_constant: Option<AnimationClipMClipBindingConstantVariant>,
    pub m_has_generic_root_transform: Option<bool>,
    pub m_has_motion_float_curves: Option<bool>,
}

impl crate::Parse for AnimationClip {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_compressed = None;
        let mut m_animation_type = None;
        let mut m_legacy = None;
        let mut m_rotation_curves = None;
        let mut m_compressed_rotation_curves = None;
        let mut m_use_high_quality_curve = None;
        let mut m_position_curves = None;
        let mut m_scale_curves = None;
        let mut m_float_curves = None;
        let mut m_euler_curves = None;
        let mut m_sample_rate = None;
        let mut m_wrap_mode = None;
        let mut m_bounds = None;
        let mut m_p_ptr_curves = None;
        let mut m_events = None;
        let mut m_muscle_clip_size = None;
        let mut m_muscle_clip = None;
        let mut m_clip_binding_constant = None;
        let mut m_has_generic_root_transform = None;
        let mut m_has_motion_float_curves = None;
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_legacy = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_animation_type = Some(rdr.read_i32()?);
        } else {
        }
            m_compressed = Some(AnimationClipMCompressedVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_use_high_quality_curve = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_rotation_curves = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { QuaternionCurve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_compressed_rotation_curves = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { CompressedAnimationCurve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_euler_curves = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector3Curve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }
            m_position_curves = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector3Curve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_scale_curves = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector3Curve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_float_curves = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { FloatCurve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_p_ptr_curves = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { PPtrCurve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }
            m_sample_rate = Some(rdr.read_f32()?);
            m_wrap_mode = Some(rdr.read_i32()?);
            m_bounds = Some(Aabb::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_muscle_clip_size = Some(rdr.read_u32()?);
            m_muscle_clip = Some(ClipMuscleConstant::parse(rdr, ver)?);
            rdr.align(4)?;
            m_clip_binding_constant = Some(AnimationClipMClipBindingConstantVariant::parse(rdr, ver)?);
            m_has_generic_root_transform = Some(rdr.read_bool()?);
            m_has_motion_float_curves = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_muscle_clip_size = Some(rdr.read_u32()?);
            m_muscle_clip = Some(ClipMuscleConstant::parse(rdr, ver)?);
            rdr.align(4)?;
            m_clip_binding_constant = Some(AnimationClipMClipBindingConstantVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_muscle_clip_size = Some(rdr.read_u32()?);
            m_muscle_clip = Some(ClipMuscleConstant::parse(rdr, ver)?);
            rdr.align(4)?;
        } else {
        }
            m_events = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AnimationEvent::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;

        Ok(AnimationClip {
            m_name: m_name.unwrap(),
            m_compressed: m_compressed.unwrap(),
            m_animation_type,
            m_legacy,
            m_rotation_curves: m_rotation_curves.unwrap(),
            m_compressed_rotation_curves: m_compressed_rotation_curves.unwrap(),
            m_use_high_quality_curve,
            m_position_curves: m_position_curves.unwrap(),
            m_scale_curves: m_scale_curves.unwrap(),
            m_float_curves: m_float_curves.unwrap(),
            m_euler_curves,
            m_sample_rate: m_sample_rate.unwrap(),
            m_wrap_mode: m_wrap_mode.unwrap(),
            m_bounds: m_bounds.unwrap(),
            m_p_ptr_curves,
            m_events: m_events.unwrap(),
            m_muscle_clip_size,
            m_muscle_clip,
            m_clip_binding_constant,
            m_has_generic_root_transform,
            m_has_motion_float_curves,
        })
    }
}
