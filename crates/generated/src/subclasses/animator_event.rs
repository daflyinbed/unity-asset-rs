//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct AnimatorEvent {
    pub m_name: String,
    pub m_type: i32,
    pub m_min: f32,
    pub m_max: f32,
    pub m_default_vector: Vector3f,
    pub m_default_float: f32,
    pub m_default_int: i32,
    pub m_default_bool: bool,
}

impl crate::Parse for AnimatorEvent {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_type = r.read_i32()?;
        let m_min = r.read_f32()?;
        let m_max = r.read_f32()?;
        let m_default_vector = Vector3f::parse(r, ver)?;
        let m_default_float = r.read_f32()?;
        let m_default_int = r.read_i32()?;
        let m_default_bool = r.read_bool()?;

        Ok(AnimatorEvent {
            m_name,
            m_type,
            m_min,
            m_max,
            m_default_vector,
            m_default_float,
            m_default_int,
            m_default_bool,
        })
    }
}
