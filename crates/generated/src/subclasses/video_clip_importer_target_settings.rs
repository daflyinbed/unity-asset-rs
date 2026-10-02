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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enable_transcoding = rdr.read_bool()?;
        rdr.align(4)?;
        let codec = rdr.read_i32()?;
        let resize_format = rdr.read_i32()?;
        let aspect_ratio = rdr.read_i32()?;
        rdr.align(4)?;
        let custom_width = rdr.read_i32()?;
        let custom_height = rdr.read_i32()?;
        let bitrate_mode = rdr.read_i32()?;
        let spatial_quality = rdr.read_i32()?;

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

impl crate::ToValue for VideoClipImporterTargetSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "enableTranscoding".to_string(),
                self.enable_transcoding.to_value(),
            )),
            Some(("codec".to_string(), self.codec.to_value())),
            Some(("resizeFormat".to_string(), self.resize_format.to_value())),
            Some(("aspectRatio".to_string(), self.aspect_ratio.to_value())),
            Some(("customWidth".to_string(), self.custom_width.to_value())),
            Some(("customHeight".to_string(), self.custom_height.to_value())),
            Some(("bitrateMode".to_string(), self.bitrate_mode.to_value())),
            Some((
                "spatialQuality".to_string(),
                self.spatial_quality.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enableTranscoding" => (self.enable_transcoding).inspect_value(path, offset, limit),
                "codec" => (self.codec).inspect_value(path, offset, limit),
                "resizeFormat" => (self.resize_format).inspect_value(path, offset, limit),
                "aspectRatio" => (self.aspect_ratio).inspect_value(path, offset, limit),
                "customWidth" => (self.custom_width).inspect_value(path, offset, limit),
                "customHeight" => (self.custom_height).inspect_value(path, offset, limit),
                "bitrateMode" => (self.bitrate_mode).inspect_value(path, offset, limit),
                "spatialQuality" => (self.spatial_quality).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enable_transcoding).inspect_value(path, offset, limit))
                .map(|v| ("enableTranscoding".to_string(), v)),
            ((self.codec).inspect_value(path, offset, limit)).map(|v| ("codec".to_string(), v)),
            ((self.resize_format).inspect_value(path, offset, limit))
                .map(|v| ("resizeFormat".to_string(), v)),
            ((self.aspect_ratio).inspect_value(path, offset, limit))
                .map(|v| ("aspectRatio".to_string(), v)),
            ((self.custom_width).inspect_value(path, offset, limit))
                .map(|v| ("customWidth".to_string(), v)),
            ((self.custom_height).inspect_value(path, offset, limit))
                .map(|v| ("customHeight".to_string(), v)),
            ((self.bitrate_mode).inspect_value(path, offset, limit))
                .map(|v| ("bitrateMode".to_string(), v)),
            ((self.spatial_quality).inspect_value(path, offset, limit))
                .map(|v| ("spatialQuality".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
