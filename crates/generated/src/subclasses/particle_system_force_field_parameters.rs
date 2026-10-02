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
        let m_direction_curve_y = MinMaxCurve::parse(rdr, ver)?;
        let m_direction_curve_z = MinMaxCurve::parse(rdr, ver)?;
        let m_gravity_curve = MinMaxCurve::parse(rdr, ver)?;
        let m_rotation_speed_curve = MinMaxCurve::parse(rdr, ver)?;
        let m_rotation_attraction_curve = MinMaxCurve::parse(rdr, ver)?;
        let m_drag_curve = MinMaxCurve::parse(rdr, ver)?;
        let m_vector_field = crate::PPtr::parse(rdr, ver)?;
        let m_vector_field_speed_curve = MinMaxCurve::parse(rdr, ver)?;
        let m_vector_field_attraction_curve = MinMaxCurve::parse(rdr, ver)?;
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

impl crate::ToValue for ParticleSystemForceFieldParameters {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Shape".to_string(), self.m_shape.to_value())),
            Some(("m_StartRange".to_string(), self.m_start_range.to_value())),
            Some(("m_EndRange".to_string(), self.m_end_range.to_value())),
            Some(("m_Length".to_string(), self.m_length.to_value())),
            Some((
                "m_GravityFocus".to_string(),
                self.m_gravity_focus.to_value(),
            )),
            Some((
                "m_RotationRandomness".to_string(),
                self.m_rotation_randomness.to_value(),
            )),
            Some((
                "m_DirectionCurveX".to_string(),
                self.m_direction_curve_x.to_value(),
            )),
            Some((
                "m_DirectionCurveY".to_string(),
                self.m_direction_curve_y.to_value(),
            )),
            Some((
                "m_DirectionCurveZ".to_string(),
                self.m_direction_curve_z.to_value(),
            )),
            Some((
                "m_GravityCurve".to_string(),
                self.m_gravity_curve.to_value(),
            )),
            Some((
                "m_RotationSpeedCurve".to_string(),
                self.m_rotation_speed_curve.to_value(),
            )),
            Some((
                "m_RotationAttractionCurve".to_string(),
                self.m_rotation_attraction_curve.to_value(),
            )),
            Some(("m_DragCurve".to_string(), self.m_drag_curve.to_value())),
            Some(("m_VectorField".to_string(), self.m_vector_field.to_value())),
            Some((
                "m_VectorFieldSpeedCurve".to_string(),
                self.m_vector_field_speed_curve.to_value(),
            )),
            Some((
                "m_VectorFieldAttractionCurve".to_string(),
                self.m_vector_field_attraction_curve.to_value(),
            )),
            Some((
                "m_MultiplyDragByParticleSize".to_string(),
                self.m_multiply_drag_by_particle_size.to_value(),
            )),
            Some((
                "m_MultiplyDragByParticleVelocity".to_string(),
                self.m_multiply_drag_by_particle_velocity.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Shape" => (self.m_shape).inspect_value(path, offset, limit),
                "m_StartRange" => (self.m_start_range).inspect_value(path, offset, limit),
                "m_EndRange" => (self.m_end_range).inspect_value(path, offset, limit),
                "m_Length" => (self.m_length).inspect_value(path, offset, limit),
                "m_GravityFocus" => (self.m_gravity_focus).inspect_value(path, offset, limit),
                "m_RotationRandomness" => {
                    (self.m_rotation_randomness).inspect_value(path, offset, limit)
                }
                "m_DirectionCurveX" => {
                    (self.m_direction_curve_x).inspect_value(path, offset, limit)
                }
                "m_DirectionCurveY" => {
                    (self.m_direction_curve_y).inspect_value(path, offset, limit)
                }
                "m_DirectionCurveZ" => {
                    (self.m_direction_curve_z).inspect_value(path, offset, limit)
                }
                "m_GravityCurve" => (self.m_gravity_curve).inspect_value(path, offset, limit),
                "m_RotationSpeedCurve" => {
                    (self.m_rotation_speed_curve).inspect_value(path, offset, limit)
                }
                "m_RotationAttractionCurve" => {
                    (self.m_rotation_attraction_curve).inspect_value(path, offset, limit)
                }
                "m_DragCurve" => (self.m_drag_curve).inspect_value(path, offset, limit),
                "m_VectorField" => (self.m_vector_field).inspect_value(path, offset, limit),
                "m_VectorFieldSpeedCurve" => {
                    (self.m_vector_field_speed_curve).inspect_value(path, offset, limit)
                }
                "m_VectorFieldAttractionCurve" => {
                    (self.m_vector_field_attraction_curve).inspect_value(path, offset, limit)
                }
                "m_MultiplyDragByParticleSize" => {
                    (self.m_multiply_drag_by_particle_size).inspect_value(path, offset, limit)
                }
                "m_MultiplyDragByParticleVelocity" => {
                    (self.m_multiply_drag_by_particle_velocity).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_shape).inspect_value(path, offset, limit)).map(|v| ("m_Shape".to_string(), v)),
            ((self.m_start_range).inspect_value(path, offset, limit))
                .map(|v| ("m_StartRange".to_string(), v)),
            ((self.m_end_range).inspect_value(path, offset, limit))
                .map(|v| ("m_EndRange".to_string(), v)),
            ((self.m_length).inspect_value(path, offset, limit))
                .map(|v| ("m_Length".to_string(), v)),
            ((self.m_gravity_focus).inspect_value(path, offset, limit))
                .map(|v| ("m_GravityFocus".to_string(), v)),
            ((self.m_rotation_randomness).inspect_value(path, offset, limit))
                .map(|v| ("m_RotationRandomness".to_string(), v)),
            ((self.m_direction_curve_x).inspect_value(path, offset, limit))
                .map(|v| ("m_DirectionCurveX".to_string(), v)),
            ((self.m_direction_curve_y).inspect_value(path, offset, limit))
                .map(|v| ("m_DirectionCurveY".to_string(), v)),
            ((self.m_direction_curve_z).inspect_value(path, offset, limit))
                .map(|v| ("m_DirectionCurveZ".to_string(), v)),
            ((self.m_gravity_curve).inspect_value(path, offset, limit))
                .map(|v| ("m_GravityCurve".to_string(), v)),
            ((self.m_rotation_speed_curve).inspect_value(path, offset, limit))
                .map(|v| ("m_RotationSpeedCurve".to_string(), v)),
            ((self.m_rotation_attraction_curve).inspect_value(path, offset, limit))
                .map(|v| ("m_RotationAttractionCurve".to_string(), v)),
            ((self.m_drag_curve).inspect_value(path, offset, limit))
                .map(|v| ("m_DragCurve".to_string(), v)),
            ((self.m_vector_field).inspect_value(path, offset, limit))
                .map(|v| ("m_VectorField".to_string(), v)),
            ((self.m_vector_field_speed_curve).inspect_value(path, offset, limit))
                .map(|v| ("m_VectorFieldSpeedCurve".to_string(), v)),
            ((self.m_vector_field_attraction_curve).inspect_value(path, offset, limit))
                .map(|v| ("m_VectorFieldAttractionCurve".to_string(), v)),
            ((self.m_multiply_drag_by_particle_size).inspect_value(path, offset, limit))
                .map(|v| ("m_MultiplyDragByParticleSize".to_string(), v)),
            ((self.m_multiply_drag_by_particle_velocity).inspect_value(path, offset, limit))
                .map(|v| ("m_MultiplyDragByParticleVelocity".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
