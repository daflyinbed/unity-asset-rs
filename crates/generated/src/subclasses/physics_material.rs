//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct PhysicsMaterial {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_name: String,
    pub dynamic_friction: f32,
    pub static_friction: f32,
    pub bounciness: f32,
    pub friction_combine: i32,
    pub bounce_combine: i32,
}

impl crate::Parse for PhysicsMaterial {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let dynamic_friction = r.read_f32()?;
        let static_friction = r.read_f32()?;
        let bounciness = r.read_f32()?;
        let friction_combine = r.read_i32()?;
        let bounce_combine = r.read_i32()?;

        Ok(PhysicsMaterial {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_name,
            dynamic_friction,
            static_friction,
            bounciness,
            friction_combine,
            bounce_combine,
        })
    }
}
