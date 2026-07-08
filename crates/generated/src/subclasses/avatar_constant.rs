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
            m_skeleton_name_id_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
            m_skeleton = Some(OffsetPtr::parse(rdr, ver)?);
            m_skeleton_pose = Some(OffsetPtr::parse(rdr, ver)?);
        }
            m_human = Some(OffsetPtr::parse(rdr, ver)?);
            m_human_skeleton_index_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_human_skeleton_reverse_index_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
        }
            m_root_motion_bone_index = Some(rdr.read_i32()?);
            m_root_motion_bone_x = Some(xform::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_root_motion_skeleton = Some(OffsetPtr::parse(rdr, ver)?);
            m_root_motion_skeleton_pose = Some(OffsetPtr::parse(rdr, ver)?);
            m_root_motion_skeleton_index_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
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
