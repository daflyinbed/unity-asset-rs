//! 自动生成。请勿手动修改。

use crate::subclasses::tier_graphics_settings_editor::TierGraphicsSettingsEditor;

#[derive(Debug, Clone)]
pub struct TierSettings {
    pub m_build_target: String,
    pub m_settings: TierGraphicsSettingsEditor,
    pub m_automatic: bool,
}

impl crate::Parse for TierSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_settings = TierGraphicsSettingsEditor::parse(rdr, ver)?;
        let m_automatic = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(TierSettings {
            m_build_target,
            m_settings,
            m_automatic,
        })
    }
}
