//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct MeshParticleEmitter {
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_emit: bool,
    pub min_size: f32,
    pub max_size: f32,
    pub min_energy: f32,
    pub max_energy: f32,
    pub min_emission: f32,
    pub max_emission: f32,
    pub world_velocity: Vector3f,
    pub local_velocity: Vector3f,
    pub rnd_velocity: Vector3f,
    pub emitter_velocity_scale: f32,
    pub tangent_velocity: Vector3f,
    pub angular_velocity: f32,
    pub rnd_angular_velocity: f32,
    pub rnd_rotation: bool,
    pub simulate_in_worldspace: bool,
    pub m_one_shot: bool,
    pub m_interpolate_triangles: bool,
    pub m_systematic: bool,
    pub m_min_normal_velocity: f32,
    pub m_max_normal_velocity: f32,
    pub m_mesh: PPtr,
}

impl crate::Parse for MeshParticleEmitter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_bool()?;
        let m_emit = rdr.read_bool()?;
        rdr.align(4)?;
        let min_size = rdr.read_f32()?;
        let max_size = rdr.read_f32()?;
        let min_energy = rdr.read_f32()?;
        let max_energy = rdr.read_f32()?;
        let min_emission = rdr.read_f32()?;
        let max_emission = rdr.read_f32()?;
        let world_velocity = Vector3f::parse(rdr, ver)?;
        let local_velocity = Vector3f::parse(rdr, ver)?;
        let rnd_velocity = Vector3f::parse(rdr, ver)?;
        let emitter_velocity_scale = rdr.read_f32()?;
        let tangent_velocity = Vector3f::parse(rdr, ver)?;
        let angular_velocity = rdr.read_f32()?;
        let rnd_angular_velocity = rdr.read_f32()?;
        let rnd_rotation = rdr.read_bool()?;
        let simulate_in_worldspace = rdr.read_bool()?;
        let m_one_shot = rdr.read_bool()?;
        let m_interpolate_triangles = rdr.read_bool()?;
        let m_systematic = rdr.read_bool()?;
        rdr.align(4)?;
        let m_min_normal_velocity = rdr.read_f32()?;
        let m_max_normal_velocity = rdr.read_f32()?;
        let m_mesh = crate::PPtr::parse(rdr, ver)?;

        Ok(MeshParticleEmitter {
            m_game_object,
            m_enabled,
            m_emit,
            min_size,
            max_size,
            min_energy,
            max_energy,
            min_emission,
            max_emission,
            world_velocity,
            local_velocity,
            rnd_velocity,
            emitter_velocity_scale,
            tangent_velocity,
            angular_velocity,
            rnd_angular_velocity,
            rnd_rotation,
            simulate_in_worldspace,
            m_one_shot,
            m_interpolate_triangles,
            m_systematic,
            m_min_normal_velocity,
            m_max_normal_velocity,
            m_mesh,
        })
    }
}

