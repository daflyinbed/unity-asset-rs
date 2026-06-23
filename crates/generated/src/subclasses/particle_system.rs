//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::clamp_velocity_module::ClampVelocityModule;
use crate::subclasses::collision_module::CollisionModule;
use crate::subclasses::color_by_speed_module::ColorBySpeedModule;
use crate::subclasses::color_module::ColorModule;
use crate::subclasses::emission_module::EmissionModule;
use crate::subclasses::force_module::ForceModule;
use crate::subclasses::initial_module::InitialModule;
use crate::subclasses::rotation_by_speed_module::RotationBySpeedModule;
use crate::subclasses::rotation_module::RotationModule;
use crate::subclasses::shape_module::ShapeModule;
use crate::subclasses::size_by_speed_module::SizeBySpeedModule;
use crate::subclasses::size_module::SizeModule;
use crate::subclasses::sub_module::SubModule;
use crate::subclasses::uv_module::UvModule;
use crate::subclasses::velocity_module::VelocityModule;

#[derive(Debug, Clone)]
pub struct ParticleSystem {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub length_in_sec: f32,
    pub start_delay: f32,
    pub speed: f32,
    pub random_seed: u32,
    pub looping: bool,
    pub prewarm: bool,
    pub play_on_awake: bool,
    pub move_with_transform: bool,
    pub initial_module: InitialModule,
    pub shape_module: ShapeModule,
    pub emission_module: EmissionModule,
    pub size_module: SizeModule,
    pub rotation_module: RotationModule,
    pub color_module: ColorModule,
    pub uv_module: UvModule,
    pub velocity_module: VelocityModule,
    pub force_module: ForceModule,
    pub clamp_velocity_module: ClampVelocityModule,
    pub size_by_speed_module: SizeBySpeedModule,
    pub rotation_by_speed_module: RotationBySpeedModule,
    pub color_by_speed_module: ColorBySpeedModule,
    pub collision_module: CollisionModule,
    pub sub_module: SubModule,
}

impl crate::Parse for ParticleSystem {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let length_in_sec = r.read_f32()?;
        let start_delay = r.read_f32()?;
        let speed = r.read_f32()?;
        let random_seed = r.read_u32()?;
        let looping = r.read_bool()?;
        let prewarm = r.read_bool()?;
        let play_on_awake = r.read_bool()?;
        let move_with_transform = r.read_bool()?;
        let initial_module = InitialModule::parse(r, ver)?;
        r.align(4)?;
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

        Ok(ParticleSystem {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            length_in_sec,
            start_delay,
            speed,
            random_seed,
            looping,
            prewarm,
            play_on_awake,
            move_with_transform,
            initial_module,
            shape_module,
            emission_module,
            size_module,
            rotation_module,
            color_module,
            uv_module,
            velocity_module,
            force_module,
            clamp_velocity_module,
            size_by_speed_module,
            rotation_by_speed_module,
            color_by_speed_module,
            collision_module,
            sub_module,
        })
    }
}
