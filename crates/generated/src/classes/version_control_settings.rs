//! 自动生成。请勿手动修改。

use crate::subclasses::collab_editor_settings::CollabEditorSettings;

#[derive(Debug, Clone)]
pub struct VersionControlSettings {
    pub m_mode: String,
    pub m_collab_editor_settings: Option<CollabEditorSettings>,
    pub m_track_packages_outside_project: Option<bool>,
}

impl crate::Parse for VersionControlSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_mode = None;
        let mut m_collab_editor_settings = None;
        let mut m_track_packages_outside_project = None;
        m_mode = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            m_track_packages_outside_project = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_collab_editor_settings = Some(CollabEditorSettings::parse(rdr, ver)?);
            m_track_packages_outside_project = Some(rdr.read_bool()?);
        } else {
            m_collab_editor_settings = Some(CollabEditorSettings::parse(rdr, ver)?);
        }

        Ok(VersionControlSettings {
            m_mode: m_mode.unwrap(),
            m_collab_editor_settings,
            m_track_packages_outside_project,
        })
    }
}
