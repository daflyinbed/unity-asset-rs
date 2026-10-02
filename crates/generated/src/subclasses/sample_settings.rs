//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SampleSettings {
    pub load_type: i32,
    pub sample_rate_setting: i32,
    pub sample_rate_override: u32,
    pub compression_format: i32,
    pub quality: f32,
    pub conversion_mode: i32,
    pub preload_audio_data: Option<bool>,
}

impl crate::Parse for SampleSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let load_type = rdr.read_i32()?;
        let sample_rate_setting = rdr.read_i32()?;
        let sample_rate_override = rdr.read_u32()?;
        let compression_format = rdr.read_i32()?;
        let quality = rdr.read_f32()?;
        let conversion_mode = rdr.read_i32()?;
        let preload_audio_data = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SampleSettings {
            load_type,
            sample_rate_setting,
            sample_rate_override,
            compression_format,
            quality,
            conversion_mode,
            preload_audio_data,
        })
    }
}

impl crate::ToValue for SampleSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("loadType".to_string(), self.load_type.to_value())),
            Some((
                "sampleRateSetting".to_string(),
                self.sample_rate_setting.to_value(),
            )),
            Some((
                "sampleRateOverride".to_string(),
                self.sample_rate_override.to_value(),
            )),
            Some((
                "compressionFormat".to_string(),
                self.compression_format.to_value(),
            )),
            Some(("quality".to_string(), self.quality.to_value())),
            Some((
                "conversionMode".to_string(),
                self.conversion_mode.to_value(),
            )),
            crate::opt_field("preloadAudioData", &self.preload_audio_data),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "loadType" => (self.load_type).inspect_value(path, offset, limit),
                "sampleRateSetting" => {
                    (self.sample_rate_setting).inspect_value(path, offset, limit)
                }
                "sampleRateOverride" => {
                    (self.sample_rate_override).inspect_value(path, offset, limit)
                }
                "compressionFormat" => (self.compression_format).inspect_value(path, offset, limit),
                "quality" => (self.quality).inspect_value(path, offset, limit),
                "conversionMode" => (self.conversion_mode).inspect_value(path, offset, limit),
                "preloadAudioData" => (self.preload_audio_data).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.load_type).inspect_value(path, offset, limit))
                .map(|v| ("loadType".to_string(), v)),
            ((self.sample_rate_setting).inspect_value(path, offset, limit))
                .map(|v| ("sampleRateSetting".to_string(), v)),
            ((self.sample_rate_override).inspect_value(path, offset, limit))
                .map(|v| ("sampleRateOverride".to_string(), v)),
            ((self.compression_format).inspect_value(path, offset, limit))
                .map(|v| ("compressionFormat".to_string(), v)),
            ((self.quality).inspect_value(path, offset, limit)).map(|v| ("quality".to_string(), v)),
            ((self.conversion_mode).inspect_value(path, offset, limit))
                .map(|v| ("conversionMode".to_string(), v)),
            ((self.preload_audio_data).inspect_value(path, offset, limit))
                .map(|v| ("preloadAudioData".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
