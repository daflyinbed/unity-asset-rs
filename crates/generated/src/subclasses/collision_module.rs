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

impl crate::ToValue for CollisionModuleQualityVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for CollisionModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("type".to_string(), self.r#type.to_value())),
            crate::opt_field("plane0", &self.plane0),
            crate::opt_field("collisionMode", &self.collision_mode),
            crate::opt_field("plane1", &self.plane1),
            crate::opt_field("colliderForce", &self.collider_force),
            crate::opt_field("plane2", &self.plane2),
            crate::opt_field(
                "multiplyColliderForceByParticleSize",
                &self.multiply_collider_force_by_particle_size,
            ),
            crate::opt_field("plane3", &self.plane3),
            crate::opt_field(
                "multiplyColliderForceByParticleSpeed",
                &self.multiply_collider_force_by_particle_speed,
            ),
            crate::opt_field("plane4", &self.plane4),
            crate::opt_field(
                "multiplyColliderForceByCollisionAngle",
                &self.multiply_collider_force_by_collision_angle,
            ),
            crate::opt_field("plane5", &self.plane5),
            crate::opt_field("m_Planes", &self.m_planes),
            crate::opt_field("dampen", &self.dampen),
            crate::opt_field("m_Dampen", &self.m_dampen),
            crate::opt_field("bounce", &self.bounce),
            crate::opt_field("m_Bounce", &self.m_bounce),
            crate::opt_field("energyLossOnCollision", &self.energy_loss_on_collision),
            crate::opt_field("m_EnergyLossOnCollision", &self.m_energy_loss_on_collision),
            Some(("minKillSpeed".to_string(), self.min_kill_speed.to_value())),
            crate::opt_field("particleRadius", &self.particle_radius),
            crate::opt_field("maxKillSpeed", &self.max_kill_speed),
            crate::opt_field("collidesWith", &self.collides_with),
            crate::opt_field("radiusScale", &self.radius_scale),
            crate::opt_field("quality", &self.quality),
            crate::opt_field("voxelSize", &self.voxel_size),
            crate::opt_field("maxCollisionShapes", &self.max_collision_shapes),
            crate::opt_field("collisionMessages", &self.collision_messages),
            crate::opt_field("collidesWithDynamic", &self.collides_with_dynamic),
            crate::opt_field("interiorCollisions", &self.interior_collisions),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "plane0" => (self.plane0).inspect_value(path, offset, limit),
                "collisionMode" => (self.collision_mode).inspect_value(path, offset, limit),
                "plane1" => (self.plane1).inspect_value(path, offset, limit),
                "colliderForce" => (self.collider_force).inspect_value(path, offset, limit),
                "plane2" => (self.plane2).inspect_value(path, offset, limit),
                "multiplyColliderForceByParticleSize" => (self
                    .multiply_collider_force_by_particle_size)
                    .inspect_value(path, offset, limit),
                "plane3" => (self.plane3).inspect_value(path, offset, limit),
                "multiplyColliderForceByParticleSpeed" => (self
                    .multiply_collider_force_by_particle_speed)
                    .inspect_value(path, offset, limit),
                "plane4" => (self.plane4).inspect_value(path, offset, limit),
                "multiplyColliderForceByCollisionAngle" => (self
                    .multiply_collider_force_by_collision_angle)
                    .inspect_value(path, offset, limit),
                "plane5" => (self.plane5).inspect_value(path, offset, limit),
                "m_Planes" => (self.m_planes).inspect_value(path, offset, limit),
                "dampen" => (self.dampen).inspect_value(path, offset, limit),
                "m_Dampen" => (self.m_dampen).inspect_value(path, offset, limit),
                "bounce" => (self.bounce).inspect_value(path, offset, limit),
                "m_Bounce" => (self.m_bounce).inspect_value(path, offset, limit),
                "energyLossOnCollision" => {
                    (self.energy_loss_on_collision).inspect_value(path, offset, limit)
                }
                "m_EnergyLossOnCollision" => {
                    (self.m_energy_loss_on_collision).inspect_value(path, offset, limit)
                }
                "minKillSpeed" => (self.min_kill_speed).inspect_value(path, offset, limit),
                "particleRadius" => (self.particle_radius).inspect_value(path, offset, limit),
                "maxKillSpeed" => (self.max_kill_speed).inspect_value(path, offset, limit),
                "collidesWith" => (self.collides_with).inspect_value(path, offset, limit),
                "radiusScale" => (self.radius_scale).inspect_value(path, offset, limit),
                "quality" => (self.quality).inspect_value(path, offset, limit),
                "voxelSize" => (self.voxel_size).inspect_value(path, offset, limit),
                "maxCollisionShapes" => {
                    (self.max_collision_shapes).inspect_value(path, offset, limit)
                }
                "collisionMessages" => (self.collision_messages).inspect_value(path, offset, limit),
                "collidesWithDynamic" => {
                    (self.collides_with_dynamic).inspect_value(path, offset, limit)
                }
                "interiorCollisions" => {
                    (self.interior_collisions).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.plane0).inspect_value(path, offset, limit)).map(|v| ("plane0".to_string(), v)),
            ((self.collision_mode).inspect_value(path, offset, limit))
                .map(|v| ("collisionMode".to_string(), v)),
            ((self.plane1).inspect_value(path, offset, limit)).map(|v| ("plane1".to_string(), v)),
            ((self.collider_force).inspect_value(path, offset, limit))
                .map(|v| ("colliderForce".to_string(), v)),
            ((self.plane2).inspect_value(path, offset, limit)).map(|v| ("plane2".to_string(), v)),
            ((self.multiply_collider_force_by_particle_size).inspect_value(path, offset, limit))
                .map(|v| ("multiplyColliderForceByParticleSize".to_string(), v)),
            ((self.plane3).inspect_value(path, offset, limit)).map(|v| ("plane3".to_string(), v)),
            ((self.multiply_collider_force_by_particle_speed).inspect_value(path, offset, limit))
                .map(|v| ("multiplyColliderForceByParticleSpeed".to_string(), v)),
            ((self.plane4).inspect_value(path, offset, limit)).map(|v| ("plane4".to_string(), v)),
            ((self.multiply_collider_force_by_collision_angle).inspect_value(path, offset, limit))
                .map(|v| ("multiplyColliderForceByCollisionAngle".to_string(), v)),
            ((self.plane5).inspect_value(path, offset, limit)).map(|v| ("plane5".to_string(), v)),
            ((self.m_planes).inspect_value(path, offset, limit))
                .map(|v| ("m_Planes".to_string(), v)),
            ((self.dampen).inspect_value(path, offset, limit)).map(|v| ("dampen".to_string(), v)),
            ((self.m_dampen).inspect_value(path, offset, limit))
                .map(|v| ("m_Dampen".to_string(), v)),
            ((self.bounce).inspect_value(path, offset, limit)).map(|v| ("bounce".to_string(), v)),
            ((self.m_bounce).inspect_value(path, offset, limit))
                .map(|v| ("m_Bounce".to_string(), v)),
            ((self.energy_loss_on_collision).inspect_value(path, offset, limit))
                .map(|v| ("energyLossOnCollision".to_string(), v)),
            ((self.m_energy_loss_on_collision).inspect_value(path, offset, limit))
                .map(|v| ("m_EnergyLossOnCollision".to_string(), v)),
            ((self.min_kill_speed).inspect_value(path, offset, limit))
                .map(|v| ("minKillSpeed".to_string(), v)),
            ((self.particle_radius).inspect_value(path, offset, limit))
                .map(|v| ("particleRadius".to_string(), v)),
            ((self.max_kill_speed).inspect_value(path, offset, limit))
                .map(|v| ("maxKillSpeed".to_string(), v)),
            ((self.collides_with).inspect_value(path, offset, limit))
                .map(|v| ("collidesWith".to_string(), v)),
            ((self.radius_scale).inspect_value(path, offset, limit))
                .map(|v| ("radiusScale".to_string(), v)),
            ((self.quality).inspect_value(path, offset, limit)).map(|v| ("quality".to_string(), v)),
            ((self.voxel_size).inspect_value(path, offset, limit))
                .map(|v| ("voxelSize".to_string(), v)),
            ((self.max_collision_shapes).inspect_value(path, offset, limit))
                .map(|v| ("maxCollisionShapes".to_string(), v)),
            ((self.collision_messages).inspect_value(path, offset, limit))
                .map(|v| ("collisionMessages".to_string(), v)),
            ((self.collides_with_dynamic).inspect_value(path, offset, limit))
                .map(|v| ("collidesWithDynamic".to_string(), v)),
            ((self.interior_collisions).inspect_value(path, offset, limit))
                .map(|v| ("interiorCollisions".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
