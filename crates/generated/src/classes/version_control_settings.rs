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
        let m_mode = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_collab_editor_settings = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = CollabEditorSettings::parse(rdr, ver)?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_track_packages_outside_project = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };

        Ok(VersionControlSettings {
            m_mode,
            m_collab_editor_settings,
            m_track_packages_outside_project,
        })
    }
}
