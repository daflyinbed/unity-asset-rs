//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::cloth_attachment::ClothAttachment;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct InteractiveCloth {
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
    pub m_mesh: PPtr,
    pub m_friction: f32,
    pub m_density: f32,
    pub m_pressure: f32,
    pub m_collision_response: f32,
    pub m_attachment_tear_factor: f32,
    pub m_attachment_response: f32,
    pub m_tear_factor: f32,
    pub m_attached_colliders: Vec<ClothAttachment>,
}

impl crate::Parse for InteractiveCloth {
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
        let m_mesh = crate::PPtr::parse(rdr, ver)?;
        let m_friction = rdr.read_f32()?;
        let m_density = rdr.read_f32()?;
        let m_pressure = rdr.read_f32()?;
        let m_collision_response = rdr.read_f32()?;
        let m_attachment_tear_factor = rdr.read_f32()?;
        let m_attachment_response = rdr.read_f32()?;
        let m_tear_factor = rdr.read_f32()?;
        let m_attached_colliders = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ClothAttachment::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(InteractiveCloth {
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
            m_mesh,
            m_friction,
            m_density,
            m_pressure,
            m_collision_response,
            m_attachment_tear_factor,
            m_attachment_response,
            m_tear_factor,
            m_attached_colliders,
        })
    }
}

impl crate::ToValue for InteractiveCloth {
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
            Some(("m_Mesh".to_string(), self.m_mesh.to_value())),
            Some(("m_Friction".to_string(), self.m_friction.to_value())),
            Some(("m_Density".to_string(), self.m_density.to_value())),
            Some(("m_Pressure".to_string(), self.m_pressure.to_value())),
            Some((
                "m_CollisionResponse".to_string(),
                self.m_collision_response.to_value(),
            )),
            Some((
                "m_AttachmentTearFactor".to_string(),
                self.m_attachment_tear_factor.to_value(),
            )),
            Some((
                "m_AttachmentResponse".to_string(),
                self.m_attachment_response.to_value(),
            )),
            Some(("m_TearFactor".to_string(), self.m_tear_factor.to_value())),
            Some((
                "m_AttachedColliders".to_string(),
                self.m_attached_colliders.to_value(),
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
                "m_Mesh" => (self.m_mesh).inspect_value(path, offset, limit),
                "m_Friction" => (self.m_friction).inspect_value(path, offset, limit),
                "m_Density" => (self.m_density).inspect_value(path, offset, limit),
                "m_Pressure" => (self.m_pressure).inspect_value(path, offset, limit),
                "m_CollisionResponse" => {
                    (self.m_collision_response).inspect_value(path, offset, limit)
                }
                "m_AttachmentTearFactor" => {
                    (self.m_attachment_tear_factor).inspect_value(path, offset, limit)
                }
                "m_AttachmentResponse" => {
                    (self.m_attachment_response).inspect_value(path, offset, limit)
                }
                "m_TearFactor" => (self.m_tear_factor).inspect_value(path, offset, limit),
                "m_AttachedColliders" => {
                    (self.m_attached_colliders).inspect_value(path, offset, limit)
                }
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
            ((self.m_mesh).inspect_value(path, offset, limit)).map(|v| ("m_Mesh".to_string(), v)),
            ((self.m_friction).inspect_value(path, offset, limit))
                .map(|v| ("m_Friction".to_string(), v)),
            ((self.m_density).inspect_value(path, offset, limit))
                .map(|v| ("m_Density".to_string(), v)),
            ((self.m_pressure).inspect_value(path, offset, limit))
                .map(|v| ("m_Pressure".to_string(), v)),
            ((self.m_collision_response).inspect_value(path, offset, limit))
                .map(|v| ("m_CollisionResponse".to_string(), v)),
            ((self.m_attachment_tear_factor).inspect_value(path, offset, limit))
                .map(|v| ("m_AttachmentTearFactor".to_string(), v)),
            ((self.m_attachment_response).inspect_value(path, offset, limit))
                .map(|v| ("m_AttachmentResponse".to_string(), v)),
            ((self.m_tear_factor).inspect_value(path, offset, limit))
                .map(|v| ("m_TearFactor".to_string(), v)),
            ((self.m_attached_colliders).inspect_value(path, offset, limit))
                .map(|v| ("m_AttachedColliders".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
