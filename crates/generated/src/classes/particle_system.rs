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
    V5_3(Box<MinMaxCurve>),
    V5_5(Box<MinMaxCurve>),
}

impl crate::Parse for ParticleSystemStartDelayVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = ({ Ok(Box::new(MinMaxCurve::parse(rdr, ver)?)) })?;
            rdr.align(4)?;
            Ok(Self::V5_5(v))
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = ({ Ok(Box::new(MinMaxCurve::parse(rdr, ver)?)) })?;
            Ok(Self::V5_3(v))
        } else {
            let v = rdr.read_f32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ParticleSystemStartDelayVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_3(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 1) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_4_1(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ParticleSystemRandomSeedVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4_1(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2021_1(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ParticleSystemAutoRandomSeedVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
            Self::V2021_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_1(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ParticleSystemMoveWithTransformVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
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
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        length_in_sec = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            simulation_speed = Some(rdr.read_f32()?);
            stop_action = Some(rdr.read_i32()?);
            culling_mode = Some(rdr.read_i32()?);
            ring_buffer_mode = Some(rdr.read_i32()?);
            ring_buffer_loop_range = Some(Vector2f::parse(rdr, ver)?);
            emitter_velocity_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            simulation_speed = Some(rdr.read_f32()?);
            stop_action = Some(rdr.read_i32()?);
            culling_mode = Some(rdr.read_i32()?);
            ring_buffer_mode = Some(rdr.read_i32()?);
            ring_buffer_loop_range = Some(Vector2f::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            simulation_speed = Some(rdr.read_f32()?);
            stop_action = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 1) {
            simulation_speed = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            speed = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 1) {
            start_delay = Some(ParticleSystemStartDelayVariant::parse(rdr, ver)?);
            speed = Some(rdr.read_f32()?);
        } else {
            start_delay = Some(ParticleSystemStartDelayVariant::parse(rdr, ver)?);
            speed = Some(rdr.read_f32()?);
            random_seed = Some(ParticleSystemRandomSeedVariant::parse(rdr, ver)?);
        }
        looping = Some(rdr.read_bool()?);
        prewarm = Some(rdr.read_bool()?);
        play_on_awake = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            use_unscaled_time = Some(rdr.read_bool()?);
            auto_random_seed = Some(ParticleSystemAutoRandomSeedVariant::parse(rdr, ver)?);
            start_delay = Some(ParticleSystemStartDelayVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            use_unscaled_time = Some(rdr.read_bool()?);
            auto_random_seed = Some(ParticleSystemAutoRandomSeedVariant::parse(rdr, ver)?);
            use_rigidbody_for_velocity = Some(rdr.read_bool()?);
            rdr.align(4)?;
            start_delay = Some(ParticleSystemStartDelayVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            auto_random_seed = Some(ParticleSystemAutoRandomSeedVariant::parse(rdr, ver)?);
            start_delay = Some(ParticleSystemStartDelayVariant::parse(rdr, ver)?);
        } else {
        }
        move_with_transform = Some(ParticleSystemMoveWithTransformVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            move_with_custom_transform = Some(crate::PPtr::parse(rdr, ver)?);
            scaling_mode = Some(rdr.read_i32()?);
            random_seed = Some(ParticleSystemRandomSeedVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 1) {
            auto_random_seed = Some(ParticleSystemAutoRandomSeedVariant::parse(rdr, ver)?);
            scaling_mode = Some(rdr.read_i32()?);
            random_seed = Some(ParticleSystemRandomSeedVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            scaling_mode = Some(rdr.read_i32()?);
        } else {
        }
        initial_module = Some(InitialModule::parse(rdr, ver)?);
        shape_module = Some(ShapeModule::parse(rdr, ver)?);
        emission_module = Some(EmissionModule::parse(rdr, ver)?);
        size_module = Some(SizeModule::parse(rdr, ver)?);
        rotation_module = Some(RotationModule::parse(rdr, ver)?);
        color_module = Some(ColorModule::parse(rdr, ver)?);
        uv_module = Some(UvModule::parse(rdr, ver)?);
        velocity_module = Some(VelocityModule::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            inherit_velocity_module = Some(InheritVelocityModule::parse(rdr, ver)?);
            lifetime_by_emitter_speed_module = Some(LifetimeByEmitterSpeedModule::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            inherit_velocity_module = Some(InheritVelocityModule::parse(rdr, ver)?);
        } else {
        }
        force_module = Some(ForceModule::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            external_forces_module = Some(ExternalForcesModule::parse(rdr, ver)?);
        } else {
        }
        clamp_velocity_module = Some(ClampVelocityModule::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            noise_module = Some(NoiseModule::parse(rdr, ver)?);
        } else {
        }
        size_by_speed_module = Some(SizeBySpeedModule::parse(rdr, ver)?);
        rotation_by_speed_module = Some(RotationBySpeedModule::parse(rdr, ver)?);
        color_by_speed_module = Some(ColorBySpeedModule::parse(rdr, ver)?);
        collision_module = Some(CollisionModule::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            trigger_module = Some(TriggerModule::parse(rdr, ver)?);
        } else {
        }
        sub_module = Some(SubModule::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            lights_module = Some(LightsModule::parse(rdr, ver)?);
            trail_module = Some(TrailModule::parse(rdr, ver)?);
            custom_data_module = Some(CustomDataModule::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            lights_module = Some(LightsModule::parse(rdr, ver)?);
            trail_module = Some(TrailModule::parse(rdr, ver)?);
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

impl crate::ToValue for ParticleSystem {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("lengthInSec".to_string(), self.length_in_sec.to_value())),
            Some(("startDelay".to_string(), self.start_delay.to_value())),
            crate::opt_field("speed", &self.speed),
            crate::opt_field("simulationSpeed", &self.simulation_speed),
            Some(("looping".to_string(), self.looping.to_value())),
            crate::opt_field("stopAction", &self.stop_action),
            Some(("randomSeed".to_string(), self.random_seed.to_value())),
            Some(("prewarm".to_string(), self.prewarm.to_value())),
            crate::opt_field("cullingMode", &self.culling_mode),
            Some(("playOnAwake".to_string(), self.play_on_awake.to_value())),
            crate::opt_field("ringBufferMode", &self.ring_buffer_mode),
            crate::opt_field("autoRandomSeed", &self.auto_random_seed),
            crate::opt_field("useUnscaledTime", &self.use_unscaled_time),
            crate::opt_field("ringBufferLoopRange", &self.ring_buffer_loop_range),
            Some((
                "moveWithTransform".to_string(),
                self.move_with_transform.to_value(),
            )),
            crate::opt_field("emitterVelocityMode", &self.emitter_velocity_mode),
            crate::opt_field("useRigidbodyForVelocity", &self.use_rigidbody_for_velocity),
            Some(("InitialModule".to_string(), self.initial_module.to_value())),
            crate::opt_field("scalingMode", &self.scaling_mode),
            crate::opt_field("moveWithCustomTransform", &self.move_with_custom_transform),
            Some(("ShapeModule".to_string(), self.shape_module.to_value())),
            Some((
                "EmissionModule".to_string(),
                self.emission_module.to_value(),
            )),
            Some(("SizeModule".to_string(), self.size_module.to_value())),
            Some((
                "RotationModule".to_string(),
                self.rotation_module.to_value(),
            )),
            Some(("ColorModule".to_string(), self.color_module.to_value())),
            Some(("UVModule".to_string(), self.uv_module.to_value())),
            Some((
                "VelocityModule".to_string(),
                self.velocity_module.to_value(),
            )),
            Some(("ForceModule".to_string(), self.force_module.to_value())),
            Some((
                "ClampVelocityModule".to_string(),
                self.clamp_velocity_module.to_value(),
            )),
            crate::opt_field("ExternalForcesModule", &self.external_forces_module),
            crate::opt_field("InheritVelocityModule", &self.inherit_velocity_module),
            Some((
                "SizeBySpeedModule".to_string(),
                self.size_by_speed_module.to_value(),
            )),
            Some((
                "RotationBySpeedModule".to_string(),
                self.rotation_by_speed_module.to_value(),
            )),
            Some((
                "ColorBySpeedModule".to_string(),
                self.color_by_speed_module.to_value(),
            )),
            Some((
                "CollisionModule".to_string(),
                self.collision_module.to_value(),
            )),
            Some(("SubModule".to_string(), self.sub_module.to_value())),
            crate::opt_field("NoiseModule", &self.noise_module),
            crate::opt_field("TriggerModule", &self.trigger_module),
            crate::opt_field(
                "LifetimeByEmitterSpeedModule",
                &self.lifetime_by_emitter_speed_module,
            ),
            crate::opt_field("LightsModule", &self.lights_module),
            crate::opt_field("TrailModule", &self.trail_module),
            crate::opt_field("CustomDataModule", &self.custom_data_module),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "lengthInSec" => (self.length_in_sec).inspect_value(path, offset, limit),
                "startDelay" => (self.start_delay).inspect_value(path, offset, limit),
                "speed" => (self.speed).inspect_value(path, offset, limit),
                "simulationSpeed" => (self.simulation_speed).inspect_value(path, offset, limit),
                "looping" => (self.looping).inspect_value(path, offset, limit),
                "stopAction" => (self.stop_action).inspect_value(path, offset, limit),
                "randomSeed" => (self.random_seed).inspect_value(path, offset, limit),
                "prewarm" => (self.prewarm).inspect_value(path, offset, limit),
                "cullingMode" => (self.culling_mode).inspect_value(path, offset, limit),
                "playOnAwake" => (self.play_on_awake).inspect_value(path, offset, limit),
                "ringBufferMode" => (self.ring_buffer_mode).inspect_value(path, offset, limit),
                "autoRandomSeed" => (self.auto_random_seed).inspect_value(path, offset, limit),
                "useUnscaledTime" => (self.use_unscaled_time).inspect_value(path, offset, limit),
                "ringBufferLoopRange" => {
                    (self.ring_buffer_loop_range).inspect_value(path, offset, limit)
                }
                "moveWithTransform" => {
                    (self.move_with_transform).inspect_value(path, offset, limit)
                }
                "emitterVelocityMode" => {
                    (self.emitter_velocity_mode).inspect_value(path, offset, limit)
                }
                "useRigidbodyForVelocity" => {
                    (self.use_rigidbody_for_velocity).inspect_value(path, offset, limit)
                }
                "InitialModule" => (self.initial_module).inspect_value(path, offset, limit),
                "scalingMode" => (self.scaling_mode).inspect_value(path, offset, limit),
                "moveWithCustomTransform" => {
                    (self.move_with_custom_transform).inspect_value(path, offset, limit)
                }
                "ShapeModule" => (self.shape_module).inspect_value(path, offset, limit),
                "EmissionModule" => (self.emission_module).inspect_value(path, offset, limit),
                "SizeModule" => (self.size_module).inspect_value(path, offset, limit),
                "RotationModule" => (self.rotation_module).inspect_value(path, offset, limit),
                "ColorModule" => (self.color_module).inspect_value(path, offset, limit),
                "UVModule" => (self.uv_module).inspect_value(path, offset, limit),
                "VelocityModule" => (self.velocity_module).inspect_value(path, offset, limit),
                "ForceModule" => (self.force_module).inspect_value(path, offset, limit),
                "ClampVelocityModule" => {
                    (self.clamp_velocity_module).inspect_value(path, offset, limit)
                }
                "ExternalForcesModule" => {
                    (self.external_forces_module).inspect_value(path, offset, limit)
                }
                "InheritVelocityModule" => {
                    (self.inherit_velocity_module).inspect_value(path, offset, limit)
                }
                "SizeBySpeedModule" => {
                    (self.size_by_speed_module).inspect_value(path, offset, limit)
                }
                "RotationBySpeedModule" => {
                    (self.rotation_by_speed_module).inspect_value(path, offset, limit)
                }
                "ColorBySpeedModule" => {
                    (self.color_by_speed_module).inspect_value(path, offset, limit)
                }
                "CollisionModule" => (self.collision_module).inspect_value(path, offset, limit),
                "SubModule" => (self.sub_module).inspect_value(path, offset, limit),
                "NoiseModule" => (self.noise_module).inspect_value(path, offset, limit),
                "TriggerModule" => (self.trigger_module).inspect_value(path, offset, limit),
                "LifetimeByEmitterSpeedModule" => {
                    (self.lifetime_by_emitter_speed_module).inspect_value(path, offset, limit)
                }
                "LightsModule" => (self.lights_module).inspect_value(path, offset, limit),
                "TrailModule" => (self.trail_module).inspect_value(path, offset, limit),
                "CustomDataModule" => (self.custom_data_module).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.length_in_sec).inspect_value(path, offset, limit))
                .map(|v| ("lengthInSec".to_string(), v)),
            ((self.start_delay).inspect_value(path, offset, limit))
                .map(|v| ("startDelay".to_string(), v)),
            ((self.speed).inspect_value(path, offset, limit)).map(|v| ("speed".to_string(), v)),
            ((self.simulation_speed).inspect_value(path, offset, limit))
                .map(|v| ("simulationSpeed".to_string(), v)),
            ((self.looping).inspect_value(path, offset, limit)).map(|v| ("looping".to_string(), v)),
            ((self.stop_action).inspect_value(path, offset, limit))
                .map(|v| ("stopAction".to_string(), v)),
            ((self.random_seed).inspect_value(path, offset, limit))
                .map(|v| ("randomSeed".to_string(), v)),
            ((self.prewarm).inspect_value(path, offset, limit)).map(|v| ("prewarm".to_string(), v)),
            ((self.culling_mode).inspect_value(path, offset, limit))
                .map(|v| ("cullingMode".to_string(), v)),
            ((self.play_on_awake).inspect_value(path, offset, limit))
                .map(|v| ("playOnAwake".to_string(), v)),
            ((self.ring_buffer_mode).inspect_value(path, offset, limit))
                .map(|v| ("ringBufferMode".to_string(), v)),
            ((self.auto_random_seed).inspect_value(path, offset, limit))
                .map(|v| ("autoRandomSeed".to_string(), v)),
            ((self.use_unscaled_time).inspect_value(path, offset, limit))
                .map(|v| ("useUnscaledTime".to_string(), v)),
            ((self.ring_buffer_loop_range).inspect_value(path, offset, limit))
                .map(|v| ("ringBufferLoopRange".to_string(), v)),
            ((self.move_with_transform).inspect_value(path, offset, limit))
                .map(|v| ("moveWithTransform".to_string(), v)),
            ((self.emitter_velocity_mode).inspect_value(path, offset, limit))
                .map(|v| ("emitterVelocityMode".to_string(), v)),
            ((self.use_rigidbody_for_velocity).inspect_value(path, offset, limit))
                .map(|v| ("useRigidbodyForVelocity".to_string(), v)),
            ((self.initial_module).inspect_value(path, offset, limit))
                .map(|v| ("InitialModule".to_string(), v)),
            ((self.scaling_mode).inspect_value(path, offset, limit))
                .map(|v| ("scalingMode".to_string(), v)),
            ((self.move_with_custom_transform).inspect_value(path, offset, limit))
                .map(|v| ("moveWithCustomTransform".to_string(), v)),
            ((self.shape_module).inspect_value(path, offset, limit))
                .map(|v| ("ShapeModule".to_string(), v)),
            ((self.emission_module).inspect_value(path, offset, limit))
                .map(|v| ("EmissionModule".to_string(), v)),
            ((self.size_module).inspect_value(path, offset, limit))
                .map(|v| ("SizeModule".to_string(), v)),
            ((self.rotation_module).inspect_value(path, offset, limit))
                .map(|v| ("RotationModule".to_string(), v)),
            ((self.color_module).inspect_value(path, offset, limit))
                .map(|v| ("ColorModule".to_string(), v)),
            ((self.uv_module).inspect_value(path, offset, limit))
                .map(|v| ("UVModule".to_string(), v)),
            ((self.velocity_module).inspect_value(path, offset, limit))
                .map(|v| ("VelocityModule".to_string(), v)),
            ((self.force_module).inspect_value(path, offset, limit))
                .map(|v| ("ForceModule".to_string(), v)),
            ((self.clamp_velocity_module).inspect_value(path, offset, limit))
                .map(|v| ("ClampVelocityModule".to_string(), v)),
            ((self.external_forces_module).inspect_value(path, offset, limit))
                .map(|v| ("ExternalForcesModule".to_string(), v)),
            ((self.inherit_velocity_module).inspect_value(path, offset, limit))
                .map(|v| ("InheritVelocityModule".to_string(), v)),
            ((self.size_by_speed_module).inspect_value(path, offset, limit))
                .map(|v| ("SizeBySpeedModule".to_string(), v)),
            ((self.rotation_by_speed_module).inspect_value(path, offset, limit))
                .map(|v| ("RotationBySpeedModule".to_string(), v)),
            ((self.color_by_speed_module).inspect_value(path, offset, limit))
                .map(|v| ("ColorBySpeedModule".to_string(), v)),
            ((self.collision_module).inspect_value(path, offset, limit))
                .map(|v| ("CollisionModule".to_string(), v)),
            ((self.sub_module).inspect_value(path, offset, limit))
                .map(|v| ("SubModule".to_string(), v)),
            ((self.noise_module).inspect_value(path, offset, limit))
                .map(|v| ("NoiseModule".to_string(), v)),
            ((self.trigger_module).inspect_value(path, offset, limit))
                .map(|v| ("TriggerModule".to_string(), v)),
            ((self.lifetime_by_emitter_speed_module).inspect_value(path, offset, limit))
                .map(|v| ("LifetimeByEmitterSpeedModule".to_string(), v)),
            ((self.lights_module).inspect_value(path, offset, limit))
                .map(|v| ("LightsModule".to_string(), v)),
            ((self.trail_module).inspect_value(path, offset, limit))
                .map(|v| ("TrailModule".to_string(), v)),
            ((self.custom_data_module).inspect_value(path, offset, limit))
                .map(|v| ("CustomDataModule".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
