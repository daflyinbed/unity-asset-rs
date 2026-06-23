//! 自动生成。请勿手动修改。

use crate::subclasses::sample_settings::SampleSettings;
use crate::subclasses::streamed_resource::StreamedResource;

#[derive(Debug, Clone)]
pub struct AudioImporterOutput {
    pub output_settings: SampleSettings,
    pub output_container_format: i32,
    pub editor_output_settings: SampleSettings,
    pub editor_output_container_format: i32,
    pub player_resource: StreamedResource,
}

impl crate::Parse for AudioImporterOutput {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let output_settings = SampleSettings::parse(r, ver)?;
        let output_container_format = r.read_i32()?;
        let editor_output_settings = SampleSettings::parse(r, ver)?;
        let editor_output_container_format = r.read_i32()?;
        let player_resource = StreamedResource::parse(r, ver)?;
        r.align(4)?;

        Ok(AudioImporterOutput {
            output_settings,
            output_container_format,
            editor_output_settings,
            editor_output_container_format,
            player_resource,
        })
    }
}
