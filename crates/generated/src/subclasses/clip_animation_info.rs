//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::animation_event::AnimationEvent;
use crate::subclasses::avatar_skeleton_mask_element::AvatarSkeletonMaskElement;
use crate::subclasses::clip_animation_info_curve::ClipAnimationInfoCurve;
use crate::subclasses::transform_mask_element::TransformMaskElement;


/// 版本枚举：ClipAnimationInfoFirstFrameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClipAnimationInfoFirstFrameVariant {
    Default(i32),
    V4_0(f32),
}

impl crate::Parse for ClipAnimationInfoFirstFrameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_f32()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ClipAnimationInfoLastFrameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClipAnimationInfoLastFrameVariant {
    Default(i32),
    V4_0(f32),
}

impl crate::Parse for ClipAnimationInfoLastFrameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_f32()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ClipAnimationInfoRLoopVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClipAnimationInfoRLoopVariant {
    Default(bool),
    V4_0(bool),
}

impl crate::Parse for ClipAnimationInfoRLoopVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ClipAnimationInfoMirrorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClipAnimationInfoMirrorVariant {
    Default(bool),
    V4_1(bool),
}

impl crate::Parse for ClipAnimationInfoMirrorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V4_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ClipAnimationInfoBodyMaskVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClipAnimationInfoBodyMaskVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
}

impl crate::Parse for ClipAnimationInfoBodyMaskVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClipAnimationInfo {
    pub name: String,
    pub first_frame: ClipAnimationInfoFirstFrameVariant,
    pub take_name: Option<String>,
    pub last_frame: ClipAnimationInfoLastFrameVariant,
    pub internal_id: Option<i64>,
    pub wrap_mode: i32,
    pub r#loop: ClipAnimationInfoRLoopVariant,
    pub orientation_offset_y: Option<f32>,
    pub level: Option<f32>,
    pub cycle_offset: Option<f32>,
    pub loop_blend: Option<bool>,
    pub loop_time: Option<bool>,
    pub has_additive_reference_pose: Option<bool>,
    pub loop_blend_orientation: Option<bool>,
    pub loop_blend_position_y: Option<bool>,
    pub loop_blend_position_xz: Option<bool>,
    pub keep_original_orientation: Option<bool>,
    pub keep_original_position_y: Option<bool>,
    pub keep_original_position_xz: Option<bool>,
    pub height_from_feet: Option<bool>,
    pub mirror: Option<ClipAnimationInfoMirrorVariant>,
    pub keep_additional_bones_animation: Option<bool>,
    pub body_mask: Option<ClipAnimationInfoBodyMaskVariant>,
    pub curves: Option<Vec<ClipAnimationInfoCurve>>,
    pub transform_mask: Option<Vec<TransformMaskElement>>,
    pub skeleton_mask_elements: Option<Vec<AvatarSkeletonMaskElement>>,
    pub events: Option<Vec<AnimationEvent>>,
    pub mask_type: Option<i32>,
    pub mask_source: Option<PPtr>,
    pub additive_reference_pose_frame: Option<f32>,
}

