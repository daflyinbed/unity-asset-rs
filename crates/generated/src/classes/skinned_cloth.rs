//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::cloth_constrain_coefficients::ClothConstrainCoefficients;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SkinnedCloth {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_bending_stiffness: f32,
    pub m_stretching_stiffness: f32,
    pub m_damping: f32,
    pub m_thickness: f32,
    pub m_use_gravity: bool,
    pub m_self_collision: bool,
    pub m_external_acceleration: Vector3f,
    pub m_random_acceleration: Vector3f,
    pub m_world_velocity_scale: f32,
    pub m_world_acceleration_scale: f32,
    pub m_coefficients: Vec<ClothConstrainCoefficients>,
}

impl crate::Parse for SkinnedCloth {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_bending_stiffness = rdr.read_f32()?;
        let m_stretching_stiffness = rdr.read_f32()?;
        let m_damping = rdr.read_f32()?;
        let m_thickness = rdr.read_f32()?;
        let m_use_gravity = rdr.read_bool()?;
        let m_self_collision = rdr.read_bool()?;
        rdr.align(4)?;
        let m_external_acceleration = Vector3f::parse(rdr, ver)?;
        let m_random_acceleration = Vector3f::parse(rdr, ver)?;
        let m_world_velocity_scale = rdr.read_f32()?;
        let m_world_acceleration_scale = rdr.read_f32()?;
        let m_coefficients = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ClothConstrainCoefficients::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(SkinnedCloth {
            m_game_object,
            m_enabled,
            m_bending_stiffness,
            m_stretching_stiffness,
            m_damping,
            m_thickness,
            m_use_gravity,
            m_self_collision,
            m_external_acceleration,
            m_random_acceleration,
            m_world_velocity_scale,
            m_world_acceleration_scale,
            m_coefficients,
        })
    }
}

impl crate::ToValue for SkinnedCloth {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_BendingStiffness".to_string(),
                self.m_bending_stiffness.to_value(),
            )),
            Some((
                "m_StretchingStiffness".to_string(),
                self.m_stretching_stiffness.to_value(),
            )),
            Some(("m_Damping".to_string(), self.m_damping.to_value())),
            Some(("m_Thickness".to_string(), self.m_thickness.to_value())),
            Some(("m_UseGravity".to_string(), self.m_use_gravity.to_value())),
            Some((
                "m_SelfCollision".to_string(),
                self.m_self_collision.to_value(),
            )),
            Some((
                "m_ExternalAcceleration".to_string(),
                self.m_external_acceleration.to_value(),
            )),
            Some((
                "m_RandomAcceleration".to_string(),
                self.m_random_acceleration.to_value(),
            )),
            Some((
                "m_WorldVelocityScale".to_string(),
                self.m_world_velocity_scale.to_value(),
            )),
            Some((
                "m_WorldAccelerationScale".to_string(),
                self.m_world_acceleration_scale.to_value(),
            )),
            Some(("m_Coefficients".to_string(), self.m_coefficients.to_value())),
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
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_BendingStiffness" => {
                    (self.m_bending_stiffness).inspect_value(path, offset, limit)
                }
                "m_StretchingStiffness" => {
                    (self.m_stretching_stiffness).inspect_value(path, offset, limit)
                }
                "m_Damping" => (self.m_damping).inspect_value(path, offset, limit),
                "m_Thickness" => (self.m_thickness).inspect_value(path, offset, limit),
                "m_UseGravity" => (self.m_use_gravity).inspect_value(path, offset, limit),
                "m_SelfCollision" => (self.m_self_collision).inspect_value(path, offset, limit),
                "m_ExternalAcceleration" => {
                    (self.m_external_acceleration).inspect_value(path, offset, limit)
                }
                "m_RandomAcceleration" => {
                    (self.m_random_acceleration).inspect_value(path, offset, limit)
                }
                "m_WorldVelocityScale" => {
                    (self.m_world_velocity_scale).inspect_value(path, offset, limit)
                }
                "m_WorldAccelerationScale" => {
                    (self.m_world_acceleration_scale).inspect_value(path, offset, limit)
                }
                "m_Coefficients" => (self.m_coefficients).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_bending_stiffness).inspect_value(path, offset, limit))
                .map(|v| ("m_BendingStiffness".to_string(), v)),
            ((self.m_stretching_stiffness).inspect_value(path, offset, limit))
                .map(|v| ("m_StretchingStiffness".to_string(), v)),
            ((self.m_damping).inspect_value(path, offset, limit))
                .map(|v| ("m_Damping".to_string(), v)),
            ((self.m_thickness).inspect_value(path, offset, limit))
                .map(|v| ("m_Thickness".to_string(), v)),
            ((self.m_use_gravity).inspect_value(path, offset, limit))
                .map(|v| ("m_UseGravity".to_string(), v)),
            ((self.m_self_collision).inspect_value(path, offset, limit))
                .map(|v| ("m_SelfCollision".to_string(), v)),
            ((self.m_external_acceleration).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalAcceleration".to_string(), v)),
            ((self.m_random_acceleration).inspect_value(path, offset, limit))
                .map(|v| ("m_RandomAcceleration".to_string(), v)),
            ((self.m_world_velocity_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_WorldVelocityScale".to_string(), v)),
            ((self.m_world_acceleration_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_WorldAccelerationScale".to_string(), v)),
            ((self.m_coefficients).inspect_value(path, offset, limit))
                .map(|v| ("m_Coefficients".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
