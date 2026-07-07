//! 自动生成。请勿手动修改。

use crate::classes::vector3f::Vector3f;
use crate::subclasses::quaternionf::Quaternionf;

#[derive(Debug, Clone)]
pub struct SkeletonBone {
    pub m_name: String,
    pub m_position: Vector3f,
    pub m_rotation: Quaternionf,
    pub m_scale: Vector3f,
    pub m_transform_modified: bool,
}

impl crate::Parse for SkeletonBone {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_position = Vector3f::parse(r, ver)?;
        let m_rotation = Quaternionf::parse(r, ver)?;
        let m_scale = Vector3f::parse(r, ver)?;
        let m_transform_modified = r.read_bool()?;
        r.align(4)?;

        Ok(SkeletonBone {
            m_name,
            m_position,
            m_rotation,
            m_scale,
            m_transform_modified,
        })
    }
}
