//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SampleSettings {
    pub load_type: i32,
    pub sample_rate_setting: i32,
    pub sample_rate_override: u32,
    pub compression_format: i32,
    pub quality: f32,
    pub conversion_mode: i32,
}

impl crate::Parse for SampleSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let load_type = r.read_i32()?;
        let sample_rate_setting = r.read_i32()?;
        let sample_rate_override = r.read_u32()?;
        let compression_format = r.read_i32()?;
        let quality = r.read_f32()?;
        let conversion_mode = r.read_i32()?;

        Ok(SampleSettings {
            load_type,
            sample_rate_setting,
            sample_rate_override,
            compression_format,
            quality,
            conversion_mode,
        })
    }
}
