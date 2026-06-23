//! 自动生成。请勿手动修改。

use crate::subclasses::collab_editor_settings::CollabEditorSettings;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct VersionControlSettings {
    pub parent: Object,
    pub m_mode: String,
    pub m_collab_editor_settings: Option<CollabEditorSettings>,
    pub m_track_packages_outside_project: Option<bool>,
}

impl crate::Parse for VersionControlSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_mode = r.read_aligned_string()?;
        r.align(4)?;
        let m_collab_editor_settings = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = CollabEditorSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_track_packages_outside_project = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(VersionControlSettings {
            parent,
            m_mode,
            m_collab_editor_settings,
            m_track_packages_outside_project,
        })
    }
}
