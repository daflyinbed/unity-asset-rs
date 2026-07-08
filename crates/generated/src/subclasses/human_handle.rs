//! 自动生成。请勿手动修改。

use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct HumanHandle {
    pub m_name: String,
    pub m_bone_name: String,
    pub m_position: Vector3f,
    pub m_rotation: Quaternionf,
    pub m_scale: Vector3f,
    pub m_look_at: bool,
}

impl crate::Parse for HumanHandle {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_bone_name = rdr.read_aligned_string()?;
        let m_position = Vector3f::parse(rdr, ver)?;
        let m_rotation = Quaternionf::parse(rdr, ver)?;
        let m_scale = Vector3f::parse(rdr, ver)?;
        let m_look_at = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(HumanHandle {
            m_name,
            m_bone_name,
            m_position,
            m_rotation,
            m_scale,
            m_look_at,
        })
    }
}
