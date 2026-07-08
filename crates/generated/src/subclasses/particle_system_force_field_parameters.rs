//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct ParticleSystemForceFieldParameters {
    pub m_shape: i32,
    pub m_start_range: f32,
    pub m_end_range: f32,
    pub m_length: f32,
    pub m_gravity_focus: f32,
    pub m_rotation_randomness: Vector2f,
    pub m_direction_curve_x: MinMaxCurve,
    pub m_direction_curve_y: MinMaxCurve,
    pub m_direction_curve_z: MinMaxCurve,
    pub m_gravity_curve: MinMaxCurve,
    pub m_rotation_speed_curve: MinMaxCurve,
    pub m_rotation_attraction_curve: MinMaxCurve,
    pub m_drag_curve: MinMaxCurve,
    pub m_vector_field: PPtr,
    pub m_vector_field_speed_curve: MinMaxCurve,
    pub m_vector_field_attraction_curve: MinMaxCurve,
    pub m_multiply_drag_by_particle_size: bool,
    pub m_multiply_drag_by_particle_velocity: bool,
}

impl crate::Parse for ParticleSystemForceFieldParameters {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_shape = rdr.read_i32()?;
        let m_start_range = rdr.read_f32()?;
        let m_end_range = rdr.read_f32()?;
        let m_length = rdr.read_f32()?;
        let m_gravity_focus = rdr.read_f32()?;
        let m_rotation_randomness = Vector2f::parse(rdr, ver)?;
        let m_direction_curve_x = MinMaxCurve::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_direction_curve_y = MinMaxCurve::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_direction_curve_z = MinMaxCurve::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_gravity_curve = MinMaxCurve::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_rotation_speed_curve = MinMaxCurve::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_rotation_attraction_curve = MinMaxCurve::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_drag_curve = MinMaxCurve::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_vector_field = crate::PPtr::parse(rdr, ver)?;
        let m_vector_field_speed_curve = MinMaxCurve::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_vector_field_attraction_curve = MinMaxCurve::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_multiply_drag_by_particle_size = rdr.read_bool()?;
        let m_multiply_drag_by_particle_velocity = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(ParticleSystemForceFieldParameters {
            m_shape,
            m_start_range,
            m_end_range,
            m_length,
            m_gravity_focus,
            m_rotation_randomness,
            m_direction_curve_x,
            m_direction_curve_y,
            m_direction_curve_z,
            m_gravity_curve,
            m_rotation_speed_curve,
            m_rotation_attraction_curve,
            m_drag_curve,
            m_vector_field,
            m_vector_field_speed_curve,
            m_vector_field_attraction_curve,
            m_multiply_drag_by_particle_size,
            m_multiply_drag_by_particle_velocity,
        })
    }
}
