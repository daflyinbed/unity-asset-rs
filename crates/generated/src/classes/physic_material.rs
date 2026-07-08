//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct PhysicMaterial {
    pub m_name: String,
    pub dynamic_friction: Option<f32>,
    pub m_dynamic_friction: Option<f32>,
    pub static_friction: Option<f32>,
    pub m_static_friction: Option<f32>,
    pub bounciness: Option<f32>,
    pub m_bounciness: Option<f32>,
    pub friction_combine: Option<i32>,
    pub m_friction_combine: Option<i32>,
    pub bounce_combine: Option<i32>,
    pub m_bounce_combine: Option<i32>,
    pub friction_direction2: Option<Vector3f>,
    pub dynamic_friction2: Option<f32>,
    pub static_friction2: Option<f32>,
}

impl crate::Parse for PhysicMaterial {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut dynamic_friction = None;
        let mut m_dynamic_friction = None;
        let mut static_friction = None;
        let mut m_static_friction = None;
        let mut bounciness = None;
        let mut m_bounciness = None;
        let mut friction_combine = None;
        let mut m_friction_combine = None;
        let mut bounce_combine = None;
        let mut m_bounce_combine = None;
        let mut friction_direction2 = None;
        let mut dynamic_friction2 = None;
        let mut static_friction2 = None;
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_dynamic_friction = Some(rdr.read_f32()?);
            m_static_friction = Some(rdr.read_f32()?);
            m_bounciness = Some(rdr.read_f32()?);
            m_friction_combine = Some(rdr.read_i32()?);
            m_bounce_combine = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            dynamic_friction = Some(rdr.read_f32()?);
            static_friction = Some(rdr.read_f32()?);
            bounciness = Some(rdr.read_f32()?);
            friction_combine = Some(rdr.read_i32()?);
            bounce_combine = Some(rdr.read_i32()?);
        } else {
            dynamic_friction = Some(rdr.read_f32()?);
            static_friction = Some(rdr.read_f32()?);
            bounciness = Some(rdr.read_f32()?);
            friction_combine = Some(rdr.read_i32()?);
            bounce_combine = Some(rdr.read_i32()?);
            friction_direction2 = Some(Vector3f::parse(rdr, ver)?);
            dynamic_friction2 = Some(rdr.read_f32()?);
            static_friction2 = Some(rdr.read_f32()?);
        }

        Ok(PhysicMaterial {
            m_name: m_name.unwrap(),
            dynamic_friction,
            m_dynamic_friction,
            static_friction,
            m_static_friction,
            bounciness,
            m_bounciness,
            friction_combine,
            m_friction_combine,
            bounce_combine,
            m_bounce_combine,
            friction_direction2,
            dynamic_friction2,
            static_friction2,
        })
    }
}
