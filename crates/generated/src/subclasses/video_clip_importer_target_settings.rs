//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VideoClipImporterTargetSettings {
    pub enable_transcoding: bool,
    pub codec: i32,
    pub resize_format: i32,
    pub aspect_ratio: i32,
    pub custom_width: i32,
    pub custom_height: i32,
    pub bitrate_mode: i32,
    pub spatial_quality: i32,
}

impl crate::Parse for VideoClipImporterTargetSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enable_transcoding = r.read_bool()?;
        r.align(4)?;
        let codec = r.read_i32()?;
        let resize_format = r.read_i32()?;
        let aspect_ratio = r.read_i32()?;
        r.align(4)?;
        let custom_width = r.read_i32()?;
        let custom_height = r.read_i32()?;
        let bitrate_mode = r.read_i32()?;
        let spatial_quality = r.read_i32()?;

        Ok(VideoClipImporterTargetSettings {
            enable_transcoding,
            codec,
            resize_format,
            aspect_ratio,
            custom_width,
            custom_height,
            bitrate_mode,
            spatial_quality,
        })
    }
}
