//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba32::ColorRgba32;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ParticleAnimator {
    pub m_game_object: PPtr,
    pub does_animate_color: bool,
    pub color_animation_0: ColorRgba32,
    pub color_animation_1: ColorRgba32,
    pub color_animation_2: ColorRgba32,
    pub color_animation_3: ColorRgba32,
    pub color_animation_4: ColorRgba32,
    pub world_rotation_axis: Vector3f,
    pub local_rotation_axis: Vector3f,
    pub size_grow: f32,
    pub rnd_force: Vector3f,
    pub force: Vector3f,
    pub damping: f32,
    pub stop_simulation: bool,
    pub autodestruct: bool,
}

impl crate::Parse for ParticleAnimator {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let does_animate_color = rdr.read_bool()?;
        rdr.align(4)?;
        let color_animation_0 = ColorRgba32::parse(rdr, ver)?;
        let color_animation_1 = ColorRgba32::parse(rdr, ver)?;
        let color_animation_2 = ColorRgba32::parse(rdr, ver)?;
        let color_animation_3 = ColorRgba32::parse(rdr, ver)?;
        let color_animation_4 = ColorRgba32::parse(rdr, ver)?;
        let world_rotation_axis = Vector3f::parse(rdr, ver)?;
        let local_rotation_axis = Vector3f::parse(rdr, ver)?;
        let size_grow = rdr.read_f32()?;
        let rnd_force = Vector3f::parse(rdr, ver)?;
        let force = Vector3f::parse(rdr, ver)?;
        let damping = rdr.read_f32()?;
        let stop_simulation = rdr.read_bool()?;
        let autodestruct = rdr.read_bool()?;

        Ok(ParticleAnimator {
            m_game_object,
            does_animate_color,
            color_animation_0,
            color_animation_1,
            color_animation_2,
            color_animation_3,
            color_animation_4,
            world_rotation_axis,
            local_rotation_axis,
            size_grow,
            rnd_force,
            force,
            damping,
            stop_simulation,
            autodestruct,
        })
    }
}
