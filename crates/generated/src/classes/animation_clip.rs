//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;
use crate::subclasses::animation_clip_binding_constant::AnimationClipBindingConstant;
use crate::subclasses::animation_event::AnimationEvent;
use crate::subclasses::compressed_animation_curve::CompressedAnimationCurve;
use crate::subclasses::float_curve::FloatCurve;
use crate::subclasses::muscle_clip_constant::MuscleClipConstant;
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

/// 版本枚举：AnimationClipMRotationCurvesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipMRotationCurvesVariant {
    Default(Vec<QuaternionCurve>),
    V2017_1(Vec<QuaternionCurve>),
}

impl crate::Parse for AnimationClipMRotationCurvesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| QuaternionCurve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| QuaternionCurve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimationClipMCompressedRotationCurvesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipMCompressedRotationCurvesVariant {
    Default(Vec<CompressedAnimationCurve>),
    V2017_1(Vec<CompressedAnimationCurve>),
}

impl crate::Parse for AnimationClipMCompressedRotationCurvesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| CompressedAnimationCurve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| CompressedAnimationCurve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimationClipMPositionCurvesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipMPositionCurvesVariant {
    Default(Vec<Vector3Curve>),
    V2017_1(Vec<Vector3Curve>),
}

impl crate::Parse for AnimationClipMPositionCurvesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3Curve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3Curve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimationClipMScaleCurvesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipMScaleCurvesVariant {
    Default(Vec<Vector3Curve>),
    V2017_1(Vec<Vector3Curve>),
}

impl crate::Parse for AnimationClipMScaleCurvesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3Curve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3Curve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimationClipMFloatCurvesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipMFloatCurvesVariant {
    Default(Vec<FloatCurve>),
    V2017_1(Vec<FloatCurve>),
}

impl crate::Parse for AnimationClipMFloatCurvesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| FloatCurve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| FloatCurve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimationClipMEulerCurvesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipMEulerCurvesVariant {
    Default(Vec<Vector3Curve>),
    V2017_1(Vec<Vector3Curve>),
}

impl crate::Parse for AnimationClipMEulerCurvesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3Curve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3Curve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimationClipMPPtrCurvesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipMPPtrCurvesVariant {
    Default(Vec<PPtrCurve>),
    V2017_1(Vec<PPtrCurve>),
}

impl crate::Parse for AnimationClipMPPtrCurvesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| PPtrCurve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| PPtrCurve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimationClipMEventsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipMEventsVariant {
    Default(Vec<AnimationEvent>),
    V2017_1(Vec<AnimationEvent>),
}

impl crate::Parse for AnimationClipMEventsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| AnimationEvent::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| AnimationEvent::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
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
    pub m_rotation_curves: AnimationClipMRotationCurvesVariant,
    pub m_compressed_rotation_curves: AnimationClipMCompressedRotationCurvesVariant,
    pub m_use_high_quality_curve: Option<bool>,
    pub m_position_curves: AnimationClipMPositionCurvesVariant,
    pub m_scale_curves: AnimationClipMScaleCurvesVariant,
    pub m_float_curves: AnimationClipMFloatCurvesVariant,
    pub m_euler_curves: Option<AnimationClipMEulerCurvesVariant>,
    pub m_sample_rate: f32,
    pub m_wrap_mode: i32,
    pub m_bounds: Aabb,
    pub m_p_ptr_curves: Option<AnimationClipMPPtrCurvesVariant>,
    pub m_events: AnimationClipMEventsVariant,
    pub m_muscle_clip_size: Option<u32>,
    pub m_muscle_clip: Option<MuscleClipConstant>,
    pub m_clip_binding_constant: Option<AnimationClipBindingConstant>,
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
        m_rotation_curves = Some(AnimationClipMRotationCurvesVariant::parse(rdr, ver)?);
        m_compressed_rotation_curves = Some(AnimationClipMCompressedRotationCurvesVariant::parse(
            rdr, ver,
        )?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_euler_curves = Some(AnimationClipMEulerCurvesVariant::parse(rdr, ver)?);
        } else {
        }
        m_position_curves = Some(AnimationClipMPositionCurvesVariant::parse(rdr, ver)?);
        m_scale_curves = Some(AnimationClipMScaleCurvesVariant::parse(rdr, ver)?);
        m_float_curves = Some(AnimationClipMFloatCurvesVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_p_ptr_curves = Some(AnimationClipMPPtrCurvesVariant::parse(rdr, ver)?);
        } else {
        }
        m_sample_rate = Some(rdr.read_f32()?);
        m_wrap_mode = Some(rdr.read_i32()?);
        m_bounds = Some(Aabb::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_muscle_clip_size = Some(rdr.read_u32()?);
            m_muscle_clip = Some(MuscleClipConstant::parse(rdr, ver)?);
            m_clip_binding_constant = Some(AnimationClipBindingConstant::parse(rdr, ver)?);
            m_has_generic_root_transform = Some(rdr.read_bool()?);
            m_has_motion_float_curves = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_muscle_clip_size = Some(rdr.read_u32()?);
            m_muscle_clip = Some(MuscleClipConstant::parse(rdr, ver)?);
            m_clip_binding_constant = Some(AnimationClipBindingConstant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_muscle_clip_size = Some(rdr.read_u32()?);
            m_muscle_clip = Some(MuscleClipConstant::parse(rdr, ver)?);
        } else {
        }
        m_events = Some(AnimationClipMEventsVariant::parse(rdr, ver)?);

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
