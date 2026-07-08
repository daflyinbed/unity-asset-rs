//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::clamp_velocity_module::ClampVelocityModule;
use crate::subclasses::collision_module::CollisionModule;
use crate::subclasses::color_by_speed_module::ColorBySpeedModule;
use crate::subclasses::color_module::ColorModule;
use crate::subclasses::custom_data_module::CustomDataModule;
use crate::subclasses::emission_module::EmissionModule;
use crate::subclasses::external_forces_module::ExternalForcesModule;
use crate::subclasses::force_module::ForceModule;
use crate::subclasses::inherit_velocity_module::InheritVelocityModule;
use crate::subclasses::initial_module::InitialModule;
use crate::subclasses::lifetime_by_emitter_speed_module::LifetimeByEmitterSpeedModule;
use crate::subclasses::lights_module::LightsModule;
use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::noise_module::NoiseModule;
use crate::subclasses::rotation_by_speed_module::RotationBySpeedModule;
use crate::subclasses::rotation_module::RotationModule;
use crate::subclasses::shape_module::ShapeModule;
use crate::subclasses::size_by_speed_module::SizeBySpeedModule;
use crate::subclasses::size_module::SizeModule;
use crate::subclasses::sub_module::SubModule;
use crate::subclasses::trail_module::TrailModule;
use crate::subclasses::trigger_module::TriggerModule;
use crate::subclasses::uv_module::UvModule;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::velocity_module::VelocityModule;


/// 版本枚举：ParticleSystemStartDelayVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemStartDelayVariant {
    Default(f32),
    V5_3(MinMaxCurve),
}

