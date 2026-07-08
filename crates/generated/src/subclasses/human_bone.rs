//! 自动生成。请勿手动修改。

use crate::subclasses::skeleton_bone_limit::SkeletonBoneLimit;

#[derive(Debug, Clone)]
pub struct HumanBone {
    pub m_bone_name: String,
    pub m_human_name: String,
    pub m_limit: SkeletonBoneLimit,
}

impl crate::Parse for HumanBone {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_bone_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_human_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_limit = SkeletonBoneLimit::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(HumanBone {
            m_bone_name,
            m_human_name,
            m_limit,
        })
    }
}
