//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct PhysicsCoreProjectSettings2D {
    pub m_object_hide_flags: u32,
    pub m_physics_core_settings: PPtr,
}

impl crate::Parse for PhysicsCoreProjectSettings2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_physics_core_settings = crate::PPtr::parse(r, ver)?;

        Ok(PhysicsCoreProjectSettings2D {
            m_object_hide_flags,
            m_physics_core_settings,
        })
    }
}
