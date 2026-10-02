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

impl crate::ToValue for ClipAnimationInfoFirstFrameVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_0(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ClipAnimationInfoLastFrameVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_0(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ClipAnimationInfoRLoopVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_0(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ClipAnimationInfoMirrorVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_1(v) => (v).inspect_value(path, offset, limit),
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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ClipAnimationInfoBodyMaskVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ClipAnimationInfoCurvesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClipAnimationInfoCurvesVariant {
    Default(Vec<ClipAnimationInfoCurve>),
    V2017_1(Vec<ClipAnimationInfoCurve>),
}

impl crate::Parse for ClipAnimationInfoCurvesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ClipAnimationInfoCurve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ClipAnimationInfoCurve::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ClipAnimationInfoCurvesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ClipAnimationInfoTransformMaskVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClipAnimationInfoTransformMaskVariant {
    Default(Vec<TransformMaskElement>),
    V2017_1(Vec<TransformMaskElement>),
}

impl crate::Parse for ClipAnimationInfoTransformMaskVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TransformMaskElement::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TransformMaskElement::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ClipAnimationInfoTransformMaskVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ClipAnimationInfoEventsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClipAnimationInfoEventsVariant {
    Default(Vec<AnimationEvent>),
    V2017_1(Vec<AnimationEvent>),
}

impl crate::Parse for ClipAnimationInfoEventsVariant {
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

impl crate::ToValue for ClipAnimationInfoEventsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
    pub curves: Option<ClipAnimationInfoCurvesVariant>,
    pub transform_mask: Option<ClipAnimationInfoTransformMaskVariant>,
    pub skeleton_mask_elements: Option<Vec<AvatarSkeletonMaskElement>>,
    pub events: Option<ClipAnimationInfoEventsVariant>,
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
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            take_name = Some(rdr.read_aligned_string()?);
            internal_id = Some(rdr.read_i64()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            take_name = Some(rdr.read_aligned_string()?);
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
            curves = Some(ClipAnimationInfoCurvesVariant::parse(rdr, ver)?);
            events = Some(ClipAnimationInfoEventsVariant::parse(rdr, ver)?);
            transform_mask = Some(ClipAnimationInfoTransformMaskVariant::parse(rdr, ver)?);
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
            curves = Some(ClipAnimationInfoCurvesVariant::parse(rdr, ver)?);
            events = Some(ClipAnimationInfoEventsVariant::parse(rdr, ver)?);
            transform_mask = Some(ClipAnimationInfoTransformMaskVariant::parse(rdr, ver)?);
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
            curves = Some(ClipAnimationInfoCurvesVariant::parse(rdr, ver)?);
            transform_mask = Some(ClipAnimationInfoTransformMaskVariant::parse(rdr, ver)?);
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
            curves = Some(ClipAnimationInfoCurvesVariant::parse(rdr, ver)?);
            skeleton_mask_elements = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AvatarSkeletonMaskElement::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
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

impl crate::ToValue for ClipAnimationInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("firstFrame".to_string(), self.first_frame.to_value())),
            crate::opt_field("takeName", &self.take_name),
            Some(("lastFrame".to_string(), self.last_frame.to_value())),
            crate::opt_field("internalID", &self.internal_id),
            Some(("wrapMode".to_string(), self.wrap_mode.to_value())),
            Some(("loop".to_string(), self.r#loop.to_value())),
            crate::opt_field("orientationOffsetY", &self.orientation_offset_y),
            crate::opt_field("level", &self.level),
            crate::opt_field("cycleOffset", &self.cycle_offset),
            crate::opt_field("loopBlend", &self.loop_blend),
            crate::opt_field("loopTime", &self.loop_time),
            crate::opt_field(
                "hasAdditiveReferencePose",
                &self.has_additive_reference_pose,
            ),
            crate::opt_field("loopBlendOrientation", &self.loop_blend_orientation),
            crate::opt_field("loopBlendPositionY", &self.loop_blend_position_y),
            crate::opt_field("loopBlendPositionXZ", &self.loop_blend_position_xz),
            crate::opt_field("keepOriginalOrientation", &self.keep_original_orientation),
            crate::opt_field("keepOriginalPositionY", &self.keep_original_position_y),
            crate::opt_field("keepOriginalPositionXZ", &self.keep_original_position_xz),
            crate::opt_field("heightFromFeet", &self.height_from_feet),
            crate::opt_field("mirror", &self.mirror),
            crate::opt_field(
                "keepAdditionalBonesAnimation",
                &self.keep_additional_bones_animation,
            ),
            crate::opt_field("bodyMask", &self.body_mask),
            crate::opt_field("curves", &self.curves),
            crate::opt_field("transformMask", &self.transform_mask),
            crate::opt_field("skeletonMaskElements", &self.skeleton_mask_elements),
            crate::opt_field("events", &self.events),
            crate::opt_field("maskType", &self.mask_type),
            crate::opt_field("maskSource", &self.mask_source),
            crate::opt_field(
                "additiveReferencePoseFrame",
                &self.additive_reference_pose_frame,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "firstFrame" => (self.first_frame).inspect_value(path, offset, limit),
                "takeName" => (self.take_name).inspect_value(path, offset, limit),
                "lastFrame" => (self.last_frame).inspect_value(path, offset, limit),
                "internalID" => (self.internal_id).inspect_value(path, offset, limit),
                "wrapMode" => (self.wrap_mode).inspect_value(path, offset, limit),
                "loop" => (self.r#loop).inspect_value(path, offset, limit),
                "orientationOffsetY" => {
                    (self.orientation_offset_y).inspect_value(path, offset, limit)
                }
                "level" => (self.level).inspect_value(path, offset, limit),
                "cycleOffset" => (self.cycle_offset).inspect_value(path, offset, limit),
                "loopBlend" => (self.loop_blend).inspect_value(path, offset, limit),
                "loopTime" => (self.loop_time).inspect_value(path, offset, limit),
                "hasAdditiveReferencePose" => {
                    (self.has_additive_reference_pose).inspect_value(path, offset, limit)
                }
                "loopBlendOrientation" => {
                    (self.loop_blend_orientation).inspect_value(path, offset, limit)
                }
                "loopBlendPositionY" => {
                    (self.loop_blend_position_y).inspect_value(path, offset, limit)
                }
                "loopBlendPositionXZ" => {
                    (self.loop_blend_position_xz).inspect_value(path, offset, limit)
                }
                "keepOriginalOrientation" => {
                    (self.keep_original_orientation).inspect_value(path, offset, limit)
                }
                "keepOriginalPositionY" => {
                    (self.keep_original_position_y).inspect_value(path, offset, limit)
                }
                "keepOriginalPositionXZ" => {
                    (self.keep_original_position_xz).inspect_value(path, offset, limit)
                }
                "heightFromFeet" => (self.height_from_feet).inspect_value(path, offset, limit),
                "mirror" => (self.mirror).inspect_value(path, offset, limit),
                "keepAdditionalBonesAnimation" => {
                    (self.keep_additional_bones_animation).inspect_value(path, offset, limit)
                }
                "bodyMask" => (self.body_mask).inspect_value(path, offset, limit),
                "curves" => (self.curves).inspect_value(path, offset, limit),
                "transformMask" => (self.transform_mask).inspect_value(path, offset, limit),
                "skeletonMaskElements" => {
                    (self.skeleton_mask_elements).inspect_value(path, offset, limit)
                }
                "events" => (self.events).inspect_value(path, offset, limit),
                "maskType" => (self.mask_type).inspect_value(path, offset, limit),
                "maskSource" => (self.mask_source).inspect_value(path, offset, limit),
                "additiveReferencePoseFrame" => {
                    (self.additive_reference_pose_frame).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.first_frame).inspect_value(path, offset, limit))
                .map(|v| ("firstFrame".to_string(), v)),
            ((self.take_name).inspect_value(path, offset, limit))
                .map(|v| ("takeName".to_string(), v)),
            ((self.last_frame).inspect_value(path, offset, limit))
                .map(|v| ("lastFrame".to_string(), v)),
            ((self.internal_id).inspect_value(path, offset, limit))
                .map(|v| ("internalID".to_string(), v)),
            ((self.wrap_mode).inspect_value(path, offset, limit))
                .map(|v| ("wrapMode".to_string(), v)),
            ((self.r#loop).inspect_value(path, offset, limit)).map(|v| ("loop".to_string(), v)),
            ((self.orientation_offset_y).inspect_value(path, offset, limit))
                .map(|v| ("orientationOffsetY".to_string(), v)),
            ((self.level).inspect_value(path, offset, limit)).map(|v| ("level".to_string(), v)),
            ((self.cycle_offset).inspect_value(path, offset, limit))
                .map(|v| ("cycleOffset".to_string(), v)),
            ((self.loop_blend).inspect_value(path, offset, limit))
                .map(|v| ("loopBlend".to_string(), v)),
            ((self.loop_time).inspect_value(path, offset, limit))
                .map(|v| ("loopTime".to_string(), v)),
            ((self.has_additive_reference_pose).inspect_value(path, offset, limit))
                .map(|v| ("hasAdditiveReferencePose".to_string(), v)),
            ((self.loop_blend_orientation).inspect_value(path, offset, limit))
                .map(|v| ("loopBlendOrientation".to_string(), v)),
            ((self.loop_blend_position_y).inspect_value(path, offset, limit))
                .map(|v| ("loopBlendPositionY".to_string(), v)),
            ((self.loop_blend_position_xz).inspect_value(path, offset, limit))
                .map(|v| ("loopBlendPositionXZ".to_string(), v)),
            ((self.keep_original_orientation).inspect_value(path, offset, limit))
                .map(|v| ("keepOriginalOrientation".to_string(), v)),
            ((self.keep_original_position_y).inspect_value(path, offset, limit))
                .map(|v| ("keepOriginalPositionY".to_string(), v)),
            ((self.keep_original_position_xz).inspect_value(path, offset, limit))
                .map(|v| ("keepOriginalPositionXZ".to_string(), v)),
            ((self.height_from_feet).inspect_value(path, offset, limit))
                .map(|v| ("heightFromFeet".to_string(), v)),
            ((self.mirror).inspect_value(path, offset, limit)).map(|v| ("mirror".to_string(), v)),
            ((self.keep_additional_bones_animation).inspect_value(path, offset, limit))
                .map(|v| ("keepAdditionalBonesAnimation".to_string(), v)),
            ((self.body_mask).inspect_value(path, offset, limit))
                .map(|v| ("bodyMask".to_string(), v)),
            ((self.curves).inspect_value(path, offset, limit)).map(|v| ("curves".to_string(), v)),
            ((self.transform_mask).inspect_value(path, offset, limit))
                .map(|v| ("transformMask".to_string(), v)),
            ((self.skeleton_mask_elements).inspect_value(path, offset, limit))
                .map(|v| ("skeletonMaskElements".to_string(), v)),
            ((self.events).inspect_value(path, offset, limit)).map(|v| ("events".to_string(), v)),
            ((self.mask_type).inspect_value(path, offset, limit))
                .map(|v| ("maskType".to_string(), v)),
            ((self.mask_source).inspect_value(path, offset, limit))
                .map(|v| ("maskSource".to_string(), v)),
            ((self.additive_reference_pose_frame).inspect_value(path, offset, limit))
                .map(|v| ("additiveReferencePoseFrame".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