impl crate::Parse for ClipAnimationInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut first_frame = None;
        let mut take_name = None;
        let mut last_frame = None;
        let mut internal_id = None;
        let mut wrap_mode = None;
        let mut r#loop = None;
        let mut orientation_offset_y = None;
        let mut level = None;
        let mut cycle_offset = None;
        let mut loop_blend = None;
        let mut loop_time = None;
        let mut has_additive_reference_pose = None;
        let mut loop_blend_orientation = None;
        let mut loop_blend_position_y = None;
        let mut loop_blend_position_xz = None;
        let mut keep_original_orientation = None;
        let mut keep_original_position_y = None;
        let mut keep_original_position_xz = None;
        let mut height_from_feet = None;
        let mut mirror = None;
        let mut keep_additional_bones_animation = None;
        let mut body_mask = None;
        let mut curves = None;
        let mut transform_mask = None;
        let mut skeleton_mask_elements = None;
        let mut events = None;
        let mut mask_type = None;
        let mut mask_source = None;
        let mut additive_reference_pose_frame = None;
            name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            take_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            internal_id = Some(rdr.read_i64()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            take_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else {
        }
            first_frame = Some(ClipAnimationInfoFirstFrameVariant::parse(rdr, ver)?);
            last_frame = Some(ClipAnimationInfoLastFrameVariant::parse(rdr, ver)?);
            wrap_mode = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            orientation_offset_y = Some(rdr.read_f32()?);
            level = Some(rdr.read_f32()?);
            cycle_offset = Some(rdr.read_f32()?);
        } else {
        }
            r#loop = Some(ClipAnimationInfoRLoopVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            has_additive_reference_pose = Some(rdr.read_bool()?);
            loop_time = Some(rdr.read_bool()?);
            loop_blend = Some(rdr.read_bool()?);
            loop_blend_orientation = Some(rdr.read_bool()?);
            loop_blend_position_y = Some(rdr.read_bool()?);
            loop_blend_position_xz = Some(rdr.read_bool()?);
            keep_original_orientation = Some(rdr.read_bool()?);
            keep_original_position_y = Some(rdr.read_bool()?);
            keep_original_position_xz = Some(rdr.read_bool()?);
            height_from_feet = Some(rdr.read_bool()?);
            mirror = Some(ClipAnimationInfoMirrorVariant::parse(rdr, ver)?);
            body_mask = Some(ClipAnimationInfoBodyMaskVariant::parse(rdr, ver)?);
            curves = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClipAnimationInfoCurve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            events = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AnimationEvent::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            transform_mask = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TransformMaskElement::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            mask_type = Some(rdr.read_i32()?);
            mask_source = Some(crate::PPtr::parse(rdr, ver)?);
            additive_reference_pose_frame = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            loop_time = Some(rdr.read_bool()?);
            loop_blend = Some(rdr.read_bool()?);
            loop_blend_orientation = Some(rdr.read_bool()?);
            loop_blend_position_y = Some(rdr.read_bool()?);
            loop_blend_position_xz = Some(rdr.read_bool()?);
            keep_original_orientation = Some(rdr.read_bool()?);
            keep_original_position_y = Some(rdr.read_bool()?);
            keep_original_position_xz = Some(rdr.read_bool()?);
            height_from_feet = Some(rdr.read_bool()?);
            mirror = Some(ClipAnimationInfoMirrorVariant::parse(rdr, ver)?);
            body_mask = Some(ClipAnimationInfoBodyMaskVariant::parse(rdr, ver)?);
            curves = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClipAnimationInfoCurve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            events = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AnimationEvent::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            transform_mask = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TransformMaskElement::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            mask_type = Some(rdr.read_i32()?);
            mask_source = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            loop_blend = Some(rdr.read_bool()?);
            loop_blend_orientation = Some(rdr.read_bool()?);
            loop_blend_position_y = Some(rdr.read_bool()?);
            loop_blend_position_xz = Some(rdr.read_bool()?);
            keep_original_orientation = Some(rdr.read_bool()?);
            keep_original_position_y = Some(rdr.read_bool()?);
            keep_original_position_xz = Some(rdr.read_bool()?);
            height_from_feet = Some(rdr.read_bool()?);
            mirror = Some(ClipAnimationInfoMirrorVariant::parse(rdr, ver)?);
            body_mask = Some(ClipAnimationInfoBodyMaskVariant::parse(rdr, ver)?);
            curves = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClipAnimationInfoCurve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            transform_mask = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TransformMaskElement::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            loop_blend = Some(rdr.read_bool()?);
            loop_blend_orientation = Some(rdr.read_bool()?);
            loop_blend_position_y = Some(rdr.read_bool()?);
            loop_blend_position_xz = Some(rdr.read_bool()?);
            keep_original_orientation = Some(rdr.read_bool()?);
            keep_original_position_y = Some(rdr.read_bool()?);
            keep_original_position_xz = Some(rdr.read_bool()?);
            height_from_feet = Some(rdr.read_bool()?);
            mirror = Some(ClipAnimationInfoMirrorVariant::parse(rdr, ver)?);
            keep_additional_bones_animation = Some(rdr.read_bool()?);
            rdr.align(4)?;
            body_mask = Some(ClipAnimationInfoBodyMaskVariant::parse(rdr, ver)?);
            curves = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClipAnimationInfoCurve::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            skeleton_mask_elements = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AvatarSkeletonMaskElement::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }

        Ok(ClipAnimationInfo {
            name: name.unwrap(),
            first_frame: first_frame.unwrap(),
            take_name,
            last_frame: last_frame.unwrap(),
            internal_id,
            wrap_mode: wrap_mode.unwrap(),
            r#loop: r#loop.unwrap(),
            orientation_offset_y,
            level,
            cycle_offset,
            loop_blend,
            loop_time,
            has_additive_reference_pose,
            loop_blend_orientation,
            loop_blend_position_y,
            loop_blend_position_xz,
            keep_original_orientation,
            keep_original_position_y,
            keep_original_position_xz,
            height_from_feet,
            mirror,
            keep_additional_bones_animation,
            body_mask,
            curves,
            transform_mask,
            skeleton_mask_elements,
            events,
            mask_type,
            mask_source,
            additive_reference_pose_frame,
        })
    }
}