impl crate::Parse for ParticleSystemStartDelayVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = MinMaxCurve::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V5_3(v))
        } else {
            let v = r.read_f32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleSystemRandomSeedVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemRandomSeedVariant {
    Default(u32),
    V5_4_1(i32),
}

impl crate::Parse for ParticleSystemRandomSeedVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 1) {
            let v = r.read_i32()?;
            Ok(Self::V5_4_1(v))
        } else {
            let v = r.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleSystemAutoRandomSeedVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemAutoRandomSeedVariant {
    Default(bool),
    V2017_1(bool),
    V2021_1(bool),
}

impl crate::Parse for ParticleSystemAutoRandomSeedVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V2021_1(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = r.read_bool()?;
            Ok(Self::V2017_1(v))
        } else {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleSystemMoveWithTransformVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleSystemMoveWithTransformVariant {
    Default(bool),
    V5_5(i32),
}

impl crate::Parse for ParticleSystemMoveWithTransformVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Ok(Self::V5_5(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ParticleSystem {
    pub m_game_object: PPtr,
    pub length_in_sec: f32,
    pub start_delay: ParticleSystemStartDelayVariant,
    pub speed: Option<f32>,
    pub simulation_speed: Option<f32>,
    pub looping: bool,
    pub stop_action: Option<i32>,
    pub random_seed: ParticleSystemRandomSeedVariant,
    pub prewarm: bool,
    pub culling_mode: Option<i32>,
    pub play_on_awake: bool,
    pub ring_buffer_mode: Option<i32>,
    pub auto_random_seed: Option<ParticleSystemAutoRandomSeedVariant>,
    pub use_unscaled_time: Option<bool>,
    pub ring_buffer_loop_range: Option<Vector2f>,
    pub move_with_transform: ParticleSystemMoveWithTransformVariant,
    pub emitter_velocity_mode: Option<i32>,
    pub use_rigidbody_for_velocity: Option<bool>,
    pub initial_module: InitialModule,
    pub scaling_mode: Option<i32>,
    pub move_with_custom_transform: Option<PPtr>,
    pub shape_module: ShapeModule,
    pub emission_module: EmissionModule,
    pub size_module: SizeModule,
    pub rotation_module: RotationModule,
    pub color_module: ColorModule,
    pub uv_module: UvModule,
    pub velocity_module: VelocityModule,
    pub force_module: ForceModule,
    pub clamp_velocity_module: ClampVelocityModule,
    pub external_forces_module: Option<ExternalForcesModule>,
    pub inherit_velocity_module: Option<InheritVelocityModule>,
    pub size_by_speed_module: SizeBySpeedModule,
    pub rotation_by_speed_module: RotationBySpeedModule,
    pub color_by_speed_module: ColorBySpeedModule,
    pub collision_module: CollisionModule,
    pub sub_module: SubModule,
    pub noise_module: Option<NoiseModule>,
    pub trigger_module: Option<TriggerModule>,
    pub lifetime_by_emitter_speed_module: Option<LifetimeByEmitterSpeedModule>,
    pub lights_module: Option<LightsModule>,
    pub trail_module: Option<TrailModule>,
    pub custom_data_module: Option<CustomDataModule>,
}

impl crate::Parse for ParticleSystem {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut length_in_sec = None;
        let mut start_delay = None;
        let mut speed = None;
        let mut simulation_speed = None;
        let mut looping = None;
        let mut stop_action = None;
        let mut random_seed = None;
        let mut prewarm = None;
        let mut culling_mode = None;
        let mut play_on_awake = None;
        let mut ring_buffer_mode = None;
        let mut auto_random_seed = None;
        let mut use_unscaled_time = None;
        let mut ring_buffer_loop_range = None;
        let mut move_with_transform = None;
        let mut emitter_velocity_mode = None;
        let mut use_rigidbody_for_velocity = None;
        let mut initial_module = None;
        let mut scaling_mode = None;
        let mut move_with_custom_transform = None;
        let mut shape_module = None;
        let mut emission_module = None;
        let mut size_module = None;
        let mut rotation_module = None;
        let mut color_module = None;
        let mut uv_module = None;
        let mut velocity_module = None;
        let mut force_module = None;
        let mut clamp_velocity_module = None;
        let mut external_forces_module = None;
        let mut inherit_velocity_module = None;
        let mut size_by_speed_module = None;
        let mut rotation_by_speed_module = None;
        let mut color_by_speed_module = None;
        let mut collision_module = None;
        let mut sub_module = None;
        let mut noise_module = None;
        let mut trigger_module = None;
        let mut lifetime_by_emitter_speed_module = None;
        let mut lights_module = None;
        let mut trail_module = None;
        let mut custom_data_module = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            length_in_sec = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            simulation_speed = Some(r.read_f32()?);
            stop_action = Some(r.read_i32()?);
            culling_mode = Some(r.read_i32()?);
            ring_buffer_mode = Some(r.read_i32()?);
            ring_buffer_loop_range = Some(Vector2f::parse(r, ver)?);
            emitter_velocity_mode = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            simulation_speed = Some(r.read_f32()?);
            stop_action = Some(r.read_i32()?);
            culling_mode = Some(r.read_i32()?);
            ring_buffer_mode = Some(r.read_i32()?);
            ring_buffer_loop_range = Some(Vector2f::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            simulation_speed = Some(r.read_f32()?);
            stop_action = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 1) {
            simulation_speed = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            speed = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 1) {
            start_delay = Some(ParticleSystemStartDelayVariant::parse(r, ver)?);
            speed = Some(r.read_f32()?);
        } else {
            start_delay = Some(ParticleSystemStartDelayVariant::parse(r, ver)?);
            speed = Some(r.read_f32()?);
            random_seed = Some(ParticleSystemRandomSeedVariant::parse(r, ver)?);
        }
            looping = Some(r.read_bool()?);
            prewarm = Some(r.read_bool()?);
            play_on_awake = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            use_unscaled_time = Some(r.read_bool()?);
            auto_random_seed = Some(ParticleSystemAutoRandomSeedVariant::parse(r, ver)?);
            start_delay = Some(ParticleSystemStartDelayVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            use_unscaled_time = Some(r.read_bool()?);
            auto_random_seed = Some(ParticleSystemAutoRandomSeedVariant::parse(r, ver)?);
            use_rigidbody_for_velocity = Some(r.read_bool()?);
            r.align(4)?;
            start_delay = Some(ParticleSystemStartDelayVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            auto_random_seed = Some(ParticleSystemAutoRandomSeedVariant::parse(r, ver)?);
            start_delay = Some(ParticleSystemStartDelayVariant::parse(r, ver)?);
        } else {
        }
            move_with_transform = Some(ParticleSystemMoveWithTransformVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            move_with_custom_transform = Some(crate::PPtr::parse(r, ver)?);
            scaling_mode = Some(r.read_i32()?);
            random_seed = Some(ParticleSystemRandomSeedVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 1) {
            auto_random_seed = Some(ParticleSystemAutoRandomSeedVariant::parse(r, ver)?);
            scaling_mode = Some(r.read_i32()?);
            random_seed = Some(ParticleSystemRandomSeedVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            scaling_mode = Some(r.read_i32()?);
        } else {
        }
            initial_module = Some(InitialModule::parse(r, ver)?);
            r.align(4)?;
            shape_module = Some(ShapeModule::parse(r, ver)?);
            r.align(4)?;
            emission_module = Some(EmissionModule::parse(r, ver)?);
            r.align(4)?;
            size_module = Some(SizeModule::parse(r, ver)?);
            r.align(4)?;
            rotation_module = Some(RotationModule::parse(r, ver)?);
            r.align(4)?;
            color_module = Some(ColorModule::parse(r, ver)?);
            r.align(4)?;
            uv_module = Some(UvModule::parse(r, ver)?);
            r.align(4)?;
            velocity_module = Some(VelocityModule::parse(r, ver)?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            inherit_velocity_module = Some(InheritVelocityModule::parse(r, ver)?);
            r.align(4)?;
            lifetime_by_emitter_speed_module = Some(LifetimeByEmitterSpeedModule::parse(r, ver)?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            inherit_velocity_module = Some(InheritVelocityModule::parse(r, ver)?);
            r.align(4)?;
        } else {
        }
            force_module = Some(ForceModule::parse(r, ver)?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            external_forces_module = Some(ExternalForcesModule::parse(r, ver)?);
            r.align(4)?;
        } else {
        }
            clamp_velocity_module = Some(ClampVelocityModule::parse(r, ver)?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            noise_module = Some(NoiseModule::parse(r, ver)?);
            r.align(4)?;
        } else {
        }
            size_by_speed_module = Some(SizeBySpeedModule::parse(r, ver)?);
            r.align(4)?;
            rotation_by_speed_module = Some(RotationBySpeedModule::parse(r, ver)?);
            r.align(4)?;
            color_by_speed_module = Some(ColorBySpeedModule::parse(r, ver)?);
            r.align(4)?;
            collision_module = Some(CollisionModule::parse(r, ver)?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            trigger_module = Some(TriggerModule::parse(r, ver)?);
            r.align(4)?;
        } else {
        }
            sub_module = Some(SubModule::parse(r, ver)?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            lights_module = Some(LightsModule::parse(r, ver)?);
            r.align(4)?;
            trail_module = Some(TrailModule::parse(r, ver)?);
            r.align(4)?;
            custom_data_module = Some(CustomDataModule::parse(r, ver)?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            lights_module = Some(LightsModule::parse(r, ver)?);
            r.align(4)?;
            trail_module = Some(TrailModule::parse(r, ver)?);
            r.align(4)?;
        } else {
        }

        Ok(ParticleSystem {
            m_game_object: m_game_object.unwrap(),
            length_in_sec: length_in_sec.unwrap(),
            start_delay: start_delay.unwrap(),
            speed,
            simulation_speed,
            looping: looping.unwrap(),
            stop_action,
            random_seed: random_seed.unwrap(),
            prewarm: prewarm.unwrap(),
            culling_mode,
            play_on_awake: play_on_awake.unwrap(),
            ring_buffer_mode,
            auto_random_seed,
            use_unscaled_time,
            ring_buffer_loop_range,
            move_with_transform: move_with_transform.unwrap(),
            emitter_velocity_mode,
            use_rigidbody_for_velocity,
            initial_module: initial_module.unwrap(),
            scaling_mode,
            move_with_custom_transform,
            shape_module: shape_module.unwrap(),
            emission_module: emission_module.unwrap(),
            size_module: size_module.unwrap(),
            rotation_module: rotation_module.unwrap(),
            color_module: color_module.unwrap(),
            uv_module: uv_module.unwrap(),
            velocity_module: velocity_module.unwrap(),
            force_module: force_module.unwrap(),
            clamp_velocity_module: clamp_velocity_module.unwrap(),
            external_forces_module,
            inherit_velocity_module,
            size_by_speed_module: size_by_speed_module.unwrap(),
            rotation_by_speed_module: rotation_by_speed_module.unwrap(),
            color_by_speed_module: color_by_speed_module.unwrap(),
            collision_module: collision_module.unwrap(),
            sub_module: sub_module.unwrap(),
            noise_module,
            trigger_module,
            lifetime_by_emitter_speed_module,
            lights_module,
            trail_module,
            custom_data_module,
        })
    }
}
