//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::aabb::Aabb;
use crate::subclasses::animation_event::AnimationEvent;
use crate::subclasses::compressed_animation_curve::CompressedAnimationCurve;
use crate::subclasses::float_curve::FloatCurve;
use crate::subclasses::quaternion_curve::QuaternionCurve;
use crate::subclasses::vector3_curve::Vector3Curve;

#[derive(Debug, Clone)]
pub struct AnimationClip {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_compressed: bool,
    pub m_rotation_curves: Vec<QuaternionCurve>,
    pub m_compressed_rotation_curves: Vec<CompressedAnimationCurve>,
    pub m_position_curves: Vec<Vector3Curve>,
    pub m_scale_curves: Vec<Vector3Curve>,
    pub m_float_curves: Vec<FloatCurve>,
    pub m_sample_rate: f32,
    pub m_wrap_mode: i32,
    pub m_bounds: Aabb,
    pub m_editor_curves: Vec<FloatCurve>,
    pub m_euler_editor_curves: Vec<FloatCurve>,
    pub m_events: Vec<AnimationEvent>,
}

impl crate::Parse for AnimationClip {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_compressed = r.read_bool()?;
        r.align(4)?;
        let m_rotation_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { QuaternionCurve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_compressed_rotation_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { CompressedAnimationCurve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_position_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3Curve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_scale_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3Curve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_float_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { FloatCurve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_sample_rate = r.read_f32()?;
        let m_wrap_mode = r.read_i32()?;
        let m_bounds = Aabb::parse(r, ver)?;
        let m_editor_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { FloatCurve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_euler_editor_curves = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { FloatCurve::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_events = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AnimationEvent::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(AnimationClip {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_compressed,
            m_rotation_curves,
            m_compressed_rotation_curves,
            m_position_curves,
            m_scale_curves,
            m_float_curves,
            m_sample_rate,
            m_wrap_mode,
            m_bounds,
            m_editor_curves,
            m_euler_editor_curves,
            m_events,
        })
    }
}
