//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;
use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct AvatarConstant {
    pub m_skeleton: OffsetPtr,
    pub m_skeleton_pose: OffsetPtr,
    pub m_human: OffsetPtr,
    pub m_human_skeleton_index_array: Vec<i32>,
    pub m_root_motion_bone_index: i32,
    pub m_root_motion_bone_x: xform,
}

impl crate::Parse for AvatarConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_skeleton = OffsetPtr::parse(r, ver)?;
        let m_skeleton_pose = OffsetPtr::parse(r, ver)?;
        let m_human = OffsetPtr::parse(r, ver)?;
        r.align(4)?;
        let m_human_skeleton_index_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_root_motion_bone_index = r.read_i32()?;
        let m_root_motion_bone_x = xform::parse(r, ver)?;

        Ok(AvatarConstant {
            m_skeleton,
            m_skeleton_pose,
            m_human,
            m_human_skeleton_index_array,
            m_root_motion_bone_index,
            m_root_motion_bone_x,
        })
    }
}
