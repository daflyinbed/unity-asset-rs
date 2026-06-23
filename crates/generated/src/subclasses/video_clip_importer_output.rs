//! 自动生成。请勿手动修改。

use crate::subclasses::streamed_resource::StreamedResource;
use crate::subclasses::video_clip_importer_target_settings::VideoClipImporterTargetSettings;

#[derive(Debug, Clone)]
pub struct VideoClipImporterOutput {
    pub format: i32,
    pub settings: VideoClipImporterTargetSettings,
    pub encoded_width: i32,
    pub encoded_height: i32,
    pub encoded_start_frame: i32,
    pub encoded_end_frame: i32,
    pub streamed_resource: StreamedResource,
}

impl crate::Parse for VideoClipImporterOutput {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let format = r.read_i32()?;
        let settings = VideoClipImporterTargetSettings::parse(r, ver)?;
        r.align(4)?;
        let encoded_width = r.read_i32()?;
        let encoded_height = r.read_i32()?;
        let encoded_start_frame = r.read_i32()?;
        let encoded_end_frame = r.read_i32()?;
        let streamed_resource = StreamedResource::parse(r, ver)?;
        r.align(4)?;

        Ok(VideoClipImporterOutput {
            format,
            settings,
            encoded_width,
            encoded_height,
            encoded_start_frame,
            encoded_end_frame,
            streamed_resource,
        })
    }
}
