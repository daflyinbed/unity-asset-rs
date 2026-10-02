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

impl crate::ToValue for VersionControlSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Mode".to_string(), self.m_mode.to_value())),
            crate::opt_field("m_CollabEditorSettings", &self.m_collab_editor_settings),
            crate::opt_field(
                "m_TrackPackagesOutsideProject",
                &self.m_track_packages_outside_project,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Mode" => (self.m_mode).inspect_value(path, offset, limit),
                "m_CollabEditorSettings" => {
                    (self.m_collab_editor_settings).inspect_value(path, offset, limit)
                }
                "m_TrackPackagesOutsideProject" => {
                    (self.m_track_packages_outside_project).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_mode).inspect_value(path, offset, limit)).map(|v| ("m_Mode".to_string(), v)),
            ((self.m_collab_editor_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_CollabEditorSettings".to_string(), v)),
            ((self.m_track_packages_outside_project).inspect_value(path, offset, limit))
                .map(|v| ("m_TrackPackagesOutsideProject".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
