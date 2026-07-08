//! 自动生成。请勿手动修改。


/// 版本枚举：TextureImportInstructionsAndroidEtc2FallbackDownscaleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImportInstructionsAndroidEtc2FallbackDownscaleVariant {
    Default(bool),
    V2020_1(bool),
}

impl crate::Parse for TextureImportInstructionsAndroidEtc2FallbackDownscaleVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct TextureImportInstructions {
    pub compressed_format: i32,
    pub uncompressed_format: i32,
    pub recommended_format: Option<i32>,
    pub desired_format: Option<i32>,
    pub usage_mode: i32,
    pub color_space: i32,
    pub width: i32,
    pub android_etc2_fallback_format: Option<i32>,
    pub height: i32,
    pub compression_quality: i32,
    pub depth: Option<i32>,
    pub cube_intermediate_size: Option<i32>,
    pub android_etc2_fallback_downscale: Option<TextureImportInstructionsAndroidEtc2FallbackDownscaleVariant>,
    pub cube_mode: Option<i32>,
    pub vt_only: Option<bool>,
    pub cube_layout: Option<i32>,
}

impl crate::Parse for TextureImportInstructions {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut compressed_format = None;
        let mut uncompressed_format = None;
        let mut recommended_format = None;
        let mut desired_format = None;
        let mut usage_mode = None;
        let mut color_space = None;
        let mut width = None;
        let mut android_etc2_fallback_format = None;
        let mut height = None;
        let mut compression_quality = None;
        let mut depth = None;
        let mut cube_intermediate_size = None;
        let mut android_etc2_fallback_downscale = None;
        let mut cube_mode = None;
        let mut vt_only = None;
        let mut cube_layout = None;
            compressed_format = Some(rdr.read_i32()?);
            uncompressed_format = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            desired_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            recommended_format = Some(rdr.read_i32()?);
            desired_format = Some(rdr.read_i32()?);
        } else {
            recommended_format = Some(rdr.read_i32()?);
        }
            usage_mode = Some(rdr.read_i32()?);
            color_space = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            android_etc2_fallback_format = Some(rdr.read_i32()?);
        } else {
        }
            width = Some(rdr.read_i32()?);
            height = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            depth = Some(rdr.read_i32()?);
            cube_intermediate_size = Some(rdr.read_i32()?);
            cube_mode = Some(rdr.read_i32()?);
            cube_layout = Some(rdr.read_i32()?);
        } else {
        }
            compression_quality = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            vt_only = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            android_etc2_fallback_downscale = Some(TextureImportInstructionsAndroidEtc2FallbackDownscaleVariant::parse(rdr, ver)?);
            vt_only = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            android_etc2_fallback_downscale = Some(TextureImportInstructionsAndroidEtc2FallbackDownscaleVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(TextureImportInstructions {
            compressed_format: compressed_format.unwrap(),
            uncompressed_format: uncompressed_format.unwrap(),
            recommended_format,
            desired_format,
            usage_mode: usage_mode.unwrap(),
            color_space: color_space.unwrap(),
            width: width.unwrap(),
            android_etc2_fallback_format,
            height: height.unwrap(),
            compression_quality: compression_quality.unwrap(),
            depth,
            cube_intermediate_size,
            android_etc2_fallback_downscale,
            cube_mode,
            vt_only,
            cube_layout,
        })
    }
}
