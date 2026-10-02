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

impl crate::ToValue for TextureImportInstructionsAndroidEtc2FallbackDownscaleVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
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
    pub android_etc2_fallback_downscale:
        Option<TextureImportInstructionsAndroidEtc2FallbackDownscaleVariant>,
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
            android_etc2_fallback_downscale = Some(
                TextureImportInstructionsAndroidEtc2FallbackDownscaleVariant::parse(rdr, ver)?,
            );
            vt_only = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            android_etc2_fallback_downscale = Some(
                TextureImportInstructionsAndroidEtc2FallbackDownscaleVariant::parse(rdr, ver)?,
            );
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

impl crate::ToValue for TextureImportInstructions {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "compressedFormat".to_string(),
                self.compressed_format.to_value(),
            )),
            Some((
                "uncompressedFormat".to_string(),
                self.uncompressed_format.to_value(),
            )),
            crate::opt_field("recommendedFormat", &self.recommended_format),
            crate::opt_field("desiredFormat", &self.desired_format),
            Some(("usageMode".to_string(), self.usage_mode.to_value())),
            Some(("colorSpace".to_string(), self.color_space.to_value())),
            Some(("width".to_string(), self.width.to_value())),
            crate::opt_field(
                "androidETC2FallbackFormat",
                &self.android_etc2_fallback_format,
            ),
            Some(("height".to_string(), self.height.to_value())),
            Some((
                "compressionQuality".to_string(),
                self.compression_quality.to_value(),
            )),
            crate::opt_field("depth", &self.depth),
            crate::opt_field("cubeIntermediateSize", &self.cube_intermediate_size),
            crate::opt_field(
                "androidETC2FallbackDownscale",
                &self.android_etc2_fallback_downscale,
            ),
            crate::opt_field("cubeMode", &self.cube_mode),
            crate::opt_field("vtOnly", &self.vt_only),
            crate::opt_field("cubeLayout", &self.cube_layout),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "compressedFormat" => (self.compressed_format).inspect_value(path, offset, limit),
                "uncompressedFormat" => {
                    (self.uncompressed_format).inspect_value(path, offset, limit)
                }
                "recommendedFormat" => (self.recommended_format).inspect_value(path, offset, limit),
                "desiredFormat" => (self.desired_format).inspect_value(path, offset, limit),
                "usageMode" => (self.usage_mode).inspect_value(path, offset, limit),
                "colorSpace" => (self.color_space).inspect_value(path, offset, limit),
                "width" => (self.width).inspect_value(path, offset, limit),
                "androidETC2FallbackFormat" => {
                    (self.android_etc2_fallback_format).inspect_value(path, offset, limit)
                }
                "height" => (self.height).inspect_value(path, offset, limit),
                "compressionQuality" => {
                    (self.compression_quality).inspect_value(path, offset, limit)
                }
                "depth" => (self.depth).inspect_value(path, offset, limit),
                "cubeIntermediateSize" => {
                    (self.cube_intermediate_size).inspect_value(path, offset, limit)
                }
                "androidETC2FallbackDownscale" => {
                    (self.android_etc2_fallback_downscale).inspect_value(path, offset, limit)
                }
                "cubeMode" => (self.cube_mode).inspect_value(path, offset, limit),
                "vtOnly" => (self.vt_only).inspect_value(path, offset, limit),
                "cubeLayout" => (self.cube_layout).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.compressed_format).inspect_value(path, offset, limit))
                .map(|v| ("compressedFormat".to_string(), v)),
            ((self.uncompressed_format).inspect_value(path, offset, limit))
                .map(|v| ("uncompressedFormat".to_string(), v)),
            ((self.recommended_format).inspect_value(path, offset, limit))
                .map(|v| ("recommendedFormat".to_string(), v)),
            ((self.desired_format).inspect_value(path, offset, limit))
                .map(|v| ("desiredFormat".to_string(), v)),
            ((self.usage_mode).inspect_value(path, offset, limit))
                .map(|v| ("usageMode".to_string(), v)),
            ((self.color_space).inspect_value(path, offset, limit))
                .map(|v| ("colorSpace".to_string(), v)),
            ((self.width).inspect_value(path, offset, limit)).map(|v| ("width".to_string(), v)),
            ((self.android_etc2_fallback_format).inspect_value(path, offset, limit))
                .map(|v| ("androidETC2FallbackFormat".to_string(), v)),
            ((self.height).inspect_value(path, offset, limit)).map(|v| ("height".to_string(), v)),
            ((self.compression_quality).inspect_value(path, offset, limit))
                .map(|v| ("compressionQuality".to_string(), v)),
            ((self.depth).inspect_value(path, offset, limit)).map(|v| ("depth".to_string(), v)),
            ((self.cube_intermediate_size).inspect_value(path, offset, limit))
                .map(|v| ("cubeIntermediateSize".to_string(), v)),
            ((self.android_etc2_fallback_downscale).inspect_value(path, offset, limit))
                .map(|v| ("androidETC2FallbackDownscale".to_string(), v)),
            ((self.cube_mode).inspect_value(path, offset, limit))
                .map(|v| ("cubeMode".to_string(), v)),
            ((self.vt_only).inspect_value(path, offset, limit)).map(|v| ("vtOnly".to_string(), v)),
            ((self.cube_layout).inspect_value(path, offset, limit))
                .map(|v| ("cubeLayout".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
