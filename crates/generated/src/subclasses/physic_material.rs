//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct PhysicMaterial {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub dynamic_friction: f32,
    pub static_friction: f32,
    pub bounciness: f32,
    pub friction_combine: i32,
    pub bounce_combine: i32,
    pub friction_direction2: Vector3f,
    pub dynamic_friction2: f32,
    pub static_friction2: f32,
}

impl crate::Parse for PhysicMaterial {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let dynamic_friction = r.read_f32()?;
        let static_friction = r.read_f32()?;
        let bounciness = r.read_f32()?;
        let friction_combine = r.read_i32()?;
        let bounce_combine = r.read_i32()?;
        let friction_direction2 = Vector3f::parse(r, ver)?;
        let dynamic_friction2 = r.read_f32()?;
        let static_friction2 = r.read_f32()?;

        Ok(PhysicMaterial {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            dynamic_friction,
            static_friction,
            bounciness,
            friction_combine,
            bounce_combine,
            friction_direction2,
            dynamic_friction2,
            static_friction2,
        })
    }
}
