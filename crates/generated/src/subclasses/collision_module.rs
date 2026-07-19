//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::min_max_curve::MinMaxCurve;

/// 版本枚举：CollisionModuleQualityVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CollisionModuleQualityVariant {
    Default(i32),
    V5_3(i32),
}

impl crate::Parse for CollisionModuleQualityVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_3(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct CollisionModule {
    pub enabled: bool,
    pub r#type: i32,
    pub plane0: Option<PPtr>,
    pub collision_mode: Option<i32>,
    pub plane1: Option<PPtr>,
    pub collider_force: Option<f32>,
    pub plane2: Option<PPtr>,
    pub multiply_collider_force_by_particle_size: Option<bool>,
    pub plane3: Option<PPtr>,
    pub multiply_collider_force_by_particle_speed: Option<bool>,
    pub plane4: Option<PPtr>,
    pub multiply_collider_force_by_collision_angle: Option<bool>,
    pub plane5: Option<PPtr>,
    pub m_planes: Option<Vec<PPtr>>,
    pub dampen: Option<f32>,
    pub m_dampen: Option<MinMaxCurve>,
    pub bounce: Option<f32>,
    pub m_bounce: Option<MinMaxCurve>,
    pub energy_loss_on_collision: Option<f32>,
    pub m_energy_loss_on_collision: Option<MinMaxCurve>,
    pub min_kill_speed: f32,
    pub particle_radius: Option<f32>,
    pub max_kill_speed: Option<f32>,
    pub collides_with: Option<BitField>,
    pub radius_scale: Option<f32>,
    pub quality: Option<CollisionModuleQualityVariant>,
    pub voxel_size: Option<f32>,
    pub max_collision_shapes: Option<i32>,
    pub collision_messages: Option<bool>,
    pub collides_with_dynamic: Option<bool>,
    pub interior_collisions: Option<bool>,
}

impl crate::Parse for CollisionModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut r#type = None;
        let mut plane0 = None;
        let mut collision_mode = None;
        let mut plane1 = None;
        let mut collider_force = None;
        let mut plane2 = None;
        let mut multiply_collider_force_by_particle_size = None;
        let mut plane3 = None;
        let mut multiply_collider_force_by_particle_speed = None;
        let mut plane4 = None;
        let mut multiply_collider_force_by_collision_angle = None;
        let mut plane5 = None;
        let mut m_planes = None;
        let mut dampen = None;
        let mut m_dampen = None;
        let mut bounce = None;
        let mut m_bounce = None;
        let mut energy_loss_on_collision = None;
        let mut m_energy_loss_on_collision = None;
        let mut min_kill_speed = None;
        let mut particle_radius = None;
        let mut max_kill_speed = None;
        let mut collides_with = None;
        let mut radius_scale = None;
        let mut quality = None;
        let mut voxel_size = None;
        let mut max_collision_shapes = None;
        let mut collision_messages = None;
        let mut collides_with_dynamic = None;
        let mut interior_collisions = None;
        enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        r#type = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            collision_mode = Some(rdr.read_i32()?);
            collider_force = Some(rdr.read_f32()?);
            multiply_collider_force_by_particle_size = Some(rdr.read_bool()?);
            multiply_collider_force_by_particle_speed = Some(rdr.read_bool()?);
            multiply_collider_force_by_collision_angle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_planes = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_dampen = Some(MinMaxCurve::parse(rdr, ver)?);
            m_bounce = Some(MinMaxCurve::parse(rdr, ver)?);
            m_energy_loss_on_collision = Some(MinMaxCurve::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            collision_mode = Some(rdr.read_i32()?);
            collider_force = Some(rdr.read_f32()?);
            multiply_collider_force_by_particle_size = Some(rdr.read_bool()?);
            multiply_collider_force_by_particle_speed = Some(rdr.read_bool()?);
            multiply_collider_force_by_collision_angle = Some(rdr.read_bool()?);
            rdr.align(4)?;
            plane0 = Some(crate::PPtr::parse(rdr, ver)?);
            plane1 = Some(crate::PPtr::parse(rdr, ver)?);
            plane2 = Some(crate::PPtr::parse(rdr, ver)?);
            plane3 = Some(crate::PPtr::parse(rdr, ver)?);
            plane4 = Some(crate::PPtr::parse(rdr, ver)?);
            plane5 = Some(crate::PPtr::parse(rdr, ver)?);
            m_dampen = Some(MinMaxCurve::parse(rdr, ver)?);
            m_bounce = Some(MinMaxCurve::parse(rdr, ver)?);
            m_energy_loss_on_collision = Some(MinMaxCurve::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            collision_mode = Some(rdr.read_i32()?);
            plane0 = Some(crate::PPtr::parse(rdr, ver)?);
            plane1 = Some(crate::PPtr::parse(rdr, ver)?);
            plane2 = Some(crate::PPtr::parse(rdr, ver)?);
            plane3 = Some(crate::PPtr::parse(rdr, ver)?);
            plane4 = Some(crate::PPtr::parse(rdr, ver)?);
            plane5 = Some(crate::PPtr::parse(rdr, ver)?);
            m_dampen = Some(MinMaxCurve::parse(rdr, ver)?);
            m_bounce = Some(MinMaxCurve::parse(rdr, ver)?);
            m_energy_loss_on_collision = Some(MinMaxCurve::parse(rdr, ver)?);
        } else {
            plane0 = Some(crate::PPtr::parse(rdr, ver)?);
            plane1 = Some(crate::PPtr::parse(rdr, ver)?);
            plane2 = Some(crate::PPtr::parse(rdr, ver)?);
            plane3 = Some(crate::PPtr::parse(rdr, ver)?);
            plane4 = Some(crate::PPtr::parse(rdr, ver)?);
            plane5 = Some(crate::PPtr::parse(rdr, ver)?);
            dampen = Some(rdr.read_f32()?);
            bounce = Some(rdr.read_f32()?);
            energy_loss_on_collision = Some(rdr.read_f32()?);
        }
        min_kill_speed = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            max_kill_speed = Some(rdr.read_f32()?);
            radius_scale = Some(rdr.read_f32()?);
            collides_with = Some(BitField::parse(rdr, ver)?);
            max_collision_shapes = Some(rdr.read_i32()?);
            quality = Some(CollisionModuleQualityVariant::parse(rdr, ver)?);
            voxel_size = Some(rdr.read_f32()?);
            collision_messages = Some(rdr.read_bool()?);
            collides_with_dynamic = Some(rdr.read_bool()?);
            interior_collisions = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            radius_scale = Some(rdr.read_f32()?);
            collides_with = Some(BitField::parse(rdr, ver)?);
            max_collision_shapes = Some(rdr.read_i32()?);
            quality = Some(CollisionModuleQualityVariant::parse(rdr, ver)?);
            voxel_size = Some(rdr.read_f32()?);
            collision_messages = Some(rdr.read_bool()?);
            collides_with_dynamic = Some(rdr.read_bool()?);
            interior_collisions = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            particle_radius = Some(rdr.read_f32()?);
            rdr.align(4)?;
            collides_with = Some(BitField::parse(rdr, ver)?);
            quality = Some(CollisionModuleQualityVariant::parse(rdr, ver)?);
            voxel_size = Some(rdr.read_f32()?);
            collision_messages = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            particle_radius = Some(rdr.read_f32()?);
            rdr.align(4)?;
            collides_with = Some(BitField::parse(rdr, ver)?);
            quality = Some(CollisionModuleQualityVariant::parse(rdr, ver)?);
            voxel_size = Some(rdr.read_f32()?);
        } else {
        }

        Ok(CollisionModule {
            enabled: enabled.unwrap(),
            r#type: r#type.unwrap(),
            plane0,
            collision_mode,
            plane1,
            collider_force,
            plane2,
            multiply_collider_force_by_particle_size,
            plane3,
            multiply_collider_force_by_particle_speed,
            plane4,
            multiply_collider_force_by_collision_angle,
            plane5,
            m_planes,
            dampen,
            m_dampen,
            bounce,
            m_bounce,
            energy_loss_on_collision,
            m_energy_loss_on_collision,
            min_kill_speed: min_kill_speed.unwrap(),
            particle_radius,
            max_kill_speed,
            collides_with,
            radius_scale,
            quality,
            voxel_size,
            max_collision_shapes,
            collision_messages,
            collides_with_dynamic,
            interior_collisions,
        })
    }
}