impl crate::ToValue for MeshParticleEmitter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Emit".to_string(), self.m_emit.to_value())),
            Some(("minSize".to_string(), self.min_size.to_value())),
            Some(("maxSize".to_string(), self.max_size.to_value())),
            Some(("minEnergy".to_string(), self.min_energy.to_value())),
            Some(("maxEnergy".to_string(), self.max_energy.to_value())),
            Some(("minEmission".to_string(), self.min_emission.to_value())),
            Some(("maxEmission".to_string(), self.max_emission.to_value())),
            Some(("worldVelocity".to_string(), self.world_velocity.to_value())),
            Some(("localVelocity".to_string(), self.local_velocity.to_value())),
            Some(("rndVelocity".to_string(), self.rnd_velocity.to_value())),
            Some((
                "emitterVelocityScale".to_string(),
                self.emitter_velocity_scale.to_value(),
            )),
            Some((
                "tangentVelocity".to_string(),
                self.tangent_velocity.to_value(),
            )),
            Some((
                "angularVelocity".to_string(),
                self.angular_velocity.to_value(),
            )),
            Some((
                "rndAngularVelocity".to_string(),
                self.rnd_angular_velocity.to_value(),
            )),
            Some(("rndRotation".to_string(), self.rnd_rotation.to_value())),
            Some((
                "Simulate in Worldspace?".to_string(),
                self.simulate_in_worldspace.to_value(),
            )),
            Some(("m_OneShot".to_string(), self.m_one_shot.to_value())),
            Some((
                "m_InterpolateTriangles".to_string(),
                self.m_interpolate_triangles.to_value(),
            )),
            Some(("m_Systematic".to_string(), self.m_systematic.to_value())),
            Some((
                "m_MinNormalVelocity".to_string(),
                self.m_min_normal_velocity.to_value(),
            )),
            Some((
                "m_MaxNormalVelocity".to_string(),
                self.m_max_normal_velocity.to_value(),
            )),
            Some(("m_Mesh".to_string(), self.m_mesh.to_value())),
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
                "m_Emit" => (self.m_emit).inspect_value(path, offset, limit),
                "minSize" => (self.min_size).inspect_value(path, offset, limit),
                "maxSize" => (self.max_size).inspect_value(path, offset, limit),
                "minEnergy" => (self.min_energy).inspect_value(path, offset, limit),
                "maxEnergy" => (self.max_energy).inspect_value(path, offset, limit),
                "minEmission" => (self.min_emission).inspect_value(path, offset, limit),
                "maxEmission" => (self.max_emission).inspect_value(path, offset, limit),
                "worldVelocity" => (self.world_velocity).inspect_value(path, offset, limit),
                "localVelocity" => (self.local_velocity).inspect_value(path, offset, limit),
                "rndVelocity" => (self.rnd_velocity).inspect_value(path, offset, limit),
                "emitterVelocityScale" => {
                    (self.emitter_velocity_scale).inspect_value(path, offset, limit)
                }
                "tangentVelocity" => (self.tangent_velocity).inspect_value(path, offset, limit),
                "angularVelocity" => (self.angular_velocity).inspect_value(path, offset, limit),
                "rndAngularVelocity" => {
                    (self.rnd_angular_velocity).inspect_value(path, offset, limit)
                }
                "rndRotation" => (self.rnd_rotation).inspect_value(path, offset, limit),
                "Simulate in Worldspace?" => {
                    (self.simulate_in_worldspace).inspect_value(path, offset, limit)
                }
                "m_OneShot" => (self.m_one_shot).inspect_value(path, offset, limit),
                "m_InterpolateTriangles" => {
                    (self.m_interpolate_triangles).inspect_value(path, offset, limit)
                }
                "m_Systematic" => (self.m_systematic).inspect_value(path, offset, limit),
                "m_MinNormalVelocity" => {
                    (self.m_min_normal_velocity).inspect_value(path, offset, limit)
                }
                "m_MaxNormalVelocity" => {
                    (self.m_max_normal_velocity).inspect_value(path, offset, limit)
                }
                "m_Mesh" => (self.m_mesh).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_emit).inspect_value(path, offset, limit)).map(|v| ("m_Emit".to_string(), v)),
            ((self.min_size).inspect_value(path, offset, limit))
                .map(|v| ("minSize".to_string(), v)),
            ((self.max_size).inspect_value(path, offset, limit))
                .map(|v| ("maxSize".to_string(), v)),
            ((self.min_energy).inspect_value(path, offset, limit))
                .map(|v| ("minEnergy".to_string(), v)),
            ((self.max_energy).inspect_value(path, offset, limit))
                .map(|v| ("maxEnergy".to_string(), v)),
            ((self.min_emission).inspect_value(path, offset, limit))
                .map(|v| ("minEmission".to_string(), v)),
            ((self.max_emission).inspect_value(path, offset, limit))
                .map(|v| ("maxEmission".to_string(), v)),
            ((self.world_velocity).inspect_value(path, offset, limit))
                .map(|v| ("worldVelocity".to_string(), v)),
            ((self.local_velocity).inspect_value(path, offset, limit))
                .map(|v| ("localVelocity".to_string(), v)),
            ((self.rnd_velocity).inspect_value(path, offset, limit))
                .map(|v| ("rndVelocity".to_string(), v)),
            ((self.emitter_velocity_scale).inspect_value(path, offset, limit))
                .map(|v| ("emitterVelocityScale".to_string(), v)),
            ((self.tangent_velocity).inspect_value(path, offset, limit))
                .map(|v| ("tangentVelocity".to_string(), v)),
            ((self.angular_velocity).inspect_value(path, offset, limit))
                .map(|v| ("angularVelocity".to_string(), v)),
            ((self.rnd_angular_velocity).inspect_value(path, offset, limit))
                .map(|v| ("rndAngularVelocity".to_string(), v)),
            ((self.rnd_rotation).inspect_value(path, offset, limit))
                .map(|v| ("rndRotation".to_string(), v)),
            ((self.simulate_in_worldspace).inspect_value(path, offset, limit))
                .map(|v| ("Simulate in Worldspace?".to_string(), v)),
            ((self.m_one_shot).inspect_value(path, offset, limit))
                .map(|v| ("m_OneShot".to_string(), v)),
            ((self.m_interpolate_triangles).inspect_value(path, offset, limit))
                .map(|v| ("m_InterpolateTriangles".to_string(), v)),
            ((self.m_systematic).inspect_value(path, offset, limit))
                .map(|v| ("m_Systematic".to_string(), v)),
            ((self.m_min_normal_velocity).inspect_value(path, offset, limit))
                .map(|v| ("m_MinNormalVelocity".to_string(), v)),
            ((self.m_max_normal_velocity).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxNormalVelocity".to_string(), v)),
            ((self.m_mesh).inspect_value(path, offset, limit)).map(|v| ("m_Mesh".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
