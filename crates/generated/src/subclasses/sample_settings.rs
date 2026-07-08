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
