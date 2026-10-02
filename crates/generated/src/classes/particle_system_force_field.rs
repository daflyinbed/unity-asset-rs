//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::particle_system_force_field_parameters::ParticleSystemForceFieldParameters;

#[derive(Debug, Clone)]
pub struct ParticleSystemForceField {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_parameters: ParticleSystemForceFieldParameters,
}

impl crate::Parse for ParticleSystemForceField {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_parameters = ParticleSystemForceFieldParameters::parse(rdr, ver)?;

        Ok(ParticleSystemForceField {
            m_game_object,
            m_enabled,
            m_parameters,
        })
    }
}

impl crate::ToValue for ParticleSystemForceField {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Parameters".to_string(), self.m_parameters.to_value())),
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
                "m_Parameters" => (self.m_parameters).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_parameters).inspect_value(path, offset, limit))
                .map(|v| ("m_Parameters".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
