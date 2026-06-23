//! 自动生成。请勿手动修改。

use crate::subclasses::platform_shader_settings::PlatformShaderSettings;

#[derive(Debug, Clone)]
pub struct BuildTargetShaderSettings {
    pub m_build_target: String,
    pub m_shader_settings: PlatformShaderSettings,
    pub m_automatic: bool,
}

impl crate::Parse for BuildTargetShaderSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = r.read_aligned_string()?;
        r.align(4)?;
        let m_shader_settings = PlatformShaderSettings::parse(r, ver)?;
        r.align(4)?;
        let m_automatic = r.read_bool()?;
        r.align(4)?;

        Ok(BuildTargetShaderSettings {
            m_build_target,
            m_shader_settings,
            m_automatic,
        })
    }
}
