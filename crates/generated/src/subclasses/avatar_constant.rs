//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;
use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct AvatarConstant {
    pub m_skeleton: Option<OffsetPtr>,
    pub m_avatar_skeleton: Option<OffsetPtr>,
    pub m_skeleton_pose: Option<OffsetPtr>,
    pub m_avatar_skeleton_pose: Option<OffsetPtr>,
    pub m_human: OffsetPtr,
    pub m_default_pose: Option<OffsetPtr>,
    pub m_human_skeleton_index_array: Vec<i32>,
    pub m_skeleton_name_id_array: Option<Vec<u32>>,
    pub m_root_motion_bone_index: i32,
    pub m_root_motion_bone_x: xform,
    pub m_human_skeleton_reverse_index_array: Option<Vec<i32>>,
    pub m_root_motion_skeleton: Option<OffsetPtr>,
    pub m_root_motion_skeleton_pose: Option<OffsetPtr>,
    pub m_root_motion_skeleton_index_array: Option<Vec<i32>>,
}

impl crate::Parse for AvatarConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_skeleton = None;
        let mut m_avatar_skeleton = None;
        let mut m_skeleton_pose = None;
        let mut m_avatar_skeleton_pose = None;
        let mut m_human = None;
        let mut m_default_pose = None;
        let mut m_human_skeleton_index_array = None;
        let mut m_skeleton_name_id_array = None;
        let mut m_root_motion_bone_index = None;
        let mut m_root_motion_bone_x = None;
        let mut m_human_skeleton_reverse_index_array = None;
        let mut m_root_motion_skeleton = None;
        let mut m_root_motion_skeleton_pose = None;
        let mut m_root_motion_skeleton_index_array = None;
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_avatar_skeleton = Some(OffsetPtr::parse(rdr, ver)?);
            m_avatar_skeleton_pose = Some(OffsetPtr::parse(rdr, ver)?);
            m_default_pose = Some(OffsetPtr::parse(rdr, ver)?);
            m_skeleton_name_id_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
            m_skeleton = Some(OffsetPtr::parse(rdr, ver)?);
            m_skeleton_pose = Some(OffsetPtr::parse(rdr, ver)?);
        }
        m_human = Some(OffsetPtr::parse(rdr, ver)?);
        m_human_skeleton_index_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_i32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_human_skeleton_reverse_index_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
        }
        m_root_motion_bone_index = Some(rdr.read_i32()?);
        m_root_motion_bone_x = Some(xform::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_root_motion_skeleton = Some(OffsetPtr::parse(rdr, ver)?);
            m_root_motion_skeleton_pose = Some(OffsetPtr::parse(rdr, ver)?);
            m_root_motion_skeleton_index_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }

        Ok(AvatarConstant {
            m_skeleton,
            m_avatar_skeleton,
            m_skeleton_pose,
            m_avatar_skeleton_pose,
            m_human: m_human.unwrap(),
            m_default_pose,
            m_human_skeleton_index_array: m_human_skeleton_index_array.unwrap(),
            m_skeleton_name_id_array,
            m_root_motion_bone_index: m_root_motion_bone_index.unwrap(),
            m_root_motion_bone_x: m_root_motion_bone_x.unwrap(),
            m_human_skeleton_reverse_index_array,
            m_root_motion_skeleton,
            m_root_motion_skeleton_pose,
            m_root_motion_skeleton_index_array,
        })
    }
}

