//! 自动生成。请勿手动修改。

use crate::subclasses::sample_settings::SampleSettings;
use crate::subclasses::streamed_resource::StreamedResource;

#[derive(Debug, Clone)]
pub struct AudioImporterOutput {
    pub output_settings: SampleSettings,
    pub output_container_format: i32,
    pub editor_output_settings: SampleSettings,
    pub editor_output_container_format: i32,
    pub player_resource: Option<StreamedResource>,
}

impl crate::Parse for AudioImporterOutput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut output_settings = None;
        let mut output_container_format = None;
        let mut editor_output_settings = None;
        let mut editor_output_container_format = None;
        let mut player_resource = None;
        output_settings = Some(SampleSettings::parse(rdr, ver)?);
        output_container_format = Some(rdr.read_i32()?);
        editor_output_settings = Some(SampleSettings::parse(rdr, ver)?);
        editor_output_container_format = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
        } else {
            player_resource = Some(StreamedResource::parse(rdr, ver)?);
        }

        Ok(AudioImporterOutput {
            output_settings: output_settings.unwrap(),
            output_container_format: output_container_format.unwrap(),
            editor_output_settings: editor_output_settings.unwrap(),
            editor_output_container_format: editor_output_container_format.unwrap(),
            player_resource,
        })
    }
}

impl crate::ToValue for AudioImporterOutput {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "outputSettings".to_string(),
                self.output_settings.to_value(),
            )),
            Some((
                "outputContainerFormat".to_string(),
                self.output_container_format.to_value(),
            )),
            Some((
                "editorOutputSettings".to_string(),
                self.editor_output_settings.to_value(),
            )),
            Some((
                "editorOutputContainerFormat".to_string(),
                self.editor_output_container_format.to_value(),
            )),
            crate::opt_field("playerResource", &self.player_resource),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "outputSettings" => (self.output_settings).inspect_value(path, offset, limit),
                "outputContainerFormat" => {
                    (self.output_container_format).inspect_value(path, offset, limit)
                }
                "editorOutputSettings" => {
                    (self.editor_output_settings).inspect_value(path, offset, limit)
                }
                "editorOutputContainerFormat" => {
                    (self.editor_output_container_format).inspect_value(path, offset, limit)
                }
                "playerResource" => (self.player_resource).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.output_settings).inspect_value(path, offset, limit))
                .map(|v| ("outputSettings".to_string(), v)),
            ((self.output_container_format).inspect_value(path, offset, limit))
                .map(|v| ("outputContainerFormat".to_string(), v)),
            ((self.editor_output_settings).inspect_value(path, offset, limit))
                .map(|v| ("editorOutputSettings".to_string(), v)),
            ((self.editor_output_container_format).inspect_value(path, offset, limit))
                .map(|v| ("editorOutputContainerFormat".to_string(), v)),
            ((self.player_resource).inspect_value(path, offset, limit))
                .map(|v| ("playerResource".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
