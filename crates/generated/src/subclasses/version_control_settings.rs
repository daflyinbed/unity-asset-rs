//! 自动生成。请勿手动修改。

use crate::subclasses::collab_editor_settings::CollabEditorSettings;

#[derive(Debug, Clone)]
pub struct VersionControlSettings {
    pub m_object_hide_flags: u32,
    pub m_mode: String,
    pub m_collab_editor_settings: CollabEditorSettings,
}

impl crate::Parse for VersionControlSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_mode = r.read_aligned_string()?;
        r.align(4)?;
        let m_collab_editor_settings = CollabEditorSettings::parse(r, ver)?;
        r.align(4)?;

        Ok(VersionControlSettings {
            m_object_hide_flags,
            m_mode,
            m_collab_editor_settings,
        })
    }
}
