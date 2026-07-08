//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct AutoOffMeshLinkData {
    pub m_start: Vector3f,
    pub m_end: Vector3f,
    pub m_radius: f32,
    pub m_link_type: u16,
    pub m_area: u8,
    pub m_link_direction: u8,
}

impl crate::Parse for AutoOffMeshLinkData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_start = Vector3f::parse(rdr, ver)?;
        let m_end = Vector3f::parse(rdr, ver)?;
        let m_radius = rdr.read_f32()?;
        let m_link_type = rdr.read_u16()?;
        let m_area = rdr.read_u8()?;
        let m_link_direction = rdr.read_u8()?;

        Ok(AutoOffMeshLinkData {
            m_start,
            m_end,
            m_radius,
            m_link_type,
            m_area,
            m_link_direction,
        })
    }
}
