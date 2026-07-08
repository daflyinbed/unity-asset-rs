//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PhysicsMaterial2D {
    pub m_name: String,
    pub friction: f32,
    pub bounciness: f32,
    pub m_friction_combine: Option<i32>,
    pub m_bounce_combine: Option<i32>,
}

impl crate::Parse for PhysicsMaterial2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let friction = rdr.read_f32()?;
        let bounciness = rdr.read_f32()?;
        let m_friction_combine = if *ver >= crate::UnityVersion::new(6000, 0, 14) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_bounce_combine = if *ver >= crate::UnityVersion::new(6000, 0, 14) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(PhysicsMaterial2D {
            m_name,
            friction,
            bounciness,
            m_friction_combine,
            m_bounce_combine,
        })
    }
}
