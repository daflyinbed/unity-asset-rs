//! 自动生成。请勿手动修改。

/// 版本枚举：GoogleUseSustainedPerformanceModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GoogleUseSustainedPerformanceModeVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for GoogleUseSustainedPerformanceModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for GoogleUseSustainedPerformanceModeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Google {
    pub depth_format: i32,
    pub use_sustained_performance_mode: GoogleUseSustainedPerformanceModeVariant,
    pub enable_video_layer: Option<bool>,
    pub use_protected_video_memory: Option<bool>,
    pub minimum_supported_head_tracking: Option<i32>,
    pub maximum_supported_head_tracking: Option<i32>,
}

impl crate::Parse for Google {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let depth_format = rdr.read_i32()?;
        let use_sustained_performance_mode =
            GoogleUseSustainedPerformanceModeVariant::parse(rdr, ver)?;
        let enable_video_layer = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };
        let use_protected_video_memory = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let minimum_supported_head_tracking = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let maximum_supported_head_tracking = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(Google {
            depth_format,
            use_sustained_performance_mode,
            enable_video_layer,
            use_protected_video_memory,
            minimum_supported_head_tracking,
            maximum_supported_head_tracking,
        })
    }
}

impl crate::ToValue for Google {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("depthFormat".to_string(), self.depth_format.to_value())),
            Some((
                "useSustainedPerformanceMode".to_string(),
                self.use_sustained_performance_mode.to_value(),
            )),
            crate::opt_field("enableVideoLayer", &self.enable_video_layer),
            crate::opt_field("useProtectedVideoMemory", &self.use_protected_video_memory),
            crate::opt_field(
                "minimumSupportedHeadTracking",
                &self.minimum_supported_head_tracking,
            ),
            crate::opt_field(
                "maximumSupportedHeadTracking",
                &self.maximum_supported_head_tracking,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "depthFormat" => (self.depth_format).inspect_value(path, offset, limit),
                "useSustainedPerformanceMode" => {
                    (self.use_sustained_performance_mode).inspect_value(path, offset, limit)
                }
                "enableVideoLayer" => (self.enable_video_layer).inspect_value(path, offset, limit),
                "useProtectedVideoMemory" => {
                    (self.use_protected_video_memory).inspect_value(path, offset, limit)
                }
                "minimumSupportedHeadTracking" => {
                    (self.minimum_supported_head_tracking).inspect_value(path, offset, limit)
                }
                "maximumSupportedHeadTracking" => {
                    (self.maximum_supported_head_tracking).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.depth_format).inspect_value(path, offset, limit))
                .map(|v| ("depthFormat".to_string(), v)),
            ((self.use_sustained_performance_mode).inspect_value(path, offset, limit))
                .map(|v| ("useSustainedPerformanceMode".to_string(), v)),
            ((self.enable_video_layer).inspect_value(path, offset, limit))
                .map(|v| ("enableVideoLayer".to_string(), v)),
            ((self.use_protected_video_memory).inspect_value(path, offset, limit))
                .map(|v| ("useProtectedVideoMemory".to_string(), v)),
            ((self.minimum_supported_head_tracking).inspect_value(path, offset, limit))
                .map(|v| ("minimumSupportedHeadTracking".to_string(), v)),
            ((self.maximum_supported_head_tracking).inspect_value(path, offset, limit))
                .map(|v| ("maximumSupportedHeadTracking".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
