//! 自动生成。请勿手动修改。

use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SkeletonBone {
    pub m_name: String,
    pub m_position: Vector3f,
    pub m_parent_name: Option<String>,
    pub m_rotation: Quaternionf,
    pub m_scale: Vector3f,
    pub m_transform_modified: Option<bool>,
}

impl crate::Parse for SkeletonBone {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_position = None;
        let mut m_parent_name = None;
        let mut m_rotation = None;
        let mut m_scale = None;
        let mut m_transform_modified = None;
            m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_parent_name = Some(rdr.read_aligned_string()?);
        } else {
        }
            m_position = Some(Vector3f::parse(rdr, ver)?);
            m_rotation = Some(Quaternionf::parse(rdr, ver)?);
            m_scale = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else {
            m_transform_modified = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }

        Ok(SkeletonBone {
            m_name: m_name.unwrap(),
            m_position: m_position.unwrap(),
            m_parent_name,
            m_rotation: m_rotation.unwrap(),
            m_scale: m_scale.unwrap(),
            m_transform_modified,
        })
    }
}
