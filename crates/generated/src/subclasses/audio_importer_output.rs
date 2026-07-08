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
