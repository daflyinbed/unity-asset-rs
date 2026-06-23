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
use super::component::Component;

#[derive(Debug, Clone)]
pub struct ParticleSystem {
    pub parent: Component,
    pub m_game_object: PPtr,
    pub length_in_sec: f32,
    pub start_delay: f32,
    pub speed: Option<f32>,
    pub simulation_speed: Option<f32>,
    pub looping: bool,
    pub stop_action: Option<i32>,
    pub random_seed: u32,
    pub prewarm: bool,
    pub ring_buffer_mode: Option<i32>,
    pub culling_mode: Option<i32>,
    pub play_on_awake: bool,
    pub ring_buffer_loop_range: Option<Vector2f>,
    pub auto_random_seed: Option<bool>,
    pub use_unscaled_time: Option<bool>,
    pub move_with_transform: bool,
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
        let parent = Component::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let length_in_sec = r.read_f32()?;
        let start_delay = r.read_f32()?;
        let speed = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let simulation_speed = if *ver >= crate::UnityVersion::new(5, 5, 1) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let looping = r.read_bool()?;
        let stop_action = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let random_seed = r.read_u32()?;
        let prewarm = r.read_bool()?;
        let ring_buffer_mode = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let culling_mode = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let play_on_awake = r.read_bool()?;
        let ring_buffer_loop_range = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let auto_random_seed = if *ver >= crate::UnityVersion::new(5, 4, 1) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let use_unscaled_time = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let move_with_transform = r.read_bool()?;
        let emitter_velocity_mode = if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let use_rigidbody_for_velocity = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let initial_module = InitialModule::parse(r, ver)?;
        r.align(4)?;
        let scaling_mode = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let move_with_custom_transform = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let shape_module = ShapeModule::parse(r, ver)?;
        r.align(4)?;
        let emission_module = EmissionModule::parse(r, ver)?;
        r.align(4)?;
        let size_module = SizeModule::parse(r, ver)?;
        r.align(4)?;
        let rotation_module = RotationModule::parse(r, ver)?;
        r.align(4)?;
        let color_module = ColorModule::parse(r, ver)?;
        r.align(4)?;
        let uv_module = UvModule::parse(r, ver)?;
        r.align(4)?;
        let velocity_module = VelocityModule::parse(r, ver)?;
        r.align(4)?;
        let force_module = ForceModule::parse(r, ver)?;
        r.align(4)?;
        let clamp_velocity_module = ClampVelocityModule::parse(r, ver)?;
        r.align(4)?;
        let external_forces_module = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = ExternalForcesModule::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let inherit_velocity_module = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = InheritVelocityModule::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let size_by_speed_module = SizeBySpeedModule::parse(r, ver)?;
        r.align(4)?;
        let rotation_by_speed_module = RotationBySpeedModule::parse(r, ver)?;
        r.align(4)?;
        let color_by_speed_module = ColorBySpeedModule::parse(r, ver)?;
        r.align(4)?;
        let collision_module = CollisionModule::parse(r, ver)?;
        r.align(4)?;
        let sub_module = SubModule::parse(r, ver)?;
        r.align(4)?;
        let noise_module = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = NoiseModule::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let trigger_module = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = TriggerModule::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let lifetime_by_emitter_speed_module = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = LifetimeByEmitterSpeedModule::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let lights_module = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = LightsModule::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let trail_module = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = TrailModule::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let custom_data_module = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = CustomDataModule::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(ParticleSystem {
            parent,
            m_game_object,
            length_in_sec,
            start_delay,
            speed,
            simulation_speed,
            looping,
            stop_action,
            random_seed,
            prewarm,
            ring_buffer_mode,
            culling_mode,
            play_on_awake,
            ring_buffer_loop_range,
            auto_random_seed,
            use_unscaled_time,
            move_with_transform,
            emitter_velocity_mode,
            use_rigidbody_for_velocity,
            initial_module,
            scaling_mode,
            move_with_custom_transform,
            shape_module,
            emission_module,
            size_module,
            rotation_module,
            color_module,
            uv_module,
            velocity_module,
            force_module,
            clamp_velocity_module,
            external_forces_module,
            inherit_velocity_module,
            size_by_speed_module,
            rotation_by_speed_module,
            color_by_speed_module,
            collision_module,
            sub_module,
            noise_module,
            trigger_module,
            lifetime_by_emitter_speed_module,
            lights_module,
            trail_module,
            custom_data_module,
        })
    }
}
