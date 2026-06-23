//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::aabb::Aabb;
use crate::subclasses::animation_clip_binding_constant::AnimationClipBindingConstant;
use crate::subclasses::animation_event::AnimationEvent;
use crate::subclasses::compressed_animation_curve::CompressedAnimationCurve;
use crate::subclasses::float_curve::FloatCurve;
use crate::subclasses::muscle_clip_info::MuscleClipInfo;
use crate::subclasses::p_ptr_curve::PPtrCurve;
use crate::subclasses::quaternion_curve::QuaternionCurve;
use crate::subclasses::vector3_curve::Vector3Curve;

#[derive(Debug, Clone)]
pub struct PreviewAnimationClip {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
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
    pub m_clip_binding_constant: AnimationClipBindingConstant,
    pub m_animation_clip_settings: MuscleClipInfo,
    pub m_editor_curves: Vec<FloatCurve>,
    pub m_euler_editor_curves: Vec<FloatCurve>,
    pub m_has_generic_root_transform: bool,
    pub m_has_motion_float_curves: bool,
    pub m_generate_motion_curves: bool,
    pub m_is_empty: bool,
    pub m_events: Vec<AnimationEvent>,
}

impl crate::Parse for PreviewAnimationClip {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
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
        let m_clip_binding_constant = AnimationClipBindingConstant::parse(r, ver)?;
        r.align(4)?;
        let m_animation_clip_settings = MuscleClipInfo::parse(r, ver)?;
        r.align(4)?;
        let m_editor_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { FloatCurve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_euler_editor_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { FloatCurve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_has_generic_root_transform = r.read_bool()?;
        let m_has_motion_float_curves = r.read_bool()?;
        let m_generate_motion_curves = r.read_bool()?;
        let m_is_empty = r.read_bool()?;
        r.align(4)?;
        let m_events = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AnimationEvent::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(PreviewAnimationClip {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
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
            m_clip_binding_constant,
            m_animation_clip_settings,
            m_editor_curves,
            m_euler_editor_curves,
            m_has_generic_root_transform,
            m_has_motion_float_curves,
            m_generate_motion_curves,
            m_is_empty,
            m_events,
        })
    }
}
