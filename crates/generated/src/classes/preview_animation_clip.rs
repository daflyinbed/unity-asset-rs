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

#[derive(Debug, Clone)]
pub struct PreviewAnimationClip {
    pub m_name: String,
    pub m_legacy: bool,
    pub m_compressed: bool,
    pub m_use_high_quality_curve: bool,
    pub m_rotation_curves: Vec<QuaternionCurve>,
    pub m_compressed_rotation_curves: Vec<CompressedAnimationCurve>,
    pub m_euler_curves: Vec<Vector3Curve>,
    pub m_position_curves: Vec<Vector3Curve>,
    pub m_scale_curves: Vec<Vector3Curve>,
    pub m_float_curves: Vec<FloatCurve>,
    pub m_p_ptr_curves: Vec<PPtrCurve>,
    pub m_sample_rate: f32,
    pub m_wrap_mode: i32,
    pub m_bounds: Aabb,
    pub m_muscle_clip_size: u32,
    pub m_muscle_clip: ClipMuscleConstant,
    pub m_clip_binding_constant: AnimationClipBindingConstant,
    pub m_events: Vec<AnimationEvent>,
    pub m_has_generic_root_transform: Option<bool>,
    pub m_has_motion_float_curves: Option<bool>,
}

impl crate::Parse for PreviewAnimationClip {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_legacy = r.read_bool()?;
        let m_compressed = r.read_bool()?;
        let m_use_high_quality_curve = r.read_bool()?;
        r.align(4)?;
        let m_rotation_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { QuaternionCurve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_compressed_rotation_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { CompressedAnimationCurve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_euler_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3Curve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_position_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3Curve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_scale_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3Curve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_float_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { FloatCurve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_p_ptr_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { PPtrCurve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_sample_rate = r.read_f32()?;
        let m_wrap_mode = r.read_i32()?;
        let m_bounds = Aabb::parse(r, ver)?;
        let m_muscle_clip_size = r.read_u32()?;
        let m_muscle_clip = ClipMuscleConstant::parse(r, ver)?;
        r.align(4)?;
        let m_clip_binding_constant = AnimationClipBindingConstant::parse(r, ver)?;
        r.align(4)?;
        let m_events = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AnimationEvent::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_has_generic_root_transform = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_has_motion_float_curves = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(PreviewAnimationClip {
            m_name,
            m_legacy,
            m_compressed,
            m_use_high_quality_curve,
            m_rotation_curves,
            m_compressed_rotation_curves,
            m_euler_curves,
            m_position_curves,
            m_scale_curves,
            m_float_curves,
            m_p_ptr_curves,
            m_sample_rate,
            m_wrap_mode,
            m_bounds,
            m_muscle_clip_size,
            m_muscle_clip,
            m_clip_binding_constant,
            m_events,
            m_has_generic_root_transform,
            m_has_motion_float_curves,
        })
    }
}
