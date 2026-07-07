//! 自动生成。请勿手动修改。

use crate::classes::vector3f::Vector3f;
use crate::subclasses::quaternionf::Quaternionf;

#[derive(Debug, Clone)]
pub struct SkeletonBoneLimit {
    pub m_min: Vector3f,
    pub m_max: Vector3f,
    pub m_value: Vector3f,
    pub m_pre_q: Quaternionf,
    pub m_post_q: Quaternionf,
    pub m_length: f32,
    pub m_modified: bool,
}

impl crate::Parse for SkeletonBoneLimit {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_min = Vector3f::parse(r, ver)?;
        let m_max = Vector3f::parse(r, ver)?;
        let m_value = Vector3f::parse(r, ver)?;
        let m_pre_q = Quaternionf::parse(r, ver)?;
        let m_post_q = Quaternionf::parse(r, ver)?;
        let m_length = r.read_f32()?;
        let m_modified = r.read_bool()?;
        r.align(4)?;

        Ok(SkeletonBoneLimit {
            m_min,
            m_max,
            m_value,
            m_pre_q,
            m_post_q,
            m_length,
            m_modified,
        })
    }
}
