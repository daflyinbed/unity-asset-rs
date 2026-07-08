//! 自动生成。请勿手动修改。

use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SkeletonBoneLimit {
    pub m_min: Vector3f,
    pub m_max: Vector3f,
    pub m_value: Vector3f,
    pub m_pre_q: Option<Quaternionf>,
    pub m_length: f32,
    pub m_post_q: Option<Quaternionf>,
    pub m_modified: bool,
}

impl crate::Parse for SkeletonBoneLimit {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_min = None;
        let mut m_max = None;
        let mut m_value = None;
        let mut m_pre_q = None;
        let mut m_length = None;
        let mut m_post_q = None;
        let mut m_modified = None;
            m_min = Some(Vector3f::parse(rdr, ver)?);
            m_max = Some(Vector3f::parse(rdr, ver)?);
            m_value = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
        } else {
            m_pre_q = Some(Quaternionf::parse(rdr, ver)?);
            m_post_q = Some(Quaternionf::parse(rdr, ver)?);
        }
            m_length = Some(rdr.read_f32()?);
            m_modified = Some(rdr.read_bool()?);
            rdr.align(4)?;

        Ok(SkeletonBoneLimit {
            m_min: m_min.unwrap(),
            m_max: m_max.unwrap(),
            m_value: m_value.unwrap(),
            m_pre_q,
            m_length: m_length.unwrap(),
            m_post_q,
            m_modified: m_modified.unwrap(),
        })
    }
}