impl crate::ToValue for AvatarConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_Skeleton", &self.m_skeleton),
            crate::opt_field("m_AvatarSkeleton", &self.m_avatar_skeleton),
            crate::opt_field("m_SkeletonPose", &self.m_skeleton_pose),
            crate::opt_field("m_AvatarSkeletonPose", &self.m_avatar_skeleton_pose),
            Some(("m_Human".to_string(), self.m_human.to_value())),
            crate::opt_field("m_DefaultPose", &self.m_default_pose),
            Some((
                "m_HumanSkeletonIndexArray".to_string(),
                self.m_human_skeleton_index_array.to_value(),
            )),
            crate::opt_field("m_SkeletonNameIDArray", &self.m_skeleton_name_id_array),
            Some((
                "m_RootMotionBoneIndex".to_string(),
                self.m_root_motion_bone_index.to_value(),
            )),
            Some((
                "m_RootMotionBoneX".to_string(),
                self.m_root_motion_bone_x.to_value(),
            )),
            crate::opt_field(
                "m_HumanSkeletonReverseIndexArray",
                &self.m_human_skeleton_reverse_index_array,
            ),
            crate::opt_field("m_RootMotionSkeleton", &self.m_root_motion_skeleton),
            crate::opt_field(
                "m_RootMotionSkeletonPose",
                &self.m_root_motion_skeleton_pose,
            ),
            crate::opt_field(
                "m_RootMotionSkeletonIndexArray",
                &self.m_root_motion_skeleton_index_array,
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
                "m_Skeleton" => (self.m_skeleton).inspect_value(path, offset, limit),
                "m_AvatarSkeleton" => (self.m_avatar_skeleton).inspect_value(path, offset, limit),
                "m_SkeletonPose" => (self.m_skeleton_pose).inspect_value(path, offset, limit),
                "m_AvatarSkeletonPose" => {
                    (self.m_avatar_skeleton_pose).inspect_value(path, offset, limit)
                }
                "m_Human" => (self.m_human).inspect_value(path, offset, limit),
                "m_DefaultPose" => (self.m_default_pose).inspect_value(path, offset, limit),
                "m_HumanSkeletonIndexArray" => {
                    (self.m_human_skeleton_index_array).inspect_value(path, offset, limit)
                }
                "m_SkeletonNameIDArray" => {
                    (self.m_skeleton_name_id_array).inspect_value(path, offset, limit)
                }
                "m_RootMotionBoneIndex" => {
                    (self.m_root_motion_bone_index).inspect_value(path, offset, limit)
                }
                "m_RootMotionBoneX" => {
                    (self.m_root_motion_bone_x).inspect_value(path, offset, limit)
                }
                "m_HumanSkeletonReverseIndexArray" => {
                    (self.m_human_skeleton_reverse_index_array).inspect_value(path, offset, limit)
                }
                "m_RootMotionSkeleton" => {
                    (self.m_root_motion_skeleton).inspect_value(path, offset, limit)
                }
                "m_RootMotionSkeletonPose" => {
                    (self.m_root_motion_skeleton_pose).inspect_value(path, offset, limit)
                }
                "m_RootMotionSkeletonIndexArray" => {
                    (self.m_root_motion_skeleton_index_array).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_skeleton).inspect_value(path, offset, limit))
                .map(|v| ("m_Skeleton".to_string(), v)),
            ((self.m_avatar_skeleton).inspect_value(path, offset, limit))
                .map(|v| ("m_AvatarSkeleton".to_string(), v)),
            ((self.m_skeleton_pose).inspect_value(path, offset, limit))
                .map(|v| ("m_SkeletonPose".to_string(), v)),
            ((self.m_avatar_skeleton_pose).inspect_value(path, offset, limit))
                .map(|v| ("m_AvatarSkeletonPose".to_string(), v)),
            ((self.m_human).inspect_value(path, offset, limit)).map(|v| ("m_Human".to_string(), v)),
            ((self.m_default_pose).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultPose".to_string(), v)),
            ((self.m_human_skeleton_index_array).inspect_value(path, offset, limit))
                .map(|v| ("m_HumanSkeletonIndexArray".to_string(), v)),
            ((self.m_skeleton_name_id_array).inspect_value(path, offset, limit))
                .map(|v| ("m_SkeletonNameIDArray".to_string(), v)),
            ((self.m_root_motion_bone_index).inspect_value(path, offset, limit))
                .map(|v| ("m_RootMotionBoneIndex".to_string(), v)),
            ((self.m_root_motion_bone_x).inspect_value(path, offset, limit))
                .map(|v| ("m_RootMotionBoneX".to_string(), v)),
            ((self.m_human_skeleton_reverse_index_array).inspect_value(path, offset, limit))
                .map(|v| ("m_HumanSkeletonReverseIndexArray".to_string(), v)),
            ((self.m_root_motion_skeleton).inspect_value(path, offset, limit))
                .map(|v| ("m_RootMotionSkeleton".to_string(), v)),
            ((self.m_root_motion_skeleton_pose).inspect_value(path, offset, limit))
                .map(|v| ("m_RootMotionSkeletonPose".to_string(), v)),
            ((self.m_root_motion_skeleton_index_array).inspect_value(path, offset, limit))
                .map(|v| ("m_RootMotionSkeletonIndexArray".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
