//! 自动生成。请勿手动修改。

use crate::subclasses::tier_graphics_settings_editor::TierGraphicsSettingsEditor;

#[derive(Debug, Clone)]
pub struct TierSettings {
    pub m_build_target: i32,
    pub m_tier: i32,
    pub m_settings: TierGraphicsSettingsEditor,
    pub m_automatic: bool,
}

impl crate::Parse for TierSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = r.read_i32()?;
        let m_tier = r.read_i32()?;
        let m_settings = TierGraphicsSettingsEditor::parse(r, ver)?;
        r.align(4)?;
        let m_automatic = r.read_bool()?;
        r.align(4)?;

        Ok(TierSettings {
            m_build_target,
            m_tier,
            m_settings,
            m_automatic,
        })
    }
}
