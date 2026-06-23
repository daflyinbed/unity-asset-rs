//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct PhysicMaterial {
    pub parent: NamedObject,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let dynamic_friction = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_dynamic_friction = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let static_friction = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_static_friction = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let bounciness = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_bounciness = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let friction_combine = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_friction_combine = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let bounce_combine = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_bounce_combine = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let friction_direction2 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(Vector3f::parse(r, ver)?)
        } else {
            None
        };
        let dynamic_friction2 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let static_friction2 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };

        Ok(PhysicMaterial {
            parent,
            m_name,
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
