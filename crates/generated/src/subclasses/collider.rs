//! 自动生成。请勿手动修改。

use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct Collider {
    pub m_x: xform,
    pub m_type: u32,
    pub m_x_motion_type: u32,
    pub m_y_motion_type: u32,
    pub m_z_motion_type: u32,
    pub m_min_limit_x: f32,
    pub m_max_limit_x: f32,
    pub m_max_limit_y: f32,
    pub m_max_limit_z: f32,
}

impl crate::Parse for Collider {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_x = xform::parse(rdr, ver)?;
        let m_type = rdr.read_u32()?;
        let m_x_motion_type = rdr.read_u32()?;
        let m_y_motion_type = rdr.read_u32()?;
        let m_z_motion_type = rdr.read_u32()?;
        let m_min_limit_x = rdr.read_f32()?;
        let m_max_limit_x = rdr.read_f32()?;
        let m_max_limit_y = rdr.read_f32()?;
        let m_max_limit_z = rdr.read_f32()?;

        Ok(Collider {
            m_x,
            m_type,
            m_x_motion_type,
            m_y_motion_type,
            m_z_motion_type,
            m_min_limit_x,
            m_max_limit_x,
            m_max_limit_y,
            m_max_limit_z,
        })
    }
}
