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
        let use_sustained_performance_mode = GoogleUseSustainedPerformanceModeVariant::parse(rdr, ver)?;
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
