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
