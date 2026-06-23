//! 自动生成。请勿手动修改。

use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct PhysicsMaterial2D {
    pub parent: NamedObject,
    pub m_name: String,
    pub friction: f32,
    pub bounciness: f32,
    pub m_friction_combine: Option<i32>,
    pub m_bounce_combine: Option<i32>,
}

impl crate::Parse for PhysicsMaterial2D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let friction = r.read_f32()?;
        let bounciness = r.read_f32()?;
        let m_friction_combine = if *ver >= crate::UnityVersion::new(6000, 0, 14) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_bounce_combine = if *ver >= crate::UnityVersion::new(6000, 0, 14) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(PhysicsMaterial2D {
            parent,
            m_name,
            friction,
            bounciness,
            m_friction_combine,
            m_bounce_combine,
        })
    }
}
