//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct AnimatorControllerParameter {
    pub m_name: String,
    pub m_type: i32,
    pub m_default_vector: Vector3f,
    pub m_default_float: f32,
    pub m_default_int: i32,
    pub m_default_bool: bool,
    pub m_controller: PPtr,
}

impl crate::Parse for AnimatorControllerParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_type = rdr.read_i32()?;
        let m_default_vector = Vector3f::parse(rdr, ver)?;
        let m_default_float = rdr.read_f32()?;
        let m_default_int = rdr.read_i32()?;
        let m_default_bool = rdr.read_bool()?;
        rdr.align(4)?;
        let m_controller = crate::PPtr::parse(rdr, ver)?;

        Ok(AnimatorControllerParameter {
            m_name,
            m_type,
            m_default_vector,
            m_default_float,
            m_default_int,
            m_default_bool,
            m_controller,
        })
    }
}
