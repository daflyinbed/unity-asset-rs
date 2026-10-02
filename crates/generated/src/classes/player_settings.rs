//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::android_device_filter_data::AndroidDeviceFilterData;
use crate::subclasses::aspect_ratios::AspectRatios;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::splash_screen_logo::SplashScreenLogo;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vr_settings::VrSettings;

/// 版本枚举：PlayerSettingsAndroidProfilerVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsAndroidProfilerVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for PlayerSettingsAndroidProfilerVariant {
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

impl crate::ToValue for PlayerSettingsAndroidProfilerVariant {
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

/// 版本枚举：PlayerSettingsAndroidFilterTouchesWhenObscuredVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsAndroidFilterTouchesWhenObscuredVariant {
    Default(bool),
    V2018_1(bool),
}

impl crate::Parse for PlayerSettingsAndroidFilterTouchesWhenObscuredVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsAndroidFilterTouchesWhenObscuredVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsTargetResolutionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsTargetResolutionVariant {
    Default(i32),
    V3_5(i32),
}

impl crate::Parse for PlayerSettingsTargetResolutionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V3_5(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsTargetResolutionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V3_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V3_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsMShowUnitySplashScreenVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsMShowUnitySplashScreenVariant {
    Default(bool),
    V5_5(bool),
}

impl crate::Parse for PlayerSettingsMShowUnitySplashScreenVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsMShowUnitySplashScreenVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsMVirtualRealitySplashScreenVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsMVirtualRealitySplashScreenVariant {
    Default(PPtr),
    V2021_1(PPtr),
}

impl crate::Parse for PlayerSettingsMVirtualRealitySplashScreenVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::V2021_1(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsMVirtualRealitySplashScreenVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2021_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsMMtRenderingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsMMtRenderingVariant {
    Default(bool),
    V4_0(bool),
}

impl crate::Parse for PlayerSettingsMMtRenderingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsMMtRenderingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsMUseDx11Variant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsMUseDx11Variant {
    Default(bool),
    V4_5(bool),
}

impl crate::Parse for PlayerSettingsMUseDx11Variant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsMUseDx11Variant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsMMobileMtRenderingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsMMobileMtRenderingVariant {
    Default(bool),
    V5_4(bool),
}

impl crate::Parse for PlayerSettingsMMobileMtRenderingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsMMobileMtRenderingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsUse32BitDisplayBufferVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsUse32BitDisplayBufferVariant {
    Default(bool),
    V3_5(bool),
}

impl crate::Parse for PlayerSettingsUse32BitDisplayBufferVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V3_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsUse32BitDisplayBufferVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V3_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V3_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsMStackTraceTypesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsMStackTraceTypesVariant {
    Default(Vec<i32>),
    V2017_1(Vec<i32>),
}

impl crate::Parse for PlayerSettingsMStackTraceTypesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_i32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_i32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsMStackTraceTypesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsMSplashScreenLogosVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsMSplashScreenLogosVariant {
    Default(Vec<SplashScreenLogo>),
    V2017_1(Vec<SplashScreenLogo>),
}

impl crate::Parse for PlayerSettingsMSplashScreenLogosVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SplashScreenLogo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SplashScreenLogo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsMSplashScreenLogosVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsUseMacAppStoreValidationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsUseMacAppStoreValidationVariant {
    Default(bool),
    V5_6_1(bool),
}

impl crate::Parse for PlayerSettingsUseMacAppStoreValidationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 1) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_6_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsUseMacAppStoreValidationVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsXboxSkinOnGpuVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsXboxSkinOnGpuVariant {
    Default(bool),
    V3_5(bool),
}

impl crate::Parse for PlayerSettingsXboxSkinOnGpuVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V3_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsXboxSkinOnGpuVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V3_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V3_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsDisableDepthAndStencilBuffersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsDisableDepthAndStencilBuffersVariant {
    Default(bool),
    V2017_2(bool),
    V2017_3(bool),
    V2018_3(bool),
}

impl crate::Parse for PlayerSettingsDisableDepthAndStencilBuffersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_3(v))
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2017_3(v))
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsDisableDepthAndStencilBuffersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_2(v) => v.to_value(),
            Self::V2017_3(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsPrepareIosForRecordingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsPrepareIosForRecordingVariant {
    Default(bool),
    V5_6_5(bool),
    V2017_1(bool),
}

impl crate::Parse for PlayerSettingsPrepareIosForRecordingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 6, 5) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_6_5(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsPrepareIosForRecordingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6_5(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6_5(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsMobileMtRenderingBakedVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsMobileMtRenderingBakedVariant {
    Default(bool),
    V2020_1(bool),
}

impl crate::Parse for PlayerSettingsMobileMtRenderingBakedVariant {
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

impl crate::ToValue for PlayerSettingsMobileMtRenderingBakedVariant {
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

/// 版本枚举：PlayerSettingsXboxEnableFitnessVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsXboxEnableFitnessVariant {
    Default(bool),
    V4_5_4(bool),
}

impl crate::Parse for PlayerSettingsXboxEnableFitnessVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 4) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_5_4(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsXboxEnableFitnessVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsGpuSkinningVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsGpuSkinningVariant {
    Default(bool),
    V2023_1(bool),
}

impl crate::Parse for PlayerSettingsGpuSkinningVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsGpuSkinningVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsXboxEnableHeadOrientationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsXboxEnableHeadOrientationVariant {
    Default(bool),
    V4_3(bool),
    V5_3(bool),
}

impl crate::Parse for PlayerSettingsXboxEnableHeadOrientationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_3(v))
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsXboxEnableHeadOrientationVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_3(v) => v.to_value(),
            Self::V5_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_3(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsVisibleInBackgroundVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsVisibleInBackgroundVariant {
    Default(bool),
    V5_3(bool),
}

impl crate::Parse for PlayerSettingsVisibleInBackgroundVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsVisibleInBackgroundVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsAndroidRenderOutsideSafeAreaVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsAndroidRenderOutsideSafeAreaVariant {
    Default(bool),
    V2019_2(bool),
}

impl crate::Parse for PlayerSettingsAndroidRenderOutsideSafeAreaVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2019_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsAndroidRenderOutsideSafeAreaVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsXboxOneResolutionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsXboxOneResolutionVariant {
    Default(i32),
    V5_3_1(i32),
}

impl crate::Parse for PlayerSettingsXboxOneResolutionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 1) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_3_1(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsXboxOneResolutionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_3_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsForceIosSpeakersWhenRecordingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsForceIosSpeakersWhenRecordingVariant {
    Default(bool),
    V2017_1_2(bool),
}

impl crate::Parse for PlayerSettingsForceIosSpeakersWhenRecordingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 2) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2017_1_2(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsForceIosSpeakersWhenRecordingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsMetroEnableLowLatencyPresentationApiVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsMetroEnableLowLatencyPresentationApiVariant {
    Default(bool),
    V5_0(bool),
}

impl crate::Parse for PlayerSettingsMetroEnableLowLatencyPresentationApiVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsMetroEnableLowLatencyPresentationApiVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsN3dsEnableVSyncVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsN3dsEnableVSyncVariant {
    Default(bool),
    V5_3(bool),
}

impl crate::Parse for PlayerSettingsN3dsEnableVSyncVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsN3dsEnableVSyncVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsPreloadedAssetsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsPreloadedAssetsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for PlayerSettingsPreloadedAssetsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsPreloadedAssetsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsXboxOneDisableKinectGpuReservationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsXboxOneDisableKinectGpuReservationVariant {
    Default(bool),
    V5_6(bool),
}

impl crate::Parse for PlayerSettingsXboxOneDisableKinectGpuReservationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsXboxOneDisableKinectGpuReservationVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsAndroidPredictiveBackSupportVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsAndroidPredictiveBackSupportVariant {
    Default(bool),
    V6000_0_5(bool),
}

impl crate::Parse for PlayerSettingsAndroidPredictiveBackSupportVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 0, 5) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V6000_0_5(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsAndroidPredictiveBackSupportVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_0_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_0_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsMetalFramebufferOnlyVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsMetalFramebufferOnlyVariant {
    Default(bool),
    V2018_3(bool),
}

impl crate::Parse for PlayerSettingsMetalFramebufferOnlyVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsMetalFramebufferOnlyVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsProtectGraphicsMemoryVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsProtectGraphicsMemoryVariant {
    Default(bool),
    V2018_3(bool),
}

impl crate::Parse for PlayerSettingsProtectGraphicsMemoryVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsProtectGraphicsMemoryVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsSwitchQueueCommandMemoryVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsSwitchQueueCommandMemoryVariant {
    Default(i32),
    V2018_3(i32),
    V2018_3_5(i32),
}

impl crate::Parse for PlayerSettingsSwitchQueueCommandMemoryVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 5) {
            let v = rdr.read_i32()?;
            Ok(Self::V2018_3_5(v))
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsSwitchQueueCommandMemoryVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
            Self::V2018_3_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsVulkanEnableSetSrgbWriteVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsVulkanEnableSetSrgbWriteVariant {
    Default(bool),
    V2018_3(bool),
    V2019_4_6(bool),
    V2020_1(bool),
    V2020_1_7(bool),
}

impl crate::Parse for PlayerSettingsVulkanEnableSetSrgbWriteVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 7) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_1_7(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 4, 6) {
            let v = rdr.read_bool()?;
            Ok(Self::V2019_4_6(v))
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsVulkanEnableSetSrgbWriteVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
            Self::V2019_4_6(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_1_7(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_4_6(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1_7(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsSwitchNvnOtherPoolsGranularityVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsSwitchNvnOtherPoolsGranularityVariant {
    Default(i32),
    V2018_4_26(i32),
    V2019_1(i32),
    V2019_3_15(i32),
}

impl crate::Parse for PlayerSettingsSwitchNvnOtherPoolsGranularityVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 15) {
            let v = rdr.read_i32()?;
            Ok(Self::V2019_3_15(v))
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 26) {
            let v = rdr.read_i32()?;
            Ok(Self::V2018_4_26(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsSwitchNvnOtherPoolsGranularityVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_26(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
            Self::V2019_3_15(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_26(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3_15(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant {
    Default(i32),
    V2019_4_6(i32),
}

impl crate::Parse for PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 4, 6) {
            let v = rdr.read_i32()?;
            Ok(Self::V2019_4_6(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_4_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_4_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsVulkanEnableLateAcquireNextImageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsVulkanEnableLateAcquireNextImageVariant {
    Default(bool),
    V2020_3_18(bool),
    V2021_1(bool),
    V2021_1_14(bool),
}

impl crate::Parse for PlayerSettingsVulkanEnableLateAcquireNextImageVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 1, 14) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_1_14(v))
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2021_1(v))
        } else if *ver >= crate::UnityVersion::new(2020, 3, 18) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_3_18(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsVulkanEnableLateAcquireNextImageVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_3_18(v) => v.to_value(),
            Self::V2021_1(v) => v.to_value(),
            Self::V2021_1_14(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_3_18(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_1_14(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsEnableFrameTimingStatsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsEnableFrameTimingStatsVariant {
    Default(bool),
    V2021_3_2(bool),
    V2022_1(bool),
    V2022_2(bool),
}

impl crate::Parse for PlayerSettingsEnableFrameTimingStatsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2022_1(v))
        } else if *ver >= crate::UnityVersion::new(2021, 3, 2) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_3_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsEnableFrameTimingStatsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2021_3_2(v) => v.to_value(),
            Self::V2022_1(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_3_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsResolutionScalingModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsResolutionScalingModeVariant {
    Default(i32),
    V2017_2(i32),
}

impl crate::Parse for PlayerSettingsResolutionScalingModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2017_2(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsResolutionScalingModeVariant {
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

/// 版本枚举：PlayerSettingsAndroidMaxAspectRatioVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsAndroidMaxAspectRatioVariant {
    Default(f32),
    V2023_2(f32),
}

impl crate::Parse for PlayerSettingsAndroidMaxAspectRatioVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = rdr.read_f32()?;
            Ok(Self::V2023_2(v))
        } else {
            let v = rdr.read_f32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsAndroidMaxAspectRatioVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant {
    Default(i32),
    V2023_1(i32),
    V6000_2(i32),
}

impl crate::Parse for PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V6000_2(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
            Self::V6000_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsLegacyClampBlendShapeWeightsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsLegacyClampBlendShapeWeightsVariant {
    Default(bool),
    V2021_2(bool),
}

impl crate::Parse for PlayerSettingsLegacyClampBlendShapeWeightsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2021_2(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsLegacyClampBlendShapeWeightsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2021_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsVirtualTexturingSupportEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsVirtualTexturingSupportEnabledVariant {
    Default(bool),
    V2022_1(bool),
}

impl crate::Parse for PlayerSettingsVirtualTexturingSupportEnabledVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2022_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsVirtualTexturingSupportEnabledVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：PlayerSettingsD3d12DeviceFilterListAssetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PlayerSettingsD3d12DeviceFilterListAssetVariant {
    Default(PPtr),
    V6000_3(PPtr),
}

impl crate::Parse for PlayerSettingsD3d12DeviceFilterListAssetVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::V6000_3(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PlayerSettingsD3d12DeviceFilterListAssetVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PlayerSettings {
    pub android_profiler: PlayerSettingsAndroidProfilerVariant,
    pub product_guid: Option<Guid>,
    pub default_screen_orientation: i32,
    pub target_device: i32,
    pub android_filter_touches_when_obscured:
        Option<PlayerSettingsAndroidFilterTouchesWhenObscuredVariant>,
    pub target_platform: Option<i32>,
    pub target_resolution: Option<PlayerSettingsTargetResolutionVariant>,
    pub target_gles_graphics: Option<i32>,
    pub use_on_demand_resources: Option<bool>,
    pub android_enable_sustained_performance_mode: Option<bool>,
    pub accelerometer_frequency: Option<i32>,
    pub target_ios_graphics: Option<i32>,
    pub company_name: String,
    pub product_name: String,
    pub android_license_public_key: Option<String>,
    pub default_cursor: Option<PPtr>,
    pub default_screen_width: i32,
    pub cursor_hotspot: Option<Vector2f>,
    pub default_screen_height: i32,
    pub cloud_project_id: Option<String>,
    pub m_show_unity_splash_screen: Option<PlayerSettingsMShowUnitySplashScreenVariant>,
    pub default_screen_width_web: i32,
    pub m_virtual_reality_splash_screen: Option<PlayerSettingsMVirtualRealitySplashScreenVariant>,
    pub m_splash_screen_style: Option<i32>,
    pub m_splash_screen_background_color: Option<ColorRgbAf>,
    pub default_screen_height_web: i32,
    pub m_rendering_path: Option<i32>,
    pub m_show_unity_splash_logo: Option<bool>,
    pub display_resolution_dialog: Option<i32>,
    pub m_splash_screen_overlay_opacity: Option<f32>,
    pub allowed_autorotate_to_portrait: bool,
    pub m_active_color_space: Option<i32>,
    pub m_splash_screen_animation: Option<i32>,
    pub allowed_autorotate_to_portrait_upside_down: bool,
    pub m_mt_rendering: Option<PlayerSettingsMMtRenderingVariant>,
    pub m_mobile_rendering_path: Option<i32>,
    pub m_splash_screen_logo_style: Option<i32>,
    pub allowed_autorotate_to_landscape_right: bool,
    pub ios_show_activity_indicator_on_loading: Option<i32>,
    pub m_splash_screen_draw_mode: Option<i32>,
    pub allowed_autorotate_to_landscape_left: bool,
    pub android_show_activity_indicator_on_loading: Option<i32>,
    pub m_use_dx11: Option<PlayerSettingsMUseDx11Variant>,
    pub m_splash_screen_background_animation_zoom: Option<f32>,
    pub use_os_autorotation: bool,
    pub m_mobile_mt_rendering: Option<PlayerSettingsMMobileMtRenderingVariant>,
    pub m_splash_screen_logo_animation_zoom: Option<f32>,
    pub use32_bit_display_buffer: PlayerSettingsUse32BitDisplayBufferVariant,
    pub m_stereoscopic3_d: Option<bool>,
    pub m_splash_screen_background_landscape_aspect: Option<f32>,
    pub default_is_full_screen: Option<bool>,
    pub m_splash_screen_background_portrait_aspect: Option<f32>,
    pub use_alpha_in_dashboard: Option<bool>,
    pub m_splash_screen_background_landscape_uvs: Option<Rectf>,
    pub run_in_background: bool,
    pub ios_app_in_background_behavior: Option<i32>,
    pub m_stack_trace_types: Option<PlayerSettingsMStackTraceTypesVariant>,
    pub m_splash_screen_background_portrait_uvs: Option<Rectf>,
    pub capture_single_screen: Option<bool>,
    pub m_splash_screen_logos: Option<PlayerSettingsMSplashScreenLogosVariant>,
    pub override_i_pod_music: Option<bool>,
    pub m_splash_screen_background_landscape: Option<PPtr>,
    pub use_player_log: bool,
    pub use24_bit_depth_buffer: Option<bool>,
    pub ios_allow_http_download: Option<bool>,
    pub tizen_show_activity_indicator_on_loading: Option<i32>,
    pub m_splash_screen_background_portrait: Option<PPtr>,
    pub use_mac_app_store_validation: PlayerSettingsUseMacAppStoreValidationVariant,
    pub xbox_skin_on_gpu: Option<PlayerSettingsXboxSkinOnGpuVariant>,
    pub m_holographic_tracking_loss_screen: Option<PPtr>,
    pub m_supported_aspect_ratios: Option<AspectRatios>,
    pub m_unity_splash_logo: Option<PPtr>,
    pub default_is_native_resolution: Option<bool>,
    pub disable_depth_and_stencil_buffers:
        Option<PlayerSettingsDisableDepthAndStencilBuffersVariant>,
    pub enable_hw_statistics: Option<bool>,
    pub m_stereo_rendering_path: Option<i32>,
    pub xbox_enable_avatar: Option<bool>,
    pub strip_physics: Option<bool>,
    pub prepare_ios_for_recording: Option<PlayerSettingsPrepareIosForRecordingVariant>,
    pub xbox_enable_kinect: Option<bool>,
    pub resizable_window: Option<bool>,
    pub xbox_enable_kinect_auto_tracking: Option<bool>,
    pub xbox_enable_speech: Option<bool>,
    pub mobile_mt_rendering_baked: Option<PlayerSettingsMobileMtRenderingBakedVariant>,
    pub unsupported_msaa_fallback: Option<i32>,
    pub m_sprite_batch_vertex_threshold: Option<i32>,
    pub wii_hio2_usage: Option<i32>,
    pub force_single_instance: Option<bool>,
    pub submit_analytics: Option<bool>,
    pub m_sprite_batch_max_vertex_count: Option<i32>,
    pub wii_loading_screen_rect_placement: Option<i32>,
    pub mute_other_audio_sources: Option<bool>,
    pub mip_stripping: Option<bool>,
    pub aid: Option<Hash128>,
    pub wii_loading_screen_background: Option<ColorRgbAf>,
    pub bake_collision_meshes: Option<bool>,
    pub number_of_mips_stripped: Option<i32>,
    pub player_min_open_gles_version: Option<i32>,
    pub wii_loading_screen_period: Option<i32>,
    pub xbox_enable_fitness: Option<PlayerSettingsXboxEnableFitnessVariant>,
    pub gpu_skinning: Option<PlayerSettingsGpuSkinningVariant>,
    pub wii_loading_screen_file_name: Option<String>,
    pub mac_fullscreen_mode: Option<i32>,
    pub ios_use_custom_app_background_behavior: Option<bool>,
    pub wii_loading_screen_rect: Option<Rectf>,
    pub xbox_speech_db: Option<u32>,
    pub xbox_pix_texture_capture: Option<bool>,
    pub debug_unload_mode: Option<i32>,
    pub xbox_enable_head_orientation: Option<PlayerSettingsXboxEnableHeadOrientationVariant>,
    pub number_of_mips_stripped_per_mipmap_limit_group: Option<Vec<(String, i32)>>,
    pub graphics_jobs: Option<bool>,
    pub visible_in_background: Option<PlayerSettingsVisibleInBackgroundVariant>,
    pub preserve_framebuffer_alpha: Option<bool>,
    pub d3d9_fullscreen_mode: Option<i32>,
    pub xbox_enable_guest: Option<bool>,
    pub android_blit_type: Option<i32>,
    pub android_start_in_fullscreen: Option<bool>,
    pub i_phone_bundle_identifier: Option<String>,
    pub d3d11_force_exclusive_mode: Option<bool>,
    pub d3d11_fullscreen_mode: Option<i32>,
    pub allow_fullscreen_switch: Option<bool>,
    pub android_render_outside_safe_area: Option<PlayerSettingsAndroidRenderOutsideSafeAreaVariant>,
    pub video_memory_for_vertex_buffers: Option<i32>,
    pub android_use_swappy: Option<bool>,
    pub mac_retina_support: Option<bool>,
    pub android_resizable_window: Option<bool>,
    pub metro_enable_independent_input_source: Option<bool>,
    pub xbox_one_resolution: Option<PlayerSettingsXboxOneResolutionVariant>,
    pub n3ds_disable_stereoscopic_view: Option<bool>,
    pub defer_system_gestures_mode: Option<i32>,
    pub force_ios_speakers_when_recording:
        Option<PlayerSettingsForceIosSpeakersWhenRecordingVariant>,
    pub android_default_window_width: Option<i32>,
    pub metro_enable_low_latency_presentation_api:
        Option<PlayerSettingsMetroEnableLowLatencyPresentationApiVariant>,
    pub ps3_splash_screen: Option<PPtr>,
    pub n3ds_enable_shared_list_opt: Option<bool>,
    pub hide_home_button: Option<bool>,
    pub android_default_window_height: Option<i32>,
    pub adjust_iosfps_using_thermal_state: Option<bool>,
    pub n3ds_enable_v_sync: Option<PlayerSettingsN3dsEnableVSyncVariant>,
    pub android_minimum_window_width: Option<i32>,
    pub thermal_state_serious_iosfps: Option<i32>,
    pub psp2_power_mode: Option<i32>,
    pub xbox_enable_pix_sampling: Option<bool>,
    pub android_minimum_window_height: Option<i32>,
    pub thermal_state_critical_iosfps: Option<i32>,
    pub psp2_acquire_bgm: Option<bool>,
    pub xbox_enable_enable_render_thread_runs_jobs: Option<bool>,
    pub android_fullscreen_mode: Option<i32>,
    pub android_display_options: Option<i32>,
    pub bundle_identifier: Option<String>,
    pub mac_app_store_category: Option<String>,
    pub android_resizeable_activity: Option<bool>,
    pub bundle_version: Option<String>,
    pub ui_use16_bit_depth_buffer: Option<bool>,
    pub preloaded_assets: Option<PlayerSettingsPreloadedAssetsVariant>,
    pub wii_utv_resolution: Option<i32>,
    pub ignore_alpha_clear: Option<bool>,
    pub android_requested_visible_insets: Option<i32>,
    pub wii_u_game_pad_msaa: Option<i32>,
    pub android_system_bars_behavior: Option<i32>,
    pub wii_u_supports_nunchuk: Option<bool>,
    pub android_auto_rotation_behavior: Option<i32>,
    pub xbox_one_disable_kinect_gpu_reservation:
        Option<PlayerSettingsXboxOneDisableKinectGpuReservationVariant>,
    pub wii_u_supports_classic_controller: Option<bool>,
    pub xbox_one_mono_logging_level: Option<i32>,
    pub android_predictive_back_support: Option<PlayerSettingsAndroidPredictiveBackSupportVariant>,
    pub android_application_entry: Option<i32>,
    pub virtual_reality_supported: Option<bool>,
    pub wii_u_supports_balance_board: Option<bool>,
    pub xbox_one_logging_level: Option<i32>,
    pub use_flip_model_swapchain: Option<bool>,
    pub project_id: Option<String>,
    pub wii_u_supports_motion_plus: Option<bool>,
    pub project_name: Option<String>,
    pub wii_u_supports_pro_controller: Option<bool>,
    pub organization_id: Option<String>,
    pub wii_u_allow_screen_capture: Option<bool>,
    pub graphics_job_mode: Option<i32>,
    pub cloud_enabled: Option<bool>,
    pub wii_u_controller_count: Option<i32>,
    pub audio_spatial_experience: Option<i32>,
    pub dedicated_server_optimizations: Option<bool>,
    pub fullscreen_mode: Option<i32>,
    pub call_on_disable_on_asset_bundle_unload: Option<bool>,
    pub metal_framebuffer_only: Option<PlayerSettingsMetalFramebufferOnlyVariant>,
    pub xbox_one_s_resolution: Option<i32>,
    pub xbox_one_x_resolution: Option<i32>,
    pub mesh_deformation: Option<i32>,
    pub xbox_one_disable_esram: Option<bool>,
    pub single_pass_stereo_rendering: Option<bool>,
    pub protect_graphics_memory: Option<PlayerSettingsProtectGraphicsMemoryVariant>,
    pub xbox_one_present_immediate_threshold: Option<u32>,
    pub xbox_one_enable_type_optimization: Option<bool>,
    pub switch_queue_command_memory: Option<PlayerSettingsSwitchQueueCommandMemoryVariant>,
    pub switch_queue_control_memory: Option<i32>,
    pub vulkan_enable_set_srgb_write: Option<PlayerSettingsVulkanEnableSetSrgbWriteVariant>,
    pub switch_queue_compute_memory: Option<i32>,
    pub switch_nvn_shader_pools_granularity: Option<i32>,
    pub switch_nvn_default_pools_granularity: Option<i32>,
    pub switch_nvn_other_pools_granularity:
        Option<PlayerSettingsSwitchNvnOtherPoolsGranularityVariant>,
    pub metro_input_source: Option<i32>,
    pub vulkan_num_swapchain_buffers: Option<u32>,
    pub wsa_transparent_swapchain: Option<bool>,
    pub vulkan_use_sw_command_buffers: Option<bool>,
    pub switch_nvn_max_public_texture_id_count: Option<i32>,
    pub stadia_present_mode: Option<i32>,
    pub m_holographic_pause_on_tracking_loss: Option<bool>,
    pub switch_nvn_max_public_sampler_id_count:
        Option<PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant>,
    pub stadia_target_framerate: Option<i32>,
    pub xbox_one_enable7th_core: Option<bool>,
    pub is_wsa_holographic_remoting_enabled: Option<bool>,
    pub vr_settings: Option<VrSettings>,
    pub metal_use_metal_display_link: Option<bool>,
    pub vulkan_enable_late_acquire_next_image:
        Option<PlayerSettingsVulkanEnableLateAcquireNextImageVariant>,
    pub use_hdr_display: Option<bool>,
    pub enable_frame_timing_stats: Option<PlayerSettingsEnableFrameTimingStatsVariant>,
    pub m_color_gamuts: Option<Vec<i32>>,
    pub target_pixel_density: Option<i32>,
    pub vulkan_enable_pre_transform: Option<bool>,
    pub switch_gpu_scratch_pool_granularity: Option<i32>,
    pub resolution_scaling_mode: Option<PlayerSettingsResolutionScalingModeVariant>,
    pub switch_allow_gpu_scratch_shrinking: Option<bool>,
    pub enable_new_input_system: Option<bool>,
    pub android_supported_aspect_ratio: Option<i32>,
    pub vulkan_enable_command_buffer_recycling: Option<bool>,
    pub switch_max_worker_multiple: Option<i32>,
    pub android_max_aspect_ratio: Option<PlayerSettingsAndroidMaxAspectRatioVariant>,
    pub d3dhdr_bit_depth: Option<i32>,
    pub switch_nvn_graphics_firmware_memory:
        Option<PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant>,
    pub framebuffer_depth_memoryless_mode: Option<i32>,
    pub enable_native_platform_backends_for_new_input_system: Option<bool>,
    pub disable_old_input_manager_support: Option<bool>,
    pub legacy_clamp_blend_shape_weights: Option<PlayerSettingsLegacyClampBlendShapeWeightsVariant>,
    pub load_store_debug_mode_enabled: Option<bool>,
    pub switch_graphics_jobs_sync_after_kick: Option<bool>,
    pub invalidated_pattern_texture: Option<PPtr>,
    pub active_input_handler: Option<i32>,
    pub enable_open_gl_profiler_gpu_recorders: Option<bool>,
    pub vision_os_bundle_version: Option<String>,
    pub tv_os_bundle_version: Option<String>,
    pub virtual_texturing_support_enabled:
        Option<PlayerSettingsVirtualTexturingSupportEnabledVariant>,
    pub quality_settings_names: Option<Vec<String>>,
    pub reset_resolution_on_window_resize: Option<bool>,
    pub hdr_bit_depth: Option<i32>,
    pub windows_gamepad_backend_hint: Option<i32>,
    pub allow_hdr_display_support: Option<bool>,
    pub player_data_path: Option<String>,
    pub force_srgb_blit: Option<bool>,
    pub upload_cleared_texture_data_after_creation_from_script: Option<bool>,
    pub android_min_aspect_ratio: Option<f32>,
    pub insecure_http_option: Option<i32>,
    pub hmi_loading_image: Option<PPtr>,
    pub platform_requires_readable_assets: Option<bool>,
    pub enable_direct_storage: Option<bool>,
    pub android_vulkan_deny_filter_list: Option<Vec<AndroidDeviceFilterData>>,
    pub android_vulkan_allow_filter_list: Option<Vec<AndroidDeviceFilterData>>,
    pub android_vulkan_device_filter_list_asset: Option<PPtr>,
    pub d3d12_device_filter_list_asset: Option<PlayerSettingsD3d12DeviceFilterListAssetVariant>,
    pub allowed_http_connections: Option<i32>,
}

impl crate::Parse for PlayerSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut android_profiler = None;
        let mut product_guid = None;
        let mut default_screen_orientation = None;
        let mut target_device = None;
        let mut android_filter_touches_when_obscured = None;
        let mut target_platform = None;
        let mut target_resolution = None;
        let mut target_gles_graphics = None;
        let mut use_on_demand_resources = None;
        let mut android_enable_sustained_performance_mode = None;
        let mut accelerometer_frequency = None;
        let mut target_ios_graphics = None;
        let mut company_name = None;
        let mut product_name = None;
        let mut android_license_public_key = None;
        let mut default_cursor = None;
        let mut default_screen_width = None;
        let mut cursor_hotspot = None;
        let mut default_screen_height = None;
        let mut cloud_project_id = None;
        let mut m_show_unity_splash_screen = None;
        let mut default_screen_width_web = None;
        let mut m_virtual_reality_splash_screen = None;
        let mut m_splash_screen_style = None;
        let mut m_splash_screen_background_color = None;
        let mut default_screen_height_web = None;
        let mut m_rendering_path = None;
        let mut m_show_unity_splash_logo = None;
        let mut display_resolution_dialog = None;
        let mut m_splash_screen_overlay_opacity = None;
        let mut allowed_autorotate_to_portrait = None;
        let mut m_active_color_space = None;
        let mut m_splash_screen_animation = None;
        let mut allowed_autorotate_to_portrait_upside_down = None;
        let mut m_mt_rendering = None;
        let mut m_mobile_rendering_path = None;
        let mut m_splash_screen_logo_style = None;
        let mut allowed_autorotate_to_landscape_right = None;
        let mut ios_show_activity_indicator_on_loading = None;
        let mut m_splash_screen_draw_mode = None;
        let mut allowed_autorotate_to_landscape_left = None;
        let mut android_show_activity_indicator_on_loading = None;
        let mut m_use_dx11 = None;
        let mut m_splash_screen_background_animation_zoom = None;
        let mut use_os_autorotation = None;
        let mut m_mobile_mt_rendering = None;
        let mut m_splash_screen_logo_animation_zoom = None;
        let mut use32_bit_display_buffer = None;
        let mut m_stereoscopic3_d = None;
        let mut m_splash_screen_background_landscape_aspect = None;
        let mut default_is_full_screen = None;
        let mut m_splash_screen_background_portrait_aspect = None;
        let mut use_alpha_in_dashboard = None;
        let mut m_splash_screen_background_landscape_uvs = None;
        let mut run_in_background = None;
        let mut ios_app_in_background_behavior = None;
        let mut m_stack_trace_types = None;
        let mut m_splash_screen_background_portrait_uvs = None;
        let mut capture_single_screen = None;
        let mut m_splash_screen_logos = None;
        let mut override_i_pod_music = None;
        let mut m_splash_screen_background_landscape = None;
        let mut use_player_log = None;
        let mut use24_bit_depth_buffer = None;
        let mut ios_allow_http_download = None;
        let mut tizen_show_activity_indicator_on_loading = None;
        let mut m_splash_screen_background_portrait = None;
        let mut use_mac_app_store_validation = None;
        let mut xbox_skin_on_gpu = None;
        let mut m_holographic_tracking_loss_screen = None;
        let mut m_supported_aspect_ratios = None;
        let mut m_unity_splash_logo = None;
        let mut default_is_native_resolution = None;
        let mut disable_depth_and_stencil_buffers = None;
        let mut enable_hw_statistics = None;
        let mut m_stereo_rendering_path = None;
        let mut xbox_enable_avatar = None;
        let mut strip_physics = None;
        let mut prepare_ios_for_recording = None;
        let mut xbox_enable_kinect = None;
        let mut resizable_window = None;
        let mut xbox_enable_kinect_auto_tracking = None;
        let mut xbox_enable_speech = None;
        let mut mobile_mt_rendering_baked = None;
        let mut unsupported_msaa_fallback = None;
        let mut m_sprite_batch_vertex_threshold = None;
        let mut wii_hio2_usage = None;
        let mut force_single_instance = None;
        let mut submit_analytics = None;
        let mut m_sprite_batch_max_vertex_count = None;
        let mut wii_loading_screen_rect_placement = None;
        let mut mute_other_audio_sources = None;
        let mut mip_stripping = None;
        let mut aid = None;
        let mut wii_loading_screen_background = None;
        let mut bake_collision_meshes = None;
        let mut number_of_mips_stripped = None;
        let mut player_min_open_gles_version = None;
        let mut wii_loading_screen_period = None;
        let mut xbox_enable_fitness = None;
        let mut gpu_skinning = None;
        let mut wii_loading_screen_file_name = None;
        let mut mac_fullscreen_mode = None;
        let mut ios_use_custom_app_background_behavior = None;
        let mut wii_loading_screen_rect = None;
        let mut xbox_speech_db = None;
        let mut xbox_pix_texture_capture = None;
        let mut debug_unload_mode = None;
        let mut xbox_enable_head_orientation = None;
        let mut number_of_mips_stripped_per_mipmap_limit_group = None;
        let mut graphics_jobs = None;
        let mut visible_in_background = None;
        let mut preserve_framebuffer_alpha = None;
        let mut d3d9_fullscreen_mode = None;
        let mut xbox_enable_guest = None;
        let mut android_blit_type = None;
        let mut android_start_in_fullscreen = None;
        let mut i_phone_bundle_identifier = None;
        let mut d3d11_force_exclusive_mode = None;
        let mut d3d11_fullscreen_mode = None;
        let mut allow_fullscreen_switch = None;
        let mut android_render_outside_safe_area = None;
        let mut video_memory_for_vertex_buffers = None;
        let mut android_use_swappy = None;
        let mut mac_retina_support = None;
        let mut android_resizable_window = None;
        let mut metro_enable_independent_input_source = None;
        let mut xbox_one_resolution = None;
        let mut n3ds_disable_stereoscopic_view = None;
        let mut defer_system_gestures_mode = None;
        let mut force_ios_speakers_when_recording = None;
        let mut android_default_window_width = None;
        let mut metro_enable_low_latency_presentation_api = None;
        let mut ps3_splash_screen = None;
        let mut n3ds_enable_shared_list_opt = None;
        let mut hide_home_button = None;
        let mut android_default_window_height = None;
        let mut adjust_iosfps_using_thermal_state = None;
        let mut n3ds_enable_v_sync = None;
        let mut android_minimum_window_width = None;
        let mut thermal_state_serious_iosfps = None;
        let mut psp2_power_mode = None;
        let mut xbox_enable_pix_sampling = None;
        let mut android_minimum_window_height = None;
        let mut thermal_state_critical_iosfps = None;
        let mut psp2_acquire_bgm = None;
        let mut xbox_enable_enable_render_thread_runs_jobs = None;
        let mut android_fullscreen_mode = None;
        let mut android_display_options = None;
        let mut bundle_identifier = None;
        let mut mac_app_store_category = None;
        let mut android_resizeable_activity = None;
        let mut bundle_version = None;
        let mut ui_use16_bit_depth_buffer = None;
        let mut preloaded_assets = None;
        let mut wii_utv_resolution = None;
        let mut ignore_alpha_clear = None;
        let mut android_requested_visible_insets = None;
        let mut wii_u_game_pad_msaa = None;
        let mut android_system_bars_behavior = None;
        let mut wii_u_supports_nunchuk = None;
        let mut android_auto_rotation_behavior = None;
        let mut xbox_one_disable_kinect_gpu_reservation = None;
        let mut wii_u_supports_classic_controller = None;
        let mut xbox_one_mono_logging_level = None;
        let mut android_predictive_back_support = None;
        let mut android_application_entry = None;
        let mut virtual_reality_supported = None;
        let mut wii_u_supports_balance_board = None;
        let mut xbox_one_logging_level = None;
        let mut use_flip_model_swapchain = None;
        let mut project_id = None;
        let mut wii_u_supports_motion_plus = None;
        let mut project_name = None;
        let mut wii_u_supports_pro_controller = None;
        let mut organization_id = None;
        let mut wii_u_allow_screen_capture = None;
        let mut graphics_job_mode = None;
        let mut cloud_enabled = None;
        let mut wii_u_controller_count = None;
        let mut audio_spatial_experience = None;
        let mut dedicated_server_optimizations = None;
        let mut fullscreen_mode = None;
        let mut call_on_disable_on_asset_bundle_unload = None;
        let mut metal_framebuffer_only = None;
        let mut xbox_one_s_resolution = None;
        let mut xbox_one_x_resolution = None;
        let mut mesh_deformation = None;
        let mut xbox_one_disable_esram = None;
        let mut single_pass_stereo_rendering = None;
        let mut protect_graphics_memory = None;
        let mut xbox_one_present_immediate_threshold = None;
        let mut xbox_one_enable_type_optimization = None;
        let mut switch_queue_command_memory = None;
        let mut switch_queue_control_memory = None;
        let mut vulkan_enable_set_srgb_write = None;
        let mut switch_queue_compute_memory = None;
        let mut switch_nvn_shader_pools_granularity = None;
        let mut switch_nvn_default_pools_granularity = None;
        let mut switch_nvn_other_pools_granularity = None;
        let mut metro_input_source = None;
        let mut vulkan_num_swapchain_buffers = None;
        let mut wsa_transparent_swapchain = None;
        let mut vulkan_use_sw_command_buffers = None;
        let mut switch_nvn_max_public_texture_id_count = None;
        let mut stadia_present_mode = None;
        let mut m_holographic_pause_on_tracking_loss = None;
        let mut switch_nvn_max_public_sampler_id_count = None;
        let mut stadia_target_framerate = None;
        let mut xbox_one_enable7th_core = None;
        let mut is_wsa_holographic_remoting_enabled = None;
        let mut vr_settings = None;
        let mut metal_use_metal_display_link = None;
        let mut vulkan_enable_late_acquire_next_image = None;
        let mut use_hdr_display = None;
        let mut enable_frame_timing_stats = None;
        let mut m_color_gamuts = None;
        let mut target_pixel_density = None;
        let mut vulkan_enable_pre_transform = None;
        let mut switch_gpu_scratch_pool_granularity = None;
        let mut resolution_scaling_mode = None;
        let mut switch_allow_gpu_scratch_shrinking = None;
        let mut enable_new_input_system = None;
        let mut android_supported_aspect_ratio = None;
        let mut vulkan_enable_command_buffer_recycling = None;
        let mut switch_max_worker_multiple = None;
        let mut android_max_aspect_ratio = None;
        let mut d3dhdr_bit_depth = None;
        let mut switch_nvn_graphics_firmware_memory = None;
        let mut framebuffer_depth_memoryless_mode = None;
        let mut enable_native_platform_backends_for_new_input_system = None;
        let mut disable_old_input_manager_support = None;
        let mut legacy_clamp_blend_shape_weights = None;
        let mut load_store_debug_mode_enabled = None;
        let mut switch_graphics_jobs_sync_after_kick = None;
        let mut invalidated_pattern_texture = None;
        let mut active_input_handler = None;
        let mut enable_open_gl_profiler_gpu_recorders = None;
        let mut vision_os_bundle_version = None;
        let mut tv_os_bundle_version = None;
        let mut virtual_texturing_support_enabled = None;
        let mut quality_settings_names = None;
        let mut reset_resolution_on_window_resize = None;
        let mut hdr_bit_depth = None;
        let mut windows_gamepad_backend_hint = None;
        let mut allow_hdr_display_support = None;
        let mut player_data_path = None;
        let mut force_srgb_blit = None;
        let mut upload_cleared_texture_data_after_creation_from_script = None;
        let mut android_min_aspect_ratio = None;
        let mut insecure_http_option = None;
        let mut hmi_loading_image = None;
        let mut platform_requires_readable_assets = None;
        let mut enable_direct_storage = None;
        let mut android_vulkan_deny_filter_list = None;
        let mut android_vulkan_allow_filter_list = None;
        let mut android_vulkan_device_filter_list_asset = None;
        let mut d3d12_device_filter_list_asset = None;
        let mut allowed_http_connections = None;
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            product_guid = Some(Guid::parse(rdr, ver)?);
        } else {
        }
        android_profiler = Some(PlayerSettingsAndroidProfilerVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            android_filter_touches_when_obscured =
                Some(PlayerSettingsAndroidFilterTouchesWhenObscuredVariant::parse(rdr, ver)?);
            android_enable_sustained_performance_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            android_filter_touches_when_obscured =
                Some(PlayerSettingsAndroidFilterTouchesWhenObscuredVariant::parse(rdr, ver)?);
        } else {
        }
        default_screen_orientation = Some(rdr.read_i32()?);
        target_device = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            use_on_demand_resources = Some(rdr.read_bool()?);
            rdr.align(4)?;
            accelerometer_frequency = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 2, 1) {
            target_resolution = Some(PlayerSettingsTargetResolutionVariant::parse(rdr, ver)?);
            use_on_demand_resources = Some(rdr.read_bool()?);
            rdr.align(4)?;
            accelerometer_frequency = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            target_resolution = Some(PlayerSettingsTargetResolutionVariant::parse(rdr, ver)?);
            accelerometer_frequency = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 6, 3) {
            target_gles_graphics = Some(rdr.read_i32()?);
            target_ios_graphics = Some(rdr.read_i32()?);
            target_resolution = Some(PlayerSettingsTargetResolutionVariant::parse(rdr, ver)?);
            accelerometer_frequency = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            target_gles_graphics = Some(rdr.read_i32()?);
            target_resolution = Some(PlayerSettingsTargetResolutionVariant::parse(rdr, ver)?);
            accelerometer_frequency = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            target_platform = Some(rdr.read_i32()?);
            target_resolution = Some(PlayerSettingsTargetResolutionVariant::parse(rdr, ver)?);
            accelerometer_frequency = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else {
            target_platform = Some(rdr.read_i32()?);
            target_resolution = Some(PlayerSettingsTargetResolutionVariant::parse(rdr, ver)?);
        }
        company_name = Some(rdr.read_aligned_string()?);
        product_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_splash_screen_background_color = Some(ColorRgbAf::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(
                rdr, ver,
            )?);
            m_show_unity_splash_logo = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_splash_screen_overlay_opacity = Some(rdr.read_f32()?);
            m_splash_screen_animation = Some(rdr.read_i32()?);
            m_splash_screen_logo_style = Some(rdr.read_i32()?);
            m_splash_screen_draw_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_splash_screen_background_animation_zoom = Some(rdr.read_f32()?);
            m_splash_screen_logo_animation_zoom = Some(rdr.read_f32()?);
            m_splash_screen_background_landscape_aspect = Some(rdr.read_f32()?);
            m_splash_screen_background_portrait_aspect = Some(rdr.read_f32()?);
            m_splash_screen_background_landscape_uvs = Some(Rectf::parse(rdr, ver)?);
            m_splash_screen_background_portrait_uvs = Some(Rectf::parse(rdr, ver)?);
            m_splash_screen_logos = Some(PlayerSettingsMSplashScreenLogosVariant::parse(rdr, ver)?);
            m_splash_screen_background_landscape = Some(crate::PPtr::parse(rdr, ver)?);
            m_splash_screen_background_portrait = Some(crate::PPtr::parse(rdr, ver)?);
            m_unity_splash_logo = Some(crate::PPtr::parse(rdr, ver)?);
            m_virtual_reality_splash_screen = Some(
                PlayerSettingsMVirtualRealitySplashScreenVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_splash_screen_background_color = Some(ColorRgbAf::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(
                rdr, ver,
            )?);
            m_show_unity_splash_logo = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_splash_screen_overlay_opacity = Some(rdr.read_f32()?);
            m_splash_screen_animation = Some(rdr.read_i32()?);
            m_splash_screen_logo_style = Some(rdr.read_i32()?);
            m_splash_screen_draw_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_splash_screen_background_animation_zoom = Some(rdr.read_f32()?);
            m_splash_screen_logo_animation_zoom = Some(rdr.read_f32()?);
            m_splash_screen_background_landscape_aspect = Some(rdr.read_f32()?);
            m_splash_screen_background_portrait_aspect = Some(rdr.read_f32()?);
            m_splash_screen_background_landscape_uvs = Some(Rectf::parse(rdr, ver)?);
            m_splash_screen_background_portrait_uvs = Some(Rectf::parse(rdr, ver)?);
            m_splash_screen_logos = Some(PlayerSettingsMSplashScreenLogosVariant::parse(rdr, ver)?);
            m_splash_screen_background_landscape = Some(crate::PPtr::parse(rdr, ver)?);
            m_splash_screen_background_portrait = Some(crate::PPtr::parse(rdr, ver)?);
            m_unity_splash_logo = Some(crate::PPtr::parse(rdr, ver)?);
            m_virtual_reality_splash_screen = Some(
                PlayerSettingsMVirtualRealitySplashScreenVariant::parse(rdr, ver)?,
            );
            m_holographic_tracking_loss_screen = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_splash_screen_background_color = Some(ColorRgbAf::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(
                rdr, ver,
            )?);
            m_show_unity_splash_logo = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_splash_screen_overlay_opacity = Some(rdr.read_f32()?);
            m_splash_screen_animation = Some(rdr.read_i32()?);
            m_splash_screen_logo_style = Some(rdr.read_i32()?);
            m_splash_screen_draw_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_splash_screen_background_animation_zoom = Some(rdr.read_f32()?);
            m_splash_screen_logo_animation_zoom = Some(rdr.read_f32()?);
            m_splash_screen_background_landscape_aspect = Some(rdr.read_f32()?);
            m_splash_screen_background_portrait_aspect = Some(rdr.read_f32()?);
            m_splash_screen_background_landscape_uvs = Some(Rectf::parse(rdr, ver)?);
            m_splash_screen_background_portrait_uvs = Some(Rectf::parse(rdr, ver)?);
            m_splash_screen_logos = Some(PlayerSettingsMSplashScreenLogosVariant::parse(rdr, ver)?);
            m_splash_screen_background_landscape = Some(crate::PPtr::parse(rdr, ver)?);
            m_splash_screen_background_portrait = Some(crate::PPtr::parse(rdr, ver)?);
            m_virtual_reality_splash_screen = Some(
                PlayerSettingsMVirtualRealitySplashScreenVariant::parse(rdr, ver)?,
            );
            m_holographic_tracking_loss_screen = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_splash_screen_style = Some(rdr.read_i32()?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(
                rdr, ver,
            )?);
            m_virtual_reality_splash_screen = Some(
                PlayerSettingsMVirtualRealitySplashScreenVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(
                rdr, ver,
            )?);
            m_virtual_reality_splash_screen = Some(
                PlayerSettingsMVirtualRealitySplashScreenVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 1) {
            cloud_project_id = Some(rdr.read_aligned_string()?);
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            cloud_project_id = Some(rdr.read_aligned_string()?);
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
        } else {
            android_license_public_key = Some(rdr.read_aligned_string()?);
        }
        default_screen_width = Some(rdr.read_i32()?);
        default_screen_height = Some(rdr.read_i32()?);
        default_screen_width_web = Some(rdr.read_i32()?);
        default_screen_height_web = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 0, 20) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            unsupported_msaa_fallback = Some(rdr.read_i32()?);
            m_sprite_batch_max_vertex_count = Some(rdr.read_i32()?);
            m_sprite_batch_vertex_threshold = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            number_of_mips_stripped_per_mipmap_limit_group = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 17) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            unsupported_msaa_fallback = Some(rdr.read_i32()?);
            m_sprite_batch_vertex_threshold = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            number_of_mips_stripped_per_mipmap_limit_group = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_sprite_batch_vertex_threshold = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            number_of_mips_stripped_per_mipmap_limit_group = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            unsupported_msaa_fallback = Some(rdr.read_i32()?);
            m_sprite_batch_vertex_threshold = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            number_of_mips_stripped_per_mipmap_limit_group = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 3) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_sprite_batch_vertex_threshold = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            number_of_mips_stripped_per_mipmap_limit_group = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_sprite_batch_vertex_threshold = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 32) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            unsupported_msaa_fallback = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_app_in_background_behavior = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 10) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_app_in_background_behavior = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            tizen_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_app_in_background_behavior = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 4, 40) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            tizen_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 4, 33) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            tizen_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_app_in_background_behavior = Some(rdr.read_i32()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(
                rdr, ver,
            )?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            tizen_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_app_in_background_behavior = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            m_mobile_mt_rendering = Some(PlayerSettingsMMobileMtRenderingVariant::parse(rdr, ver)?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            tizen_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_app_in_background_behavior = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 3) {
            m_rendering_path = Some(rdr.read_i32()?);
            m_mobile_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            m_mobile_mt_rendering = Some(PlayerSettingsMMobileMtRenderingVariant::parse(rdr, ver)?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            tizen_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_app_in_background_behavior = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_rendering_path = Some(rdr.read_i32()?);
            m_mobile_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            m_mobile_mt_rendering = Some(PlayerSettingsMMobileMtRenderingVariant::parse(rdr, ver)?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_app_in_background_behavior = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 1) {
            m_rendering_path = Some(rdr.read_i32()?);
            m_mobile_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            m_mobile_mt_rendering = Some(PlayerSettingsMMobileMtRenderingVariant::parse(rdr, ver)?);
            m_stereoscopic3_d = Some(rdr.read_bool()?);
            rdr.align(4)?;
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_app_in_background_behavior = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_rendering_path = Some(rdr.read_i32()?);
            m_mobile_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            m_mobile_mt_rendering = Some(PlayerSettingsMMobileMtRenderingVariant::parse(rdr, ver)?);
            m_stereoscopic3_d = Some(rdr.read_bool()?);
            rdr.align(4)?;
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_app_in_background_behavior = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_rendering_path = Some(rdr.read_i32()?);
            m_mobile_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            m_mobile_mt_rendering = Some(PlayerSettingsMMobileMtRenderingVariant::parse(rdr, ver)?);
            m_use_dx11 = Some(PlayerSettingsMUseDx11Variant::parse(rdr, ver)?);
            m_stereoscopic3_d = Some(rdr.read_bool()?);
            rdr.align(4)?;
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_app_in_background_behavior = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_rendering_path = Some(rdr.read_i32()?);
            m_mobile_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            m_mobile_mt_rendering = Some(PlayerSettingsMMobileMtRenderingVariant::parse(rdr, ver)?);
            m_use_dx11 = Some(PlayerSettingsMUseDx11Variant::parse(rdr, ver)?);
            m_stereoscopic3_d = Some(rdr.read_bool()?);
            rdr.align(4)?;
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_rendering_path = Some(rdr.read_i32()?);
            m_mobile_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            m_mobile_mt_rendering = Some(PlayerSettingsMMobileMtRenderingVariant::parse(rdr, ver)?);
            m_use_dx11 = Some(PlayerSettingsMUseDx11Variant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            m_use_dx11 = Some(PlayerSettingsMUseDx11Variant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
        } else {
            m_rendering_path = Some(rdr.read_i32()?);
            display_resolution_dialog = Some(rdr.read_i32()?);
        }
        allowed_autorotate_to_portrait = Some(rdr.read_bool()?);
        allowed_autorotate_to_portrait_upside_down = Some(rdr.read_bool()?);
        allowed_autorotate_to_landscape_right = Some(rdr.read_bool()?);
        allowed_autorotate_to_landscape_left = Some(rdr.read_bool()?);
        use_os_autorotation = Some(rdr.read_bool()?);
        use32_bit_display_buffer =
            Some(PlayerSettingsUse32BitDisplayBufferVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            adjust_iosfps_using_thermal_state = Some(rdr.read_bool()?);
            rdr.align(4)?;
            thermal_state_serious_iosfps = Some(rdr.read_i32()?);
            thermal_state_critical_iosfps = Some(rdr.read_i32()?);
            rdr.align(4)?;
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_requested_visible_insets = Some(rdr.read_i32()?);
            android_system_bars_behavior = Some(rdr.read_i32()?);
            android_display_options = Some(rdr.read_i32()?);
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizeable_activity = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            android_predictive_back_support = Some(
                PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?,
            );
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 2, 8) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_display_options = Some(rdr.read_i32()?);
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizeable_activity = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            android_predictive_back_support = Some(
                PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?,
            );
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizeable_activity = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            android_predictive_back_support = Some(
                PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?,
            );
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 60) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_display_options = Some(rdr.read_i32()?);
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizeable_activity = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            android_predictive_back_support = Some(
                PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?,
            );
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 5) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizeable_activity = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            android_predictive_back_support = Some(
                PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?,
            );
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 4) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizeable_activity = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            android_application_entry = Some(rdr.read_i32()?);
            android_predictive_back_support = Some(
                PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?,
            );
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            android_application_entry = Some(rdr.read_i32()?);
            android_predictive_back_support = Some(
                PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?,
            );
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 5) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 68) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_display_options = Some(rdr.read_i32()?);
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            android_predictive_back_support = Some(
                PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?,
            );
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 61) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            android_predictive_back_support = Some(
                PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?,
            );
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 17) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 51) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            android_predictive_back_support = Some(
                PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?,
            );
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 35) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            android_auto_rotation_behavior = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 18) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 17) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 4, 31) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            android_resizable_window = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_default_window_width = Some(rdr.read_i32()?);
            android_default_window_height = Some(rdr.read_i32()?);
            android_minimum_window_width = Some(rdr.read_i32()?);
            android_minimum_window_height = Some(rdr.read_i32()?);
            android_fullscreen_mode = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(
                PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?,
            );
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_full_screen = Some(rdr.read_bool()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_full_screen = Some(rdr.read_bool()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            disable_depth_and_stencil_buffers = Some(
                PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?,
            );
            default_is_full_screen = Some(rdr.read_bool()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            use24_bit_depth_buffer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            default_is_full_screen = Some(rdr.read_bool()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            use24_bit_depth_buffer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            default_is_full_screen = Some(rdr.read_bool()?);
        } else {
            default_is_full_screen = Some(rdr.read_bool()?);
            use_alpha_in_dashboard = Some(rdr.read_bool()?);
        }
        run_in_background = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            call_on_disable_on_asset_bundle_unload = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            force_ios_speakers_when_recording = Some(
                PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?,
            );
            audio_spatial_experience = Some(rdr.read_i32()?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 2, 5) {
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            force_ios_speakers_when_recording = Some(
                PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?,
            );
            audio_spatial_experience = Some(rdr.read_i32()?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            force_ios_speakers_when_recording = Some(
                PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?,
            );
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 58) {
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            force_ios_speakers_when_recording = Some(
                PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?,
            );
            audio_spatial_experience = Some(rdr.read_i32()?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 15) {
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            force_ios_speakers_when_recording = Some(
                PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?,
            );
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            force_ios_speakers_when_recording = Some(
                PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?,
            );
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 49) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            force_ios_speakers_when_recording = Some(
                PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?,
            );
            audio_spatial_experience = Some(rdr.read_i32()?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 2) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            force_ios_speakers_when_recording = Some(
                PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?,
            );
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            force_ios_speakers_when_recording = Some(
                PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?,
            );
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 5) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 3) {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 8) {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(
                rdr, ver,
            )?);
            enable_hw_statistics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
            enable_hw_statistics = Some(rdr.read_bool()?);
        } else {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
        }
        use_player_log = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2023, 1, 9) {
            dedicated_server_optimizations = Some(rdr.read_bool()?);
            bake_collision_meshes = Some(rdr.read_bool()?);
            force_single_instance = Some(rdr.read_bool()?);
            use_flip_model_swapchain = Some(rdr.read_bool()?);
            resizable_window = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            bake_collision_meshes = Some(rdr.read_bool()?);
            force_single_instance = Some(rdr.read_bool()?);
            use_flip_model_swapchain = Some(rdr.read_bool()?);
            resizable_window = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 8) {
            dedicated_server_optimizations = Some(rdr.read_bool()?);
            bake_collision_meshes = Some(rdr.read_bool()?);
            force_single_instance = Some(rdr.read_bool()?);
            use_flip_model_swapchain = Some(rdr.read_bool()?);
            resizable_window = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 7) {
            bake_collision_meshes = Some(rdr.read_bool()?);
            force_single_instance = Some(rdr.read_bool()?);
            use_flip_model_swapchain = Some(rdr.read_bool()?);
            resizable_window = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            bake_collision_meshes = Some(rdr.read_bool()?);
            force_single_instance = Some(rdr.read_bool()?);
            resizable_window = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            strip_physics = Some(rdr.read_bool()?);
            force_single_instance = Some(rdr.read_bool()?);
            resizable_window = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            strip_physics = Some(rdr.read_bool()?);
            resizable_window = Some(rdr.read_bool()?);
        } else {
        }
        use_mac_app_store_validation = Some(PlayerSettingsUseMacAppStoreValidationVariant::parse(
            rdr, ver,
        )?);
        if *ver >= crate::UnityVersion::new(6000, 4, 1) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            metal_use_metal_display_link = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            enable_direct_storage = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
            d3d12_device_filter_list_asset = Some(
                PlayerSettingsD3d12DeviceFilterListAssetVariant::parse(rdr, ver)?,
            );
            allowed_http_connections = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            metal_use_metal_display_link = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
            d3d12_device_filter_list_asset = Some(
                PlayerSettingsD3d12DeviceFilterListAssetVariant::parse(rdr, ver)?,
            );
            allowed_http_connections = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 3, 11) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            metal_use_metal_display_link = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
            d3d12_device_filter_list_asset = Some(
                PlayerSettingsD3d12DeviceFilterListAssetVariant::parse(rdr, ver)?,
            );
            allowed_http_connections = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
            d3d12_device_filter_list_asset = Some(
                PlayerSettingsD3d12DeviceFilterListAssetVariant::parse(rdr, ver)?,
            );
            allowed_http_connections = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 2, 7) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
            d3d12_device_filter_list_asset = Some(
                PlayerSettingsD3d12DeviceFilterListAssetVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 5) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 12) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 10) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 18) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 11) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 8) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 19) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 15) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 3) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 2) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            player_data_path = Some(rdr.read_aligned_string()?);
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(
                PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?,
            );
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            player_data_path = Some(rdr.read_aligned_string()?);
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 30) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            player_data_path = Some(rdr.read_aligned_string()?);
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 3, 13) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            player_data_path = Some(rdr.read_aligned_string()?);
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 3, 3) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            player_data_path = Some(rdr.read_aligned_string()?);
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 3, 2) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            player_data_path = Some(rdr.read_aligned_string()?);
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 2, 4) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            player_data_path = Some(rdr.read_aligned_string()?);
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            player_data_path = Some(rdr.read_aligned_string()?);
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
            upload_cleared_texture_data_after_creation_from_script = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 14) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 3, 42) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 3, 35) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 3, 18) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 1, 7) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 1, 1) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
            virtual_texturing_support_enabled = Some(
                PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 4, 6) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_enable_late_acquire_next_image =
                Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 3, 15) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 3, 2) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2018, 4, 26) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count =
                Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2018, 4, 17) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_enable_type_optimization = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2018, 3, 5) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(
                PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?,
            );
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(
                rdr, ver,
            )?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(
                PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2018, 2, 9) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_use_sw_command_buffers = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_enable_set_srgb_write = Some(
                PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?,
            );
            vulkan_use_sw_command_buffers = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(
                PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?,
            );
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only =
                Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio =
                Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 2) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode =
                Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 5) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_new_input_system = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 1) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_new_input_system = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
            enable_new_input_system = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 6) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ui_use16_bit_depth_buffer = Some(rdr.read_bool()?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 4) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ui_use16_bit_depth_buffer = Some(rdr.read_bool()?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 2) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ui_use16_bit_depth_buffer = Some(rdr.read_bool()?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ui_use16_bit_depth_buffer = Some(rdr.read_bool()?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 1) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ui_use16_bit_depth_buffer = Some(rdr.read_bool()?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            single_pass_stereo_rendering = Some(rdr.read_bool()?);
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ui_use16_bit_depth_buffer = Some(rdr.read_bool()?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            single_pass_stereo_rendering = Some(rdr.read_bool()?);
            protect_graphics_memory =
                Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 7) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_enable_render_thread_runs_jobs = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ui_use16_bit_depth_buffer = Some(rdr.read_bool()?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 6) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_enable_render_thread_runs_jobs = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ui_use16_bit_depth_buffer = Some(rdr.read_bool()?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 1) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ui_use16_bit_depth_buffer = Some(rdr.read_bool()?);
            ignore_alpha_clear = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            ui_use16_bit_depth_buffer = Some(rdr.read_bool()?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            n3ds_disable_stereoscopic_view = Some(rdr.read_bool()?);
            n3ds_enable_shared_list_opt = Some(rdr.read_bool()?);
            n3ds_enable_v_sync = Some(PlayerSettingsN3dsEnableVSyncVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_utv_resolution = Some(rdr.read_i32()?);
            wii_u_game_pad_msaa = Some(rdr.read_i32()?);
            wii_u_supports_nunchuk = Some(rdr.read_bool()?);
            wii_u_supports_classic_controller = Some(rdr.read_bool()?);
            wii_u_supports_balance_board = Some(rdr.read_bool()?);
            wii_u_supports_motion_plus = Some(rdr.read_bool()?);
            wii_u_supports_pro_controller = Some(rdr.read_bool()?);
            wii_u_allow_screen_capture = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_u_controller_count = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 1) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            project_id = Some(rdr.read_aligned_string()?);
            project_name = Some(rdr.read_aligned_string()?);
            organization_id = Some(rdr.read_aligned_string()?);
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            bundle_identifier = Some(rdr.read_aligned_string()?);
            bundle_version = Some(rdr.read_aligned_string()?);
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation =
                Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 5) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_force_exclusive_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 4) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background =
                Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 3) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 3) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgbAf::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api =
                Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgbAf::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgbAf::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 2) {
            xbox_skin_on_gpu = Some(PlayerSettingsXboxSkinOnGpuVariant::parse(rdr, ver)?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgbAf::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            xbox_skin_on_gpu = Some(PlayerSettingsXboxSkinOnGpuVariant::parse(rdr, ver)?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(
                PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?,
            );
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgbAf::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            xbox_skin_on_gpu = Some(PlayerSettingsXboxSkinOnGpuVariant::parse(rdr, ver)?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgbAf::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 5) {
            xbox_skin_on_gpu = Some(PlayerSettingsXboxSkinOnGpuVariant::parse(rdr, ver)?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_speech = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgbAf::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            debug_unload_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            xbox_skin_on_gpu = Some(PlayerSettingsXboxSkinOnGpuVariant::parse(rdr, ver)?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_speech = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgbAf::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
        } else {
            xbox_skin_on_gpu = Some(PlayerSettingsXboxSkinOnGpuVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
        }

        Ok(PlayerSettings {
            android_profiler: android_profiler.unwrap(),
            product_guid,
            default_screen_orientation: default_screen_orientation.unwrap(),
            target_device: target_device.unwrap(),
            android_filter_touches_when_obscured,
            target_platform,
            target_resolution,
            target_gles_graphics,
            use_on_demand_resources,
            android_enable_sustained_performance_mode,
            accelerometer_frequency,
            target_ios_graphics,
            company_name: company_name.unwrap(),
            product_name: product_name.unwrap(),
            android_license_public_key,
            default_cursor,
            default_screen_width: default_screen_width.unwrap(),
            cursor_hotspot,
            default_screen_height: default_screen_height.unwrap(),
            cloud_project_id,
            m_show_unity_splash_screen,
            default_screen_width_web: default_screen_width_web.unwrap(),
            m_virtual_reality_splash_screen,
            m_splash_screen_style,
            m_splash_screen_background_color,
            default_screen_height_web: default_screen_height_web.unwrap(),
            m_rendering_path,
            m_show_unity_splash_logo,
            display_resolution_dialog,
            m_splash_screen_overlay_opacity,
            allowed_autorotate_to_portrait: allowed_autorotate_to_portrait.unwrap(),
            m_active_color_space,
            m_splash_screen_animation,
            allowed_autorotate_to_portrait_upside_down: allowed_autorotate_to_portrait_upside_down
                .unwrap(),
            m_mt_rendering,
            m_mobile_rendering_path,
            m_splash_screen_logo_style,
            allowed_autorotate_to_landscape_right: allowed_autorotate_to_landscape_right.unwrap(),
            ios_show_activity_indicator_on_loading,
            m_splash_screen_draw_mode,
            allowed_autorotate_to_landscape_left: allowed_autorotate_to_landscape_left.unwrap(),
            android_show_activity_indicator_on_loading,
            m_use_dx11,
            m_splash_screen_background_animation_zoom,
            use_os_autorotation: use_os_autorotation.unwrap(),
            m_mobile_mt_rendering,
            m_splash_screen_logo_animation_zoom,
            use32_bit_display_buffer: use32_bit_display_buffer.unwrap(),
            m_stereoscopic3_d,
            m_splash_screen_background_landscape_aspect,
            default_is_full_screen,
            m_splash_screen_background_portrait_aspect,
            use_alpha_in_dashboard,
            m_splash_screen_background_landscape_uvs,
            run_in_background: run_in_background.unwrap(),
            ios_app_in_background_behavior,
            m_stack_trace_types,
            m_splash_screen_background_portrait_uvs,
            capture_single_screen,
            m_splash_screen_logos,
            override_i_pod_music,
            m_splash_screen_background_landscape,
            use_player_log: use_player_log.unwrap(),
            use24_bit_depth_buffer,
            ios_allow_http_download,
            tizen_show_activity_indicator_on_loading,
            m_splash_screen_background_portrait,
            use_mac_app_store_validation: use_mac_app_store_validation.unwrap(),
            xbox_skin_on_gpu,
            m_holographic_tracking_loss_screen,
            m_supported_aspect_ratios,
            m_unity_splash_logo,
            default_is_native_resolution,
            disable_depth_and_stencil_buffers,
            enable_hw_statistics,
            m_stereo_rendering_path,
            xbox_enable_avatar,
            strip_physics,
            prepare_ios_for_recording,
            xbox_enable_kinect,
            resizable_window,
            xbox_enable_kinect_auto_tracking,
            xbox_enable_speech,
            mobile_mt_rendering_baked,
            unsupported_msaa_fallback,
            m_sprite_batch_vertex_threshold,
            wii_hio2_usage,
            force_single_instance,
            submit_analytics,
            m_sprite_batch_max_vertex_count,
            wii_loading_screen_rect_placement,
            mute_other_audio_sources,
            mip_stripping,
            aid,
            wii_loading_screen_background,
            bake_collision_meshes,
            number_of_mips_stripped,
            player_min_open_gles_version,
            wii_loading_screen_period,
            xbox_enable_fitness,
            gpu_skinning,
            wii_loading_screen_file_name,
            mac_fullscreen_mode,
            ios_use_custom_app_background_behavior,
            wii_loading_screen_rect,
            xbox_speech_db,
            xbox_pix_texture_capture,
            debug_unload_mode,
            xbox_enable_head_orientation,
            number_of_mips_stripped_per_mipmap_limit_group,
            graphics_jobs,
            visible_in_background,
            preserve_framebuffer_alpha,
            d3d9_fullscreen_mode,
            xbox_enable_guest,
            android_blit_type,
            android_start_in_fullscreen,
            i_phone_bundle_identifier,
            d3d11_force_exclusive_mode,
            d3d11_fullscreen_mode,
            allow_fullscreen_switch,
            android_render_outside_safe_area,
            video_memory_for_vertex_buffers,
            android_use_swappy,
            mac_retina_support,
            android_resizable_window,
            metro_enable_independent_input_source,
            xbox_one_resolution,
            n3ds_disable_stereoscopic_view,
            defer_system_gestures_mode,
            force_ios_speakers_when_recording,
            android_default_window_width,
            metro_enable_low_latency_presentation_api,
            ps3_splash_screen,
            n3ds_enable_shared_list_opt,
            hide_home_button,
            android_default_window_height,
            adjust_iosfps_using_thermal_state,
            n3ds_enable_v_sync,
            android_minimum_window_width,
            thermal_state_serious_iosfps,
            psp2_power_mode,
            xbox_enable_pix_sampling,
            android_minimum_window_height,
            thermal_state_critical_iosfps,
            psp2_acquire_bgm,
            xbox_enable_enable_render_thread_runs_jobs,
            android_fullscreen_mode,
            android_display_options,
            bundle_identifier,
            mac_app_store_category,
            android_resizeable_activity,
            bundle_version,
            ui_use16_bit_depth_buffer,
            preloaded_assets,
            wii_utv_resolution,
            ignore_alpha_clear,
            android_requested_visible_insets,
            wii_u_game_pad_msaa,
            android_system_bars_behavior,
            wii_u_supports_nunchuk,
            android_auto_rotation_behavior,
            xbox_one_disable_kinect_gpu_reservation,
            wii_u_supports_classic_controller,
            xbox_one_mono_logging_level,
            android_predictive_back_support,
            android_application_entry,
            virtual_reality_supported,
            wii_u_supports_balance_board,
            xbox_one_logging_level,
            use_flip_model_swapchain,
            project_id,
            wii_u_supports_motion_plus,
            project_name,
            wii_u_supports_pro_controller,
            organization_id,
            wii_u_allow_screen_capture,
            graphics_job_mode,
            cloud_enabled,
            wii_u_controller_count,
            audio_spatial_experience,
            dedicated_server_optimizations,
            fullscreen_mode,
            call_on_disable_on_asset_bundle_unload,
            metal_framebuffer_only,
            xbox_one_s_resolution,
            xbox_one_x_resolution,
            mesh_deformation,
            xbox_one_disable_esram,
            single_pass_stereo_rendering,
            protect_graphics_memory,
            xbox_one_present_immediate_threshold,
            xbox_one_enable_type_optimization,
            switch_queue_command_memory,
            switch_queue_control_memory,
            vulkan_enable_set_srgb_write,
            switch_queue_compute_memory,
            switch_nvn_shader_pools_granularity,
            switch_nvn_default_pools_granularity,
            switch_nvn_other_pools_granularity,
            metro_input_source,
            vulkan_num_swapchain_buffers,
            wsa_transparent_swapchain,
            vulkan_use_sw_command_buffers,
            switch_nvn_max_public_texture_id_count,
            stadia_present_mode,
            m_holographic_pause_on_tracking_loss,
            switch_nvn_max_public_sampler_id_count,
            stadia_target_framerate,
            xbox_one_enable7th_core,
            is_wsa_holographic_remoting_enabled,
            vr_settings,
            metal_use_metal_display_link,
            vulkan_enable_late_acquire_next_image,
            use_hdr_display,
            enable_frame_timing_stats,
            m_color_gamuts,
            target_pixel_density,
            vulkan_enable_pre_transform,
            switch_gpu_scratch_pool_granularity,
            resolution_scaling_mode,
            switch_allow_gpu_scratch_shrinking,
            enable_new_input_system,
            android_supported_aspect_ratio,
            vulkan_enable_command_buffer_recycling,
            switch_max_worker_multiple,
            android_max_aspect_ratio,
            d3dhdr_bit_depth,
            switch_nvn_graphics_firmware_memory,
            framebuffer_depth_memoryless_mode,
            enable_native_platform_backends_for_new_input_system,
            disable_old_input_manager_support,
            legacy_clamp_blend_shape_weights,
            load_store_debug_mode_enabled,
            switch_graphics_jobs_sync_after_kick,
            invalidated_pattern_texture,
            active_input_handler,
            enable_open_gl_profiler_gpu_recorders,
            vision_os_bundle_version,
            tv_os_bundle_version,
            virtual_texturing_support_enabled,
            quality_settings_names,
            reset_resolution_on_window_resize,
            hdr_bit_depth,
            windows_gamepad_backend_hint,
            allow_hdr_display_support,
            player_data_path,
            force_srgb_blit,
            upload_cleared_texture_data_after_creation_from_script,
            android_min_aspect_ratio,
            insecure_http_option,
            hmi_loading_image,
            platform_requires_readable_assets,
            enable_direct_storage,
            android_vulkan_deny_filter_list,
            android_vulkan_allow_filter_list,
            android_vulkan_device_filter_list_asset,
            d3d12_device_filter_list_asset,
            allowed_http_connections,
        })
    }
}

impl crate::ToValue for PlayerSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "AndroidProfiler".to_string(),
                self.android_profiler.to_value(),
            )),
            crate::opt_field("productGUID", &self.product_guid),
            Some((
                "defaultScreenOrientation".to_string(),
                self.default_screen_orientation.to_value(),
            )),
            Some(("targetDevice".to_string(), self.target_device.to_value())),
            crate::opt_field(
                "AndroidFilterTouchesWhenObscured",
                &self.android_filter_touches_when_obscured,
            ),
            crate::opt_field("targetPlatform", &self.target_platform),
            crate::opt_field("targetResolution", &self.target_resolution),
            crate::opt_field("targetGlesGraphics", &self.target_gles_graphics),
            crate::opt_field("useOnDemandResources", &self.use_on_demand_resources),
            crate::opt_field(
                "AndroidEnableSustainedPerformanceMode",
                &self.android_enable_sustained_performance_mode,
            ),
            crate::opt_field("accelerometerFrequency", &self.accelerometer_frequency),
            crate::opt_field("targetIOSGraphics", &self.target_ios_graphics),
            Some(("companyName".to_string(), self.company_name.to_value())),
            Some(("productName".to_string(), self.product_name.to_value())),
            crate::opt_field("AndroidLicensePublicKey", &self.android_license_public_key),
            crate::opt_field("defaultCursor", &self.default_cursor),
            Some((
                "defaultScreenWidth".to_string(),
                self.default_screen_width.to_value(),
            )),
            crate::opt_field("cursorHotspot", &self.cursor_hotspot),
            Some((
                "defaultScreenHeight".to_string(),
                self.default_screen_height.to_value(),
            )),
            crate::opt_field("cloudProjectId", &self.cloud_project_id),
            crate::opt_field("m_ShowUnitySplashScreen", &self.m_show_unity_splash_screen),
            Some((
                "defaultScreenWidthWeb".to_string(),
                self.default_screen_width_web.to_value(),
            )),
            crate::opt_field(
                "m_VirtualRealitySplashScreen",
                &self.m_virtual_reality_splash_screen,
            ),
            crate::opt_field("m_SplashScreenStyle", &self.m_splash_screen_style),
            crate::opt_field(
                "m_SplashScreenBackgroundColor",
                &self.m_splash_screen_background_color,
            ),
            Some((
                "defaultScreenHeightWeb".to_string(),
                self.default_screen_height_web.to_value(),
            )),
            crate::opt_field("m_RenderingPath", &self.m_rendering_path),
            crate::opt_field("m_ShowUnitySplashLogo", &self.m_show_unity_splash_logo),
            crate::opt_field("displayResolutionDialog", &self.display_resolution_dialog),
            crate::opt_field(
                "m_SplashScreenOverlayOpacity",
                &self.m_splash_screen_overlay_opacity,
            ),
            Some((
                "allowedAutorotateToPortrait".to_string(),
                self.allowed_autorotate_to_portrait.to_value(),
            )),
            crate::opt_field("m_ActiveColorSpace", &self.m_active_color_space),
            crate::opt_field("m_SplashScreenAnimation", &self.m_splash_screen_animation),
            Some((
                "allowedAutorotateToPortraitUpsideDown".to_string(),
                self.allowed_autorotate_to_portrait_upside_down.to_value(),
            )),
            crate::opt_field("m_MTRendering", &self.m_mt_rendering),
            crate::opt_field("m_MobileRenderingPath", &self.m_mobile_rendering_path),
            crate::opt_field("m_SplashScreenLogoStyle", &self.m_splash_screen_logo_style),
            Some((
                "allowedAutorotateToLandscapeRight".to_string(),
                self.allowed_autorotate_to_landscape_right.to_value(),
            )),
            crate::opt_field(
                "iosShowActivityIndicatorOnLoading",
                &self.ios_show_activity_indicator_on_loading,
            ),
            crate::opt_field("m_SplashScreenDrawMode", &self.m_splash_screen_draw_mode),
            Some((
                "allowedAutorotateToLandscapeLeft".to_string(),
                self.allowed_autorotate_to_landscape_left.to_value(),
            )),
            crate::opt_field(
                "androidShowActivityIndicatorOnLoading",
                &self.android_show_activity_indicator_on_loading,
            ),
            crate::opt_field("m_UseDX11", &self.m_use_dx11),
            crate::opt_field(
                "m_SplashScreenBackgroundAnimationZoom",
                &self.m_splash_screen_background_animation_zoom,
            ),
            Some((
                "useOSAutorotation".to_string(),
                self.use_os_autorotation.to_value(),
            )),
            crate::opt_field("m_MobileMTRendering", &self.m_mobile_mt_rendering),
            crate::opt_field(
                "m_SplashScreenLogoAnimationZoom",
                &self.m_splash_screen_logo_animation_zoom,
            ),
            Some((
                "use32BitDisplayBuffer".to_string(),
                self.use32_bit_display_buffer.to_value(),
            )),
            crate::opt_field("m_Stereoscopic3D", &self.m_stereoscopic3_d),
            crate::opt_field(
                "m_SplashScreenBackgroundLandscapeAspect",
                &self.m_splash_screen_background_landscape_aspect,
            ),
            crate::opt_field("defaultIsFullScreen", &self.default_is_full_screen),
            crate::opt_field(
                "m_SplashScreenBackgroundPortraitAspect",
                &self.m_splash_screen_background_portrait_aspect,
            ),
            crate::opt_field("useAlphaInDashboard", &self.use_alpha_in_dashboard),
            crate::opt_field(
                "m_SplashScreenBackgroundLandscapeUvs",
                &self.m_splash_screen_background_landscape_uvs,
            ),
            Some((
                "runInBackground".to_string(),
                self.run_in_background.to_value(),
            )),
            crate::opt_field(
                "iosAppInBackgroundBehavior",
                &self.ios_app_in_background_behavior,
            ),
            crate::opt_field("m_StackTraceTypes", &self.m_stack_trace_types),
            crate::opt_field(
                "m_SplashScreenBackgroundPortraitUvs",
                &self.m_splash_screen_background_portrait_uvs,
            ),
            crate::opt_field("captureSingleScreen", &self.capture_single_screen),
            crate::opt_field("m_SplashScreenLogos", &self.m_splash_screen_logos),
            crate::opt_field("Override IPod Music", &self.override_i_pod_music),
            crate::opt_field(
                "m_SplashScreenBackgroundLandscape",
                &self.m_splash_screen_background_landscape,
            ),
            Some(("usePlayerLog".to_string(), self.use_player_log.to_value())),
            crate::opt_field("use24BitDepthBuffer", &self.use24_bit_depth_buffer),
            crate::opt_field("iosAllowHTTPDownload", &self.ios_allow_http_download),
            crate::opt_field(
                "tizenShowActivityIndicatorOnLoading",
                &self.tizen_show_activity_indicator_on_loading,
            ),
            crate::opt_field(
                "m_SplashScreenBackgroundPortrait",
                &self.m_splash_screen_background_portrait,
            ),
            Some((
                "useMacAppStoreValidation".to_string(),
                self.use_mac_app_store_validation.to_value(),
            )),
            crate::opt_field("xboxSkinOnGPU", &self.xbox_skin_on_gpu),
            crate::opt_field(
                "m_HolographicTrackingLossScreen",
                &self.m_holographic_tracking_loss_screen,
            ),
            crate::opt_field("m_SupportedAspectRatios", &self.m_supported_aspect_ratios),
            crate::opt_field("m_UnitySplashLogo", &self.m_unity_splash_logo),
            crate::opt_field(
                "defaultIsNativeResolution",
                &self.default_is_native_resolution,
            ),
            crate::opt_field(
                "disableDepthAndStencilBuffers",
                &self.disable_depth_and_stencil_buffers,
            ),
            crate::opt_field("enableHWStatistics", &self.enable_hw_statistics),
            crate::opt_field("m_StereoRenderingPath", &self.m_stereo_rendering_path),
            crate::opt_field("xboxEnableAvatar", &self.xbox_enable_avatar),
            crate::opt_field("stripPhysics", &self.strip_physics),
            crate::opt_field("Prepare IOS For Recording", &self.prepare_ios_for_recording),
            crate::opt_field("xboxEnableKinect", &self.xbox_enable_kinect),
            crate::opt_field("resizableWindow", &self.resizable_window),
            crate::opt_field(
                "xboxEnableKinectAutoTracking",
                &self.xbox_enable_kinect_auto_tracking,
            ),
            crate::opt_field("xboxEnableSpeech", &self.xbox_enable_speech),
            crate::opt_field("mobileMTRenderingBaked", &self.mobile_mt_rendering_baked),
            crate::opt_field("unsupportedMSAAFallback", &self.unsupported_msaa_fallback),
            crate::opt_field(
                "m_SpriteBatchVertexThreshold",
                &self.m_sprite_batch_vertex_threshold,
            ),
            crate::opt_field("wiiHio2Usage", &self.wii_hio2_usage),
            crate::opt_field("forceSingleInstance", &self.force_single_instance),
            crate::opt_field("submitAnalytics", &self.submit_analytics),
            crate::opt_field(
                "m_SpriteBatchMaxVertexCount",
                &self.m_sprite_batch_max_vertex_count,
            ),
            crate::opt_field(
                "wiiLoadingScreenRectPlacement",
                &self.wii_loading_screen_rect_placement,
            ),
            crate::opt_field("muteOtherAudioSources", &self.mute_other_audio_sources),
            crate::opt_field("mipStripping", &self.mip_stripping),
            crate::opt_field("AID", &self.aid),
            crate::opt_field(
                "wiiLoadingScreenBackground",
                &self.wii_loading_screen_background,
            ),
            crate::opt_field("bakeCollisionMeshes", &self.bake_collision_meshes),
            crate::opt_field("numberOfMipsStripped", &self.number_of_mips_stripped),
            crate::opt_field(
                "playerMinOpenGLESVersion",
                &self.player_min_open_gles_version,
            ),
            crate::opt_field("wiiLoadingScreenPeriod", &self.wii_loading_screen_period),
            crate::opt_field("xboxEnableFitness", &self.xbox_enable_fitness),
            crate::opt_field("gpuSkinning", &self.gpu_skinning),
            crate::opt_field(
                "wiiLoadingScreenFileName",
                &self.wii_loading_screen_file_name,
            ),
            crate::opt_field("macFullscreenMode", &self.mac_fullscreen_mode),
            crate::opt_field(
                "iosUseCustomAppBackgroundBehavior",
                &self.ios_use_custom_app_background_behavior,
            ),
            crate::opt_field("wiiLoadingScreenRect", &self.wii_loading_screen_rect),
            crate::opt_field("xboxSpeechDB", &self.xbox_speech_db),
            crate::opt_field("xboxPIXTextureCapture", &self.xbox_pix_texture_capture),
            crate::opt_field("debugUnloadMode", &self.debug_unload_mode),
            crate::opt_field(
                "xboxEnableHeadOrientation",
                &self.xbox_enable_head_orientation,
            ),
            crate::opt_field(
                "numberOfMipsStrippedPerMipmapLimitGroup",
                &self.number_of_mips_stripped_per_mipmap_limit_group,
            ),
            crate::opt_field("graphicsJobs", &self.graphics_jobs),
            crate::opt_field("visibleInBackground", &self.visible_in_background),
            crate::opt_field("preserveFramebufferAlpha", &self.preserve_framebuffer_alpha),
            crate::opt_field("d3d9FullscreenMode", &self.d3d9_fullscreen_mode),
            crate::opt_field("xboxEnableGuest", &self.xbox_enable_guest),
            crate::opt_field("androidBlitType", &self.android_blit_type),
            crate::opt_field(
                "androidStartInFullscreen",
                &self.android_start_in_fullscreen,
            ),
            crate::opt_field("iPhoneBundleIdentifier", &self.i_phone_bundle_identifier),
            crate::opt_field("d3d11ForceExclusiveMode", &self.d3d11_force_exclusive_mode),
            crate::opt_field("d3d11FullscreenMode", &self.d3d11_fullscreen_mode),
            crate::opt_field("allowFullscreenSwitch", &self.allow_fullscreen_switch),
            crate::opt_field(
                "androidRenderOutsideSafeArea",
                &self.android_render_outside_safe_area,
            ),
            crate::opt_field(
                "videoMemoryForVertexBuffers",
                &self.video_memory_for_vertex_buffers,
            ),
            crate::opt_field("androidUseSwappy", &self.android_use_swappy),
            crate::opt_field("macRetinaSupport", &self.mac_retina_support),
            crate::opt_field("androidResizableWindow", &self.android_resizable_window),
            crate::opt_field(
                "metroEnableIndependentInputSource",
                &self.metro_enable_independent_input_source,
            ),
            crate::opt_field("xboxOneResolution", &self.xbox_one_resolution),
            crate::opt_field(
                "n3dsDisableStereoscopicView",
                &self.n3ds_disable_stereoscopic_view,
            ),
            crate::opt_field("deferSystemGesturesMode", &self.defer_system_gestures_mode),
            crate::opt_field(
                "Force IOS Speakers When Recording",
                &self.force_ios_speakers_when_recording,
            ),
            crate::opt_field(
                "androidDefaultWindowWidth",
                &self.android_default_window_width,
            ),
            crate::opt_field(
                "metroEnableLowLatencyPresentationAPI",
                &self.metro_enable_low_latency_presentation_api,
            ),
            crate::opt_field("ps3SplashScreen", &self.ps3_splash_screen),
            crate::opt_field("n3dsEnableSharedListOpt", &self.n3ds_enable_shared_list_opt),
            crate::opt_field("hideHomeButton", &self.hide_home_button),
            crate::opt_field(
                "androidDefaultWindowHeight",
                &self.android_default_window_height,
            ),
            crate::opt_field(
                "adjustIOSFPSUsingThermalState",
                &self.adjust_iosfps_using_thermal_state,
            ),
            crate::opt_field("n3dsEnableVSync", &self.n3ds_enable_v_sync),
            crate::opt_field(
                "androidMinimumWindowWidth",
                &self.android_minimum_window_width,
            ),
            crate::opt_field(
                "thermalStateSeriousIOSFPS",
                &self.thermal_state_serious_iosfps,
            ),
            crate::opt_field("psp2PowerMode", &self.psp2_power_mode),
            crate::opt_field("xboxEnablePIXSampling", &self.xbox_enable_pix_sampling),
            crate::opt_field(
                "androidMinimumWindowHeight",
                &self.android_minimum_window_height,
            ),
            crate::opt_field(
                "thermalStateCriticalIOSFPS",
                &self.thermal_state_critical_iosfps,
            ),
            crate::opt_field("psp2AcquireBGM", &self.psp2_acquire_bgm),
            crate::opt_field(
                "xboxEnableEnableRenderThreadRunsJobs",
                &self.xbox_enable_enable_render_thread_runs_jobs,
            ),
            crate::opt_field("androidFullscreenMode", &self.android_fullscreen_mode),
            crate::opt_field("androidDisplayOptions", &self.android_display_options),
            crate::opt_field("bundleIdentifier", &self.bundle_identifier),
            crate::opt_field("macAppStoreCategory", &self.mac_app_store_category),
            crate::opt_field(
                "androidResizeableActivity",
                &self.android_resizeable_activity,
            ),
            crate::opt_field("bundleVersion", &self.bundle_version),
            crate::opt_field("uiUse16BitDepthBuffer", &self.ui_use16_bit_depth_buffer),
            crate::opt_field("preloadedAssets", &self.preloaded_assets),
            crate::opt_field("wiiUTVResolution", &self.wii_utv_resolution),
            crate::opt_field("ignoreAlphaClear", &self.ignore_alpha_clear),
            crate::opt_field(
                "androidRequestedVisibleInsets",
                &self.android_requested_visible_insets,
            ),
            crate::opt_field("wiiUGamePadMSAA", &self.wii_u_game_pad_msaa),
            crate::opt_field(
                "androidSystemBarsBehavior",
                &self.android_system_bars_behavior,
            ),
            crate::opt_field("wiiUSupportsNunchuk", &self.wii_u_supports_nunchuk),
            crate::opt_field(
                "androidAutoRotationBehavior",
                &self.android_auto_rotation_behavior,
            ),
            crate::opt_field(
                "xboxOneDisableKinectGpuReservation",
                &self.xbox_one_disable_kinect_gpu_reservation,
            ),
            crate::opt_field(
                "wiiUSupportsClassicController",
                &self.wii_u_supports_classic_controller,
            ),
            crate::opt_field("xboxOneMonoLoggingLevel", &self.xbox_one_mono_logging_level),
            crate::opt_field(
                "androidPredictiveBackSupport",
                &self.android_predictive_back_support,
            ),
            crate::opt_field("androidApplicationEntry", &self.android_application_entry),
            crate::opt_field("virtualRealitySupported", &self.virtual_reality_supported),
            crate::opt_field(
                "wiiUSupportsBalanceBoard",
                &self.wii_u_supports_balance_board,
            ),
            crate::opt_field("xboxOneLoggingLevel", &self.xbox_one_logging_level),
            crate::opt_field("useFlipModelSwapchain", &self.use_flip_model_swapchain),
            crate::opt_field("projectId", &self.project_id),
            crate::opt_field("wiiUSupportsMotionPlus", &self.wii_u_supports_motion_plus),
            crate::opt_field("projectName", &self.project_name),
            crate::opt_field(
                "wiiUSupportsProController",
                &self.wii_u_supports_pro_controller,
            ),
            crate::opt_field("organizationId", &self.organization_id),
            crate::opt_field("wiiUAllowScreenCapture", &self.wii_u_allow_screen_capture),
            crate::opt_field("graphicsJobMode", &self.graphics_job_mode),
            crate::opt_field("cloudEnabled", &self.cloud_enabled),
            crate::opt_field("wiiUControllerCount", &self.wii_u_controller_count),
            crate::opt_field("audioSpatialExperience", &self.audio_spatial_experience),
            crate::opt_field(
                "dedicatedServerOptimizations",
                &self.dedicated_server_optimizations,
            ),
            crate::opt_field("fullscreenMode", &self.fullscreen_mode),
            crate::opt_field(
                "callOnDisableOnAssetBundleUnload",
                &self.call_on_disable_on_asset_bundle_unload,
            ),
            crate::opt_field("metalFramebufferOnly", &self.metal_framebuffer_only),
            crate::opt_field("xboxOneSResolution", &self.xbox_one_s_resolution),
            crate::opt_field("xboxOneXResolution", &self.xbox_one_x_resolution),
            crate::opt_field("meshDeformation", &self.mesh_deformation),
            crate::opt_field("xboxOneDisableEsram", &self.xbox_one_disable_esram),
            crate::opt_field(
                "singlePassStereoRendering",
                &self.single_pass_stereo_rendering,
            ),
            crate::opt_field("protectGraphicsMemory", &self.protect_graphics_memory),
            crate::opt_field(
                "xboxOnePresentImmediateThreshold",
                &self.xbox_one_present_immediate_threshold,
            ),
            crate::opt_field(
                "xboxOneEnableTypeOptimization",
                &self.xbox_one_enable_type_optimization,
            ),
            crate::opt_field(
                "switchQueueCommandMemory",
                &self.switch_queue_command_memory,
            ),
            crate::opt_field(
                "switchQueueControlMemory",
                &self.switch_queue_control_memory,
            ),
            crate::opt_field(
                "vulkanEnableSetSRGBWrite",
                &self.vulkan_enable_set_srgb_write,
            ),
            crate::opt_field(
                "switchQueueComputeMemory",
                &self.switch_queue_compute_memory,
            ),
            crate::opt_field(
                "switchNVNShaderPoolsGranularity",
                &self.switch_nvn_shader_pools_granularity,
            ),
            crate::opt_field(
                "switchNVNDefaultPoolsGranularity",
                &self.switch_nvn_default_pools_granularity,
            ),
            crate::opt_field(
                "switchNVNOtherPoolsGranularity",
                &self.switch_nvn_other_pools_granularity,
            ),
            crate::opt_field("metroInputSource", &self.metro_input_source),
            crate::opt_field(
                "vulkanNumSwapchainBuffers",
                &self.vulkan_num_swapchain_buffers,
            ),
            crate::opt_field("wsaTransparentSwapchain", &self.wsa_transparent_swapchain),
            crate::opt_field(
                "vulkanUseSWCommandBuffers",
                &self.vulkan_use_sw_command_buffers,
            ),
            crate::opt_field(
                "switchNVNMaxPublicTextureIDCount",
                &self.switch_nvn_max_public_texture_id_count,
            ),
            crate::opt_field("stadiaPresentMode", &self.stadia_present_mode),
            crate::opt_field(
                "m_HolographicPauseOnTrackingLoss",
                &self.m_holographic_pause_on_tracking_loss,
            ),
            crate::opt_field(
                "switchNVNMaxPublicSamplerIDCount",
                &self.switch_nvn_max_public_sampler_id_count,
            ),
            crate::opt_field("stadiaTargetFramerate", &self.stadia_target_framerate),
            crate::opt_field("xboxOneEnable7thCore", &self.xbox_one_enable7th_core),
            crate::opt_field(
                "isWsaHolographicRemotingEnabled",
                &self.is_wsa_holographic_remoting_enabled,
            ),
            crate::opt_field("vrSettings", &self.vr_settings),
            crate::opt_field(
                "metalUseMetalDisplayLink",
                &self.metal_use_metal_display_link,
            ),
            crate::opt_field(
                "vulkanEnableLateAcquireNextImage",
                &self.vulkan_enable_late_acquire_next_image,
            ),
            crate::opt_field("useHDRDisplay", &self.use_hdr_display),
            crate::opt_field("enableFrameTimingStats", &self.enable_frame_timing_stats),
            crate::opt_field("m_ColorGamuts", &self.m_color_gamuts),
            crate::opt_field("targetPixelDensity", &self.target_pixel_density),
            crate::opt_field(
                "vulkanEnablePreTransform",
                &self.vulkan_enable_pre_transform,
            ),
            crate::opt_field(
                "switchGpuScratchPoolGranularity",
                &self.switch_gpu_scratch_pool_granularity,
            ),
            crate::opt_field("resolutionScalingMode", &self.resolution_scaling_mode),
            crate::opt_field(
                "switchAllowGpuScratchShrinking",
                &self.switch_allow_gpu_scratch_shrinking,
            ),
            crate::opt_field("enableNewInputSystem", &self.enable_new_input_system),
            crate::opt_field(
                "androidSupportedAspectRatio",
                &self.android_supported_aspect_ratio,
            ),
            crate::opt_field(
                "vulkanEnableCommandBufferRecycling",
                &self.vulkan_enable_command_buffer_recycling,
            ),
            crate::opt_field("switchMaxWorkerMultiple", &self.switch_max_worker_multiple),
            crate::opt_field("androidMaxAspectRatio", &self.android_max_aspect_ratio),
            crate::opt_field("D3DHDRBitDepth", &self.d3dhdr_bit_depth),
            crate::opt_field(
                "switchNVNGraphicsFirmwareMemory",
                &self.switch_nvn_graphics_firmware_memory,
            ),
            crate::opt_field(
                "framebufferDepthMemorylessMode",
                &self.framebuffer_depth_memoryless_mode,
            ),
            crate::opt_field(
                "enableNativePlatformBackendsForNewInputSystem",
                &self.enable_native_platform_backends_for_new_input_system,
            ),
            crate::opt_field(
                "disableOldInputManagerSupport",
                &self.disable_old_input_manager_support,
            ),
            crate::opt_field(
                "legacyClampBlendShapeWeights",
                &self.legacy_clamp_blend_shape_weights,
            ),
            crate::opt_field(
                "loadStoreDebugModeEnabled",
                &self.load_store_debug_mode_enabled,
            ),
            crate::opt_field(
                "switchGraphicsJobsSyncAfterKick",
                &self.switch_graphics_jobs_sync_after_kick,
            ),
            crate::opt_field(
                "invalidatedPatternTexture",
                &self.invalidated_pattern_texture,
            ),
            crate::opt_field("activeInputHandler", &self.active_input_handler),
            crate::opt_field(
                "enableOpenGLProfilerGPURecorders",
                &self.enable_open_gl_profiler_gpu_recorders,
            ),
            crate::opt_field("visionOSBundleVersion", &self.vision_os_bundle_version),
            crate::opt_field("tvOSBundleVersion", &self.tv_os_bundle_version),
            crate::opt_field(
                "virtualTexturingSupportEnabled",
                &self.virtual_texturing_support_enabled,
            ),
            crate::opt_field("qualitySettingsNames", &self.quality_settings_names),
            crate::opt_field(
                "resetResolutionOnWindowResize",
                &self.reset_resolution_on_window_resize,
            ),
            crate::opt_field("hdrBitDepth", &self.hdr_bit_depth),
            crate::opt_field(
                "windowsGamepadBackendHint",
                &self.windows_gamepad_backend_hint,
            ),
            crate::opt_field("allowHDRDisplaySupport", &self.allow_hdr_display_support),
            crate::opt_field("playerDataPath", &self.player_data_path),
            crate::opt_field("forceSRGBBlit", &self.force_srgb_blit),
            crate::opt_field(
                "uploadClearedTextureDataAfterCreationFromScript",
                &self.upload_cleared_texture_data_after_creation_from_script,
            ),
            crate::opt_field("androidMinAspectRatio", &self.android_min_aspect_ratio),
            crate::opt_field("insecureHttpOption", &self.insecure_http_option),
            crate::opt_field("hmiLoadingImage", &self.hmi_loading_image),
            crate::opt_field(
                "platformRequiresReadableAssets",
                &self.platform_requires_readable_assets,
            ),
            crate::opt_field("enableDirectStorage", &self.enable_direct_storage),
            crate::opt_field(
                "androidVulkanDenyFilterList",
                &self.android_vulkan_deny_filter_list,
            ),
            crate::opt_field(
                "androidVulkanAllowFilterList",
                &self.android_vulkan_allow_filter_list,
            ),
            crate::opt_field(
                "androidVulkanDeviceFilterListAsset",
                &self.android_vulkan_device_filter_list_asset,
            ),
            crate::opt_field(
                "d3d12DeviceFilterListAsset",
                &self.d3d12_device_filter_list_asset,
            ),
            crate::opt_field("allowedHttpConnections", &self.allowed_http_connections),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "AndroidProfiler" => (self.android_profiler).inspect_value(path, offset, limit),
                "productGUID" => (self.product_guid).inspect_value(path, offset, limit),
                "defaultScreenOrientation" => {
                    (self.default_screen_orientation).inspect_value(path, offset, limit)
                }
                "targetDevice" => (self.target_device).inspect_value(path, offset, limit),
                "AndroidFilterTouchesWhenObscured" => {
                    (self.android_filter_touches_when_obscured).inspect_value(path, offset, limit)
                }
                "targetPlatform" => (self.target_platform).inspect_value(path, offset, limit),
                "targetResolution" => (self.target_resolution).inspect_value(path, offset, limit),
                "targetGlesGraphics" => {
                    (self.target_gles_graphics).inspect_value(path, offset, limit)
                }
                "useOnDemandResources" => {
                    (self.use_on_demand_resources).inspect_value(path, offset, limit)
                }
                "AndroidEnableSustainedPerformanceMode" => (self
                    .android_enable_sustained_performance_mode)
                    .inspect_value(path, offset, limit),
                "accelerometerFrequency" => {
                    (self.accelerometer_frequency).inspect_value(path, offset, limit)
                }
                "targetIOSGraphics" => {
                    (self.target_ios_graphics).inspect_value(path, offset, limit)
                }
                "companyName" => (self.company_name).inspect_value(path, offset, limit),
                "productName" => (self.product_name).inspect_value(path, offset, limit),
                "AndroidLicensePublicKey" => {
                    (self.android_license_public_key).inspect_value(path, offset, limit)
                }
                "defaultCursor" => (self.default_cursor).inspect_value(path, offset, limit),
                "defaultScreenWidth" => {
                    (self.default_screen_width).inspect_value(path, offset, limit)
                }
                "cursorHotspot" => (self.cursor_hotspot).inspect_value(path, offset, limit),
                "defaultScreenHeight" => {
                    (self.default_screen_height).inspect_value(path, offset, limit)
                }
                "cloudProjectId" => (self.cloud_project_id).inspect_value(path, offset, limit),
                "m_ShowUnitySplashScreen" => {
                    (self.m_show_unity_splash_screen).inspect_value(path, offset, limit)
                }
                "defaultScreenWidthWeb" => {
                    (self.default_screen_width_web).inspect_value(path, offset, limit)
                }
                "m_VirtualRealitySplashScreen" => {
                    (self.m_virtual_reality_splash_screen).inspect_value(path, offset, limit)
                }
                "m_SplashScreenStyle" => {
                    (self.m_splash_screen_style).inspect_value(path, offset, limit)
                }
                "m_SplashScreenBackgroundColor" => {
                    (self.m_splash_screen_background_color).inspect_value(path, offset, limit)
                }
                "defaultScreenHeightWeb" => {
                    (self.default_screen_height_web).inspect_value(path, offset, limit)
                }
                "m_RenderingPath" => (self.m_rendering_path).inspect_value(path, offset, limit),
                "m_ShowUnitySplashLogo" => {
                    (self.m_show_unity_splash_logo).inspect_value(path, offset, limit)
                }
                "displayResolutionDialog" => {
                    (self.display_resolution_dialog).inspect_value(path, offset, limit)
                }
                "m_SplashScreenOverlayOpacity" => {
                    (self.m_splash_screen_overlay_opacity).inspect_value(path, offset, limit)
                }
                "allowedAutorotateToPortrait" => {
                    (self.allowed_autorotate_to_portrait).inspect_value(path, offset, limit)
                }
                "m_ActiveColorSpace" => {
                    (self.m_active_color_space).inspect_value(path, offset, limit)
                }
                "m_SplashScreenAnimation" => {
                    (self.m_splash_screen_animation).inspect_value(path, offset, limit)
                }
                "allowedAutorotateToPortraitUpsideDown" => (self
                    .allowed_autorotate_to_portrait_upside_down)
                    .inspect_value(path, offset, limit),
                "m_MTRendering" => (self.m_mt_rendering).inspect_value(path, offset, limit),
                "m_MobileRenderingPath" => {
                    (self.m_mobile_rendering_path).inspect_value(path, offset, limit)
                }
                "m_SplashScreenLogoStyle" => {
                    (self.m_splash_screen_logo_style).inspect_value(path, offset, limit)
                }
                "allowedAutorotateToLandscapeRight" => {
                    (self.allowed_autorotate_to_landscape_right).inspect_value(path, offset, limit)
                }
                "iosShowActivityIndicatorOnLoading" => {
                    (self.ios_show_activity_indicator_on_loading).inspect_value(path, offset, limit)
                }
                "m_SplashScreenDrawMode" => {
                    (self.m_splash_screen_draw_mode).inspect_value(path, offset, limit)
                }
                "allowedAutorotateToLandscapeLeft" => {
                    (self.allowed_autorotate_to_landscape_left).inspect_value(path, offset, limit)
                }
                "androidShowActivityIndicatorOnLoading" => (self
                    .android_show_activity_indicator_on_loading)
                    .inspect_value(path, offset, limit),
                "m_UseDX11" => (self.m_use_dx11).inspect_value(path, offset, limit),
                "m_SplashScreenBackgroundAnimationZoom" => (self
                    .m_splash_screen_background_animation_zoom)
                    .inspect_value(path, offset, limit),
                "useOSAutorotation" => {
                    (self.use_os_autorotation).inspect_value(path, offset, limit)
                }
                "m_MobileMTRendering" => {
                    (self.m_mobile_mt_rendering).inspect_value(path, offset, limit)
                }
                "m_SplashScreenLogoAnimationZoom" => {
                    (self.m_splash_screen_logo_animation_zoom).inspect_value(path, offset, limit)
                }
                "use32BitDisplayBuffer" => {
                    (self.use32_bit_display_buffer).inspect_value(path, offset, limit)
                }
                "m_Stereoscopic3D" => (self.m_stereoscopic3_d).inspect_value(path, offset, limit),
                "m_SplashScreenBackgroundLandscapeAspect" => (self
                    .m_splash_screen_background_landscape_aspect)
                    .inspect_value(path, offset, limit),
                "defaultIsFullScreen" => {
                    (self.default_is_full_screen).inspect_value(path, offset, limit)
                }
                "m_SplashScreenBackgroundPortraitAspect" => (self
                    .m_splash_screen_background_portrait_aspect)
                    .inspect_value(path, offset, limit),
                "useAlphaInDashboard" => {
                    (self.use_alpha_in_dashboard).inspect_value(path, offset, limit)
                }
                "m_SplashScreenBackgroundLandscapeUvs" => (self
                    .m_splash_screen_background_landscape_uvs)
                    .inspect_value(path, offset, limit),
                "runInBackground" => (self.run_in_background).inspect_value(path, offset, limit),
                "iosAppInBackgroundBehavior" => {
                    (self.ios_app_in_background_behavior).inspect_value(path, offset, limit)
                }
                "m_StackTraceTypes" => {
                    (self.m_stack_trace_types).inspect_value(path, offset, limit)
                }
                "m_SplashScreenBackgroundPortraitUvs" => (self
                    .m_splash_screen_background_portrait_uvs)
                    .inspect_value(path, offset, limit),
                "captureSingleScreen" => {
                    (self.capture_single_screen).inspect_value(path, offset, limit)
                }
                "m_SplashScreenLogos" => {
                    (self.m_splash_screen_logos).inspect_value(path, offset, limit)
                }
                "Override IPod Music" => {
                    (self.override_i_pod_music).inspect_value(path, offset, limit)
                }
                "m_SplashScreenBackgroundLandscape" => {
                    (self.m_splash_screen_background_landscape).inspect_value(path, offset, limit)
                }
                "usePlayerLog" => (self.use_player_log).inspect_value(path, offset, limit),
                "use24BitDepthBuffer" => {
                    (self.use24_bit_depth_buffer).inspect_value(path, offset, limit)
                }
                "iosAllowHTTPDownload" => {
                    (self.ios_allow_http_download).inspect_value(path, offset, limit)
                }
                "tizenShowActivityIndicatorOnLoading" => (self
                    .tizen_show_activity_indicator_on_loading)
                    .inspect_value(path, offset, limit),
                "m_SplashScreenBackgroundPortrait" => {
                    (self.m_splash_screen_background_portrait).inspect_value(path, offset, limit)
                }
                "useMacAppStoreValidation" => {
                    (self.use_mac_app_store_validation).inspect_value(path, offset, limit)
                }
                "xboxSkinOnGPU" => (self.xbox_skin_on_gpu).inspect_value(path, offset, limit),
                "m_HolographicTrackingLossScreen" => {
                    (self.m_holographic_tracking_loss_screen).inspect_value(path, offset, limit)
                }
                "m_SupportedAspectRatios" => {
                    (self.m_supported_aspect_ratios).inspect_value(path, offset, limit)
                }
                "m_UnitySplashLogo" => {
                    (self.m_unity_splash_logo).inspect_value(path, offset, limit)
                }
                "defaultIsNativeResolution" => {
                    (self.default_is_native_resolution).inspect_value(path, offset, limit)
                }
                "disableDepthAndStencilBuffers" => {
                    (self.disable_depth_and_stencil_buffers).inspect_value(path, offset, limit)
                }
                "enableHWStatistics" => {
                    (self.enable_hw_statistics).inspect_value(path, offset, limit)
                }
                "m_StereoRenderingPath" => {
                    (self.m_stereo_rendering_path).inspect_value(path, offset, limit)
                }
                "xboxEnableAvatar" => (self.xbox_enable_avatar).inspect_value(path, offset, limit),
                "stripPhysics" => (self.strip_physics).inspect_value(path, offset, limit),
                "Prepare IOS For Recording" => {
                    (self.prepare_ios_for_recording).inspect_value(path, offset, limit)
                }
                "xboxEnableKinect" => (self.xbox_enable_kinect).inspect_value(path, offset, limit),
                "resizableWindow" => (self.resizable_window).inspect_value(path, offset, limit),
                "xboxEnableKinectAutoTracking" => {
                    (self.xbox_enable_kinect_auto_tracking).inspect_value(path, offset, limit)
                }
                "xboxEnableSpeech" => (self.xbox_enable_speech).inspect_value(path, offset, limit),
                "mobileMTRenderingBaked" => {
                    (self.mobile_mt_rendering_baked).inspect_value(path, offset, limit)
                }
                "unsupportedMSAAFallback" => {
                    (self.unsupported_msaa_fallback).inspect_value(path, offset, limit)
                }
                "m_SpriteBatchVertexThreshold" => {
                    (self.m_sprite_batch_vertex_threshold).inspect_value(path, offset, limit)
                }
                "wiiHio2Usage" => (self.wii_hio2_usage).inspect_value(path, offset, limit),
                "forceSingleInstance" => {
                    (self.force_single_instance).inspect_value(path, offset, limit)
                }
                "submitAnalytics" => (self.submit_analytics).inspect_value(path, offset, limit),
                "m_SpriteBatchMaxVertexCount" => {
                    (self.m_sprite_batch_max_vertex_count).inspect_value(path, offset, limit)
                }
                "wiiLoadingScreenRectPlacement" => {
                    (self.wii_loading_screen_rect_placement).inspect_value(path, offset, limit)
                }
                "muteOtherAudioSources" => {
                    (self.mute_other_audio_sources).inspect_value(path, offset, limit)
                }
                "mipStripping" => (self.mip_stripping).inspect_value(path, offset, limit),
                "AID" => (self.aid).inspect_value(path, offset, limit),
                "wiiLoadingScreenBackground" => {
                    (self.wii_loading_screen_background).inspect_value(path, offset, limit)
                }
                "bakeCollisionMeshes" => {
                    (self.bake_collision_meshes).inspect_value(path, offset, limit)
                }
                "numberOfMipsStripped" => {
                    (self.number_of_mips_stripped).inspect_value(path, offset, limit)
                }
                "playerMinOpenGLESVersion" => {
                    (self.player_min_open_gles_version).inspect_value(path, offset, limit)
                }
                "wiiLoadingScreenPeriod" => {
                    (self.wii_loading_screen_period).inspect_value(path, offset, limit)
                }
                "xboxEnableFitness" => {
                    (self.xbox_enable_fitness).inspect_value(path, offset, limit)
                }
                "gpuSkinning" => (self.gpu_skinning).inspect_value(path, offset, limit),
                "wiiLoadingScreenFileName" => {
                    (self.wii_loading_screen_file_name).inspect_value(path, offset, limit)
                }
                "macFullscreenMode" => {
                    (self.mac_fullscreen_mode).inspect_value(path, offset, limit)
                }
                "iosUseCustomAppBackgroundBehavior" => {
                    (self.ios_use_custom_app_background_behavior).inspect_value(path, offset, limit)
                }
                "wiiLoadingScreenRect" => {
                    (self.wii_loading_screen_rect).inspect_value(path, offset, limit)
                }
                "xboxSpeechDB" => (self.xbox_speech_db).inspect_value(path, offset, limit),
                "xboxPIXTextureCapture" => {
                    (self.xbox_pix_texture_capture).inspect_value(path, offset, limit)
                }
                "debugUnloadMode" => (self.debug_unload_mode).inspect_value(path, offset, limit),
                "xboxEnableHeadOrientation" => {
                    (self.xbox_enable_head_orientation).inspect_value(path, offset, limit)
                }
                "numberOfMipsStrippedPerMipmapLimitGroup" => (self
                    .number_of_mips_stripped_per_mipmap_limit_group)
                    .inspect_value(path, offset, limit),
                "graphicsJobs" => (self.graphics_jobs).inspect_value(path, offset, limit),
                "visibleInBackground" => {
                    (self.visible_in_background).inspect_value(path, offset, limit)
                }
                "preserveFramebufferAlpha" => {
                    (self.preserve_framebuffer_alpha).inspect_value(path, offset, limit)
                }
                "d3d9FullscreenMode" => {
                    (self.d3d9_fullscreen_mode).inspect_value(path, offset, limit)
                }
                "xboxEnableGuest" => (self.xbox_enable_guest).inspect_value(path, offset, limit),
                "androidBlitType" => (self.android_blit_type).inspect_value(path, offset, limit),
                "androidStartInFullscreen" => {
                    (self.android_start_in_fullscreen).inspect_value(path, offset, limit)
                }
                "iPhoneBundleIdentifier" => {
                    (self.i_phone_bundle_identifier).inspect_value(path, offset, limit)
                }
                "d3d11ForceExclusiveMode" => {
                    (self.d3d11_force_exclusive_mode).inspect_value(path, offset, limit)
                }
                "d3d11FullscreenMode" => {
                    (self.d3d11_fullscreen_mode).inspect_value(path, offset, limit)
                }
                "allowFullscreenSwitch" => {
                    (self.allow_fullscreen_switch).inspect_value(path, offset, limit)
                }
                "androidRenderOutsideSafeArea" => {
                    (self.android_render_outside_safe_area).inspect_value(path, offset, limit)
                }
                "videoMemoryForVertexBuffers" => {
                    (self.video_memory_for_vertex_buffers).inspect_value(path, offset, limit)
                }
                "androidUseSwappy" => (self.android_use_swappy).inspect_value(path, offset, limit),
                "macRetinaSupport" => (self.mac_retina_support).inspect_value(path, offset, limit),
                "androidResizableWindow" => {
                    (self.android_resizable_window).inspect_value(path, offset, limit)
                }
                "metroEnableIndependentInputSource" => {
                    (self.metro_enable_independent_input_source).inspect_value(path, offset, limit)
                }
                "xboxOneResolution" => {
                    (self.xbox_one_resolution).inspect_value(path, offset, limit)
                }
                "n3dsDisableStereoscopicView" => {
                    (self.n3ds_disable_stereoscopic_view).inspect_value(path, offset, limit)
                }
                "deferSystemGesturesMode" => {
                    (self.defer_system_gestures_mode).inspect_value(path, offset, limit)
                }
                "Force IOS Speakers When Recording" => {
                    (self.force_ios_speakers_when_recording).inspect_value(path, offset, limit)
                }
                "androidDefaultWindowWidth" => {
                    (self.android_default_window_width).inspect_value(path, offset, limit)
                }
                "metroEnableLowLatencyPresentationAPI" => (self
                    .metro_enable_low_latency_presentation_api)
                    .inspect_value(path, offset, limit),
                "ps3SplashScreen" => (self.ps3_splash_screen).inspect_value(path, offset, limit),
                "n3dsEnableSharedListOpt" => {
                    (self.n3ds_enable_shared_list_opt).inspect_value(path, offset, limit)
                }
                "hideHomeButton" => (self.hide_home_button).inspect_value(path, offset, limit),
                "androidDefaultWindowHeight" => {
                    (self.android_default_window_height).inspect_value(path, offset, limit)
                }
                "adjustIOSFPSUsingThermalState" => {
                    (self.adjust_iosfps_using_thermal_state).inspect_value(path, offset, limit)
                }
                "n3dsEnableVSync" => (self.n3ds_enable_v_sync).inspect_value(path, offset, limit),
                "androidMinimumWindowWidth" => {
                    (self.android_minimum_window_width).inspect_value(path, offset, limit)
                }
                "thermalStateSeriousIOSFPS" => {
                    (self.thermal_state_serious_iosfps).inspect_value(path, offset, limit)
                }
                "psp2PowerMode" => (self.psp2_power_mode).inspect_value(path, offset, limit),
                "xboxEnablePIXSampling" => {
                    (self.xbox_enable_pix_sampling).inspect_value(path, offset, limit)
                }
                "androidMinimumWindowHeight" => {
                    (self.android_minimum_window_height).inspect_value(path, offset, limit)
                }
                "thermalStateCriticalIOSFPS" => {
                    (self.thermal_state_critical_iosfps).inspect_value(path, offset, limit)
                }
                "psp2AcquireBGM" => (self.psp2_acquire_bgm).inspect_value(path, offset, limit),
                "xboxEnableEnableRenderThreadRunsJobs" => (self
                    .xbox_enable_enable_render_thread_runs_jobs)
                    .inspect_value(path, offset, limit),
                "androidFullscreenMode" => {
                    (self.android_fullscreen_mode).inspect_value(path, offset, limit)
                }
                "androidDisplayOptions" => {
                    (self.android_display_options).inspect_value(path, offset, limit)
                }
                "bundleIdentifier" => (self.bundle_identifier).inspect_value(path, offset, limit),
                "macAppStoreCategory" => {
                    (self.mac_app_store_category).inspect_value(path, offset, limit)
                }
                "androidResizeableActivity" => {
                    (self.android_resizeable_activity).inspect_value(path, offset, limit)
                }
                "bundleVersion" => (self.bundle_version).inspect_value(path, offset, limit),
                "uiUse16BitDepthBuffer" => {
                    (self.ui_use16_bit_depth_buffer).inspect_value(path, offset, limit)
                }
                "preloadedAssets" => (self.preloaded_assets).inspect_value(path, offset, limit),
                "wiiUTVResolution" => (self.wii_utv_resolution).inspect_value(path, offset, limit),
                "ignoreAlphaClear" => (self.ignore_alpha_clear).inspect_value(path, offset, limit),
                "androidRequestedVisibleInsets" => {
                    (self.android_requested_visible_insets).inspect_value(path, offset, limit)
                }
                "wiiUGamePadMSAA" => (self.wii_u_game_pad_msaa).inspect_value(path, offset, limit),
                "androidSystemBarsBehavior" => {
                    (self.android_system_bars_behavior).inspect_value(path, offset, limit)
                }
                "wiiUSupportsNunchuk" => {
                    (self.wii_u_supports_nunchuk).inspect_value(path, offset, limit)
                }
                "androidAutoRotationBehavior" => {
                    (self.android_auto_rotation_behavior).inspect_value(path, offset, limit)
                }
                "xboxOneDisableKinectGpuReservation" => (self
                    .xbox_one_disable_kinect_gpu_reservation)
                    .inspect_value(path, offset, limit),
                "wiiUSupportsClassicController" => {
                    (self.wii_u_supports_classic_controller).inspect_value(path, offset, limit)
                }
                "xboxOneMonoLoggingLevel" => {
                    (self.xbox_one_mono_logging_level).inspect_value(path, offset, limit)
                }
                "androidPredictiveBackSupport" => {
                    (self.android_predictive_back_support).inspect_value(path, offset, limit)
                }
                "androidApplicationEntry" => {
                    (self.android_application_entry).inspect_value(path, offset, limit)
                }
                "virtualRealitySupported" => {
                    (self.virtual_reality_supported).inspect_value(path, offset, limit)
                }
                "wiiUSupportsBalanceBoard" => {
                    (self.wii_u_supports_balance_board).inspect_value(path, offset, limit)
                }
                "xboxOneLoggingLevel" => {
                    (self.xbox_one_logging_level).inspect_value(path, offset, limit)
                }
                "useFlipModelSwapchain" => {
                    (self.use_flip_model_swapchain).inspect_value(path, offset, limit)
                }
                "projectId" => (self.project_id).inspect_value(path, offset, limit),
                "wiiUSupportsMotionPlus" => {
                    (self.wii_u_supports_motion_plus).inspect_value(path, offset, limit)
                }
                "projectName" => (self.project_name).inspect_value(path, offset, limit),
                "wiiUSupportsProController" => {
                    (self.wii_u_supports_pro_controller).inspect_value(path, offset, limit)
                }
                "organizationId" => (self.organization_id).inspect_value(path, offset, limit),
                "wiiUAllowScreenCapture" => {
                    (self.wii_u_allow_screen_capture).inspect_value(path, offset, limit)
                }
                "graphicsJobMode" => (self.graphics_job_mode).inspect_value(path, offset, limit),
                "cloudEnabled" => (self.cloud_enabled).inspect_value(path, offset, limit),
                "wiiUControllerCount" => {
                    (self.wii_u_controller_count).inspect_value(path, offset, limit)
                }
                "audioSpatialExperience" => {
                    (self.audio_spatial_experience).inspect_value(path, offset, limit)
                }
                "dedicatedServerOptimizations" => {
                    (self.dedicated_server_optimizations).inspect_value(path, offset, limit)
                }
                "fullscreenMode" => (self.fullscreen_mode).inspect_value(path, offset, limit),
                "callOnDisableOnAssetBundleUnload" => {
                    (self.call_on_disable_on_asset_bundle_unload).inspect_value(path, offset, limit)
                }
                "metalFramebufferOnly" => {
                    (self.metal_framebuffer_only).inspect_value(path, offset, limit)
                }
                "xboxOneSResolution" => {
                    (self.xbox_one_s_resolution).inspect_value(path, offset, limit)
                }
                "xboxOneXResolution" => {
                    (self.xbox_one_x_resolution).inspect_value(path, offset, limit)
                }
                "meshDeformation" => (self.mesh_deformation).inspect_value(path, offset, limit),
                "xboxOneDisableEsram" => {
                    (self.xbox_one_disable_esram).inspect_value(path, offset, limit)
                }
                "singlePassStereoRendering" => {
                    (self.single_pass_stereo_rendering).inspect_value(path, offset, limit)
                }
                "protectGraphicsMemory" => {
                    (self.protect_graphics_memory).inspect_value(path, offset, limit)
                }
                "xboxOnePresentImmediateThreshold" => {
                    (self.xbox_one_present_immediate_threshold).inspect_value(path, offset, limit)
                }
                "xboxOneEnableTypeOptimization" => {
                    (self.xbox_one_enable_type_optimization).inspect_value(path, offset, limit)
                }
                "switchQueueCommandMemory" => {
                    (self.switch_queue_command_memory).inspect_value(path, offset, limit)
                }
                "switchQueueControlMemory" => {
                    (self.switch_queue_control_memory).inspect_value(path, offset, limit)
                }
                "vulkanEnableSetSRGBWrite" => {
                    (self.vulkan_enable_set_srgb_write).inspect_value(path, offset, limit)
                }
                "switchQueueComputeMemory" => {
                    (self.switch_queue_compute_memory).inspect_value(path, offset, limit)
                }
                "switchNVNShaderPoolsGranularity" => {
                    (self.switch_nvn_shader_pools_granularity).inspect_value(path, offset, limit)
                }
                "switchNVNDefaultPoolsGranularity" => {
                    (self.switch_nvn_default_pools_granularity).inspect_value(path, offset, limit)
                }
                "switchNVNOtherPoolsGranularity" => {
                    (self.switch_nvn_other_pools_granularity).inspect_value(path, offset, limit)
                }
                "metroInputSource" => (self.metro_input_source).inspect_value(path, offset, limit),
                "vulkanNumSwapchainBuffers" => {
                    (self.vulkan_num_swapchain_buffers).inspect_value(path, offset, limit)
                }
                "wsaTransparentSwapchain" => {
                    (self.wsa_transparent_swapchain).inspect_value(path, offset, limit)
                }
                "vulkanUseSWCommandBuffers" => {
                    (self.vulkan_use_sw_command_buffers).inspect_value(path, offset, limit)
                }
                "switchNVNMaxPublicTextureIDCount" => {
                    (self.switch_nvn_max_public_texture_id_count).inspect_value(path, offset, limit)
                }
                "stadiaPresentMode" => {
                    (self.stadia_present_mode).inspect_value(path, offset, limit)
                }
                "m_HolographicPauseOnTrackingLoss" => {
                    (self.m_holographic_pause_on_tracking_loss).inspect_value(path, offset, limit)
                }
                "switchNVNMaxPublicSamplerIDCount" => {
                    (self.switch_nvn_max_public_sampler_id_count).inspect_value(path, offset, limit)
                }
                "stadiaTargetFramerate" => {
                    (self.stadia_target_framerate).inspect_value(path, offset, limit)
                }
                "xboxOneEnable7thCore" => {
                    (self.xbox_one_enable7th_core).inspect_value(path, offset, limit)
                }
                "isWsaHolographicRemotingEnabled" => {
                    (self.is_wsa_holographic_remoting_enabled).inspect_value(path, offset, limit)
                }
                "vrSettings" => (self.vr_settings).inspect_value(path, offset, limit),
                "metalUseMetalDisplayLink" => {
                    (self.metal_use_metal_display_link).inspect_value(path, offset, limit)
                }
                "vulkanEnableLateAcquireNextImage" => {
                    (self.vulkan_enable_late_acquire_next_image).inspect_value(path, offset, limit)
                }
                "useHDRDisplay" => (self.use_hdr_display).inspect_value(path, offset, limit),
                "enableFrameTimingStats" => {
                    (self.enable_frame_timing_stats).inspect_value(path, offset, limit)
                }
                "m_ColorGamuts" => (self.m_color_gamuts).inspect_value(path, offset, limit),
                "targetPixelDensity" => {
                    (self.target_pixel_density).inspect_value(path, offset, limit)
                }
                "vulkanEnablePreTransform" => {
                    (self.vulkan_enable_pre_transform).inspect_value(path, offset, limit)
                }
                "switchGpuScratchPoolGranularity" => {
                    (self.switch_gpu_scratch_pool_granularity).inspect_value(path, offset, limit)
                }
                "resolutionScalingMode" => {
                    (self.resolution_scaling_mode).inspect_value(path, offset, limit)
                }
                "switchAllowGpuScratchShrinking" => {
                    (self.switch_allow_gpu_scratch_shrinking).inspect_value(path, offset, limit)
                }
                "enableNewInputSystem" => {
                    (self.enable_new_input_system).inspect_value(path, offset, limit)
                }
                "androidSupportedAspectRatio" => {
                    (self.android_supported_aspect_ratio).inspect_value(path, offset, limit)
                }
                "vulkanEnableCommandBufferRecycling" => {
                    (self.vulkan_enable_command_buffer_recycling).inspect_value(path, offset, limit)
                }
                "switchMaxWorkerMultiple" => {
                    (self.switch_max_worker_multiple).inspect_value(path, offset, limit)
                }
                "androidMaxAspectRatio" => {
                    (self.android_max_aspect_ratio).inspect_value(path, offset, limit)
                }
                "D3DHDRBitDepth" => (self.d3dhdr_bit_depth).inspect_value(path, offset, limit),
                "switchNVNGraphicsFirmwareMemory" => {
                    (self.switch_nvn_graphics_firmware_memory).inspect_value(path, offset, limit)
                }
                "framebufferDepthMemorylessMode" => {
                    (self.framebuffer_depth_memoryless_mode).inspect_value(path, offset, limit)
                }
                "enableNativePlatformBackendsForNewInputSystem" => (self
                    .enable_native_platform_backends_for_new_input_system)
                    .inspect_value(path, offset, limit),
                "disableOldInputManagerSupport" => {
                    (self.disable_old_input_manager_support).inspect_value(path, offset, limit)
                }
                "legacyClampBlendShapeWeights" => {
                    (self.legacy_clamp_blend_shape_weights).inspect_value(path, offset, limit)
                }
                "loadStoreDebugModeEnabled" => {
                    (self.load_store_debug_mode_enabled).inspect_value(path, offset, limit)
                }
                "switchGraphicsJobsSyncAfterKick" => {
                    (self.switch_graphics_jobs_sync_after_kick).inspect_value(path, offset, limit)
                }
                "invalidatedPatternTexture" => {
                    (self.invalidated_pattern_texture).inspect_value(path, offset, limit)
                }
                "activeInputHandler" => {
                    (self.active_input_handler).inspect_value(path, offset, limit)
                }
                "enableOpenGLProfilerGPURecorders" => {
                    (self.enable_open_gl_profiler_gpu_recorders).inspect_value(path, offset, limit)
                }
                "visionOSBundleVersion" => {
                    (self.vision_os_bundle_version).inspect_value(path, offset, limit)
                }
                "tvOSBundleVersion" => {
                    (self.tv_os_bundle_version).inspect_value(path, offset, limit)
                }
                "virtualTexturingSupportEnabled" => {
                    (self.virtual_texturing_support_enabled).inspect_value(path, offset, limit)
                }
                "qualitySettingsNames" => {
                    (self.quality_settings_names).inspect_value(path, offset, limit)
                }
                "resetResolutionOnWindowResize" => {
                    (self.reset_resolution_on_window_resize).inspect_value(path, offset, limit)
                }
                "hdrBitDepth" => (self.hdr_bit_depth).inspect_value(path, offset, limit),
                "windowsGamepadBackendHint" => {
                    (self.windows_gamepad_backend_hint).inspect_value(path, offset, limit)
                }
                "allowHDRDisplaySupport" => {
                    (self.allow_hdr_display_support).inspect_value(path, offset, limit)
                }
                "playerDataPath" => (self.player_data_path).inspect_value(path, offset, limit),
                "forceSRGBBlit" => (self.force_srgb_blit).inspect_value(path, offset, limit),
                "uploadClearedTextureDataAfterCreationFromScript" => (self
                    .upload_cleared_texture_data_after_creation_from_script)
                    .inspect_value(path, offset, limit),
                "androidMinAspectRatio" => {
                    (self.android_min_aspect_ratio).inspect_value(path, offset, limit)
                }
                "insecureHttpOption" => {
                    (self.insecure_http_option).inspect_value(path, offset, limit)
                }
                "hmiLoadingImage" => (self.hmi_loading_image).inspect_value(path, offset, limit),
                "platformRequiresReadableAssets" => {
                    (self.platform_requires_readable_assets).inspect_value(path, offset, limit)
                }
                "enableDirectStorage" => {
                    (self.enable_direct_storage).inspect_value(path, offset, limit)
                }
                "androidVulkanDenyFilterList" => {
                    (self.android_vulkan_deny_filter_list).inspect_value(path, offset, limit)
                }
                "androidVulkanAllowFilterList" => {
                    (self.android_vulkan_allow_filter_list).inspect_value(path, offset, limit)
                }
                "androidVulkanDeviceFilterListAsset" => (self
                    .android_vulkan_device_filter_list_asset)
                    .inspect_value(path, offset, limit),
                "d3d12DeviceFilterListAsset" => {
                    (self.d3d12_device_filter_list_asset).inspect_value(path, offset, limit)
                }
                "allowedHttpConnections" => {
                    (self.allowed_http_connections).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.android_profiler).inspect_value(path, offset, limit))
                .map(|v| ("AndroidProfiler".to_string(), v)),
            ((self.product_guid).inspect_value(path, offset, limit))
                .map(|v| ("productGUID".to_string(), v)),
            ((self.default_screen_orientation).inspect_value(path, offset, limit))
                .map(|v| ("defaultScreenOrientation".to_string(), v)),
            ((self.target_device).inspect_value(path, offset, limit))
                .map(|v| ("targetDevice".to_string(), v)),
            ((self.android_filter_touches_when_obscured).inspect_value(path, offset, limit))
                .map(|v| ("AndroidFilterTouchesWhenObscured".to_string(), v)),
            ((self.target_platform).inspect_value(path, offset, limit))
                .map(|v| ("targetPlatform".to_string(), v)),
            ((self.target_resolution).inspect_value(path, offset, limit))
                .map(|v| ("targetResolution".to_string(), v)),
            ((self.target_gles_graphics).inspect_value(path, offset, limit))
                .map(|v| ("targetGlesGraphics".to_string(), v)),
            ((self.use_on_demand_resources).inspect_value(path, offset, limit))
                .map(|v| ("useOnDemandResources".to_string(), v)),
            ((self.android_enable_sustained_performance_mode).inspect_value(path, offset, limit))
                .map(|v| ("AndroidEnableSustainedPerformanceMode".to_string(), v)),
            ((self.accelerometer_frequency).inspect_value(path, offset, limit))
                .map(|v| ("accelerometerFrequency".to_string(), v)),
            ((self.target_ios_graphics).inspect_value(path, offset, limit))
                .map(|v| ("targetIOSGraphics".to_string(), v)),
            ((self.company_name).inspect_value(path, offset, limit))
                .map(|v| ("companyName".to_string(), v)),
            ((self.product_name).inspect_value(path, offset, limit))
                .map(|v| ("productName".to_string(), v)),
            ((self.android_license_public_key).inspect_value(path, offset, limit))
                .map(|v| ("AndroidLicensePublicKey".to_string(), v)),
            ((self.default_cursor).inspect_value(path, offset, limit))
                .map(|v| ("defaultCursor".to_string(), v)),
            ((self.default_screen_width).inspect_value(path, offset, limit))
                .map(|v| ("defaultScreenWidth".to_string(), v)),
            ((self.cursor_hotspot).inspect_value(path, offset, limit))
                .map(|v| ("cursorHotspot".to_string(), v)),
            ((self.default_screen_height).inspect_value(path, offset, limit))
                .map(|v| ("defaultScreenHeight".to_string(), v)),
            ((self.cloud_project_id).inspect_value(path, offset, limit))
                .map(|v| ("cloudProjectId".to_string(), v)),
            ((self.m_show_unity_splash_screen).inspect_value(path, offset, limit))
                .map(|v| ("m_ShowUnitySplashScreen".to_string(), v)),
            ((self.default_screen_width_web).inspect_value(path, offset, limit))
                .map(|v| ("defaultScreenWidthWeb".to_string(), v)),
            ((self.m_virtual_reality_splash_screen).inspect_value(path, offset, limit))
                .map(|v| ("m_VirtualRealitySplashScreen".to_string(), v)),
            ((self.m_splash_screen_style).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenStyle".to_string(), v)),
            ((self.m_splash_screen_background_color).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenBackgroundColor".to_string(), v)),
            ((self.default_screen_height_web).inspect_value(path, offset, limit))
                .map(|v| ("defaultScreenHeightWeb".to_string(), v)),
            ((self.m_rendering_path).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderingPath".to_string(), v)),
            ((self.m_show_unity_splash_logo).inspect_value(path, offset, limit))
                .map(|v| ("m_ShowUnitySplashLogo".to_string(), v)),
            ((self.display_resolution_dialog).inspect_value(path, offset, limit))
                .map(|v| ("displayResolutionDialog".to_string(), v)),
            ((self.m_splash_screen_overlay_opacity).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenOverlayOpacity".to_string(), v)),
            ((self.allowed_autorotate_to_portrait).inspect_value(path, offset, limit))
                .map(|v| ("allowedAutorotateToPortrait".to_string(), v)),
            ((self.m_active_color_space).inspect_value(path, offset, limit))
                .map(|v| ("m_ActiveColorSpace".to_string(), v)),
            ((self.m_splash_screen_animation).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenAnimation".to_string(), v)),
            ((self.allowed_autorotate_to_portrait_upside_down).inspect_value(path, offset, limit))
                .map(|v| ("allowedAutorotateToPortraitUpsideDown".to_string(), v)),
            ((self.m_mt_rendering).inspect_value(path, offset, limit))
                .map(|v| ("m_MTRendering".to_string(), v)),
            ((self.m_mobile_rendering_path).inspect_value(path, offset, limit))
                .map(|v| ("m_MobileRenderingPath".to_string(), v)),
            ((self.m_splash_screen_logo_style).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenLogoStyle".to_string(), v)),
            ((self.allowed_autorotate_to_landscape_right).inspect_value(path, offset, limit))
                .map(|v| ("allowedAutorotateToLandscapeRight".to_string(), v)),
            ((self.ios_show_activity_indicator_on_loading).inspect_value(path, offset, limit))
                .map(|v| ("iosShowActivityIndicatorOnLoading".to_string(), v)),
            ((self.m_splash_screen_draw_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenDrawMode".to_string(), v)),
            ((self.allowed_autorotate_to_landscape_left).inspect_value(path, offset, limit))
                .map(|v| ("allowedAutorotateToLandscapeLeft".to_string(), v)),
            ((self.android_show_activity_indicator_on_loading).inspect_value(path, offset, limit))
                .map(|v| ("androidShowActivityIndicatorOnLoading".to_string(), v)),
            ((self.m_use_dx11).inspect_value(path, offset, limit))
                .map(|v| ("m_UseDX11".to_string(), v)),
            ((self.m_splash_screen_background_animation_zoom).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenBackgroundAnimationZoom".to_string(), v)),
            ((self.use_os_autorotation).inspect_value(path, offset, limit))
                .map(|v| ("useOSAutorotation".to_string(), v)),
            ((self.m_mobile_mt_rendering).inspect_value(path, offset, limit))
                .map(|v| ("m_MobileMTRendering".to_string(), v)),
            ((self.m_splash_screen_logo_animation_zoom).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenLogoAnimationZoom".to_string(), v)),
            ((self.use32_bit_display_buffer).inspect_value(path, offset, limit))
                .map(|v| ("use32BitDisplayBuffer".to_string(), v)),
            ((self.m_stereoscopic3_d).inspect_value(path, offset, limit))
                .map(|v| ("m_Stereoscopic3D".to_string(), v)),
            ((self.m_splash_screen_background_landscape_aspect).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenBackgroundLandscapeAspect".to_string(), v)),
            ((self.default_is_full_screen).inspect_value(path, offset, limit))
                .map(|v| ("defaultIsFullScreen".to_string(), v)),
            ((self.m_splash_screen_background_portrait_aspect).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenBackgroundPortraitAspect".to_string(), v)),
            ((self.use_alpha_in_dashboard).inspect_value(path, offset, limit))
                .map(|v| ("useAlphaInDashboard".to_string(), v)),
            ((self.m_splash_screen_background_landscape_uvs).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenBackgroundLandscapeUvs".to_string(), v)),
            ((self.run_in_background).inspect_value(path, offset, limit))
                .map(|v| ("runInBackground".to_string(), v)),
            ((self.ios_app_in_background_behavior).inspect_value(path, offset, limit))
                .map(|v| ("iosAppInBackgroundBehavior".to_string(), v)),
            ((self.m_stack_trace_types).inspect_value(path, offset, limit))
                .map(|v| ("m_StackTraceTypes".to_string(), v)),
            ((self.m_splash_screen_background_portrait_uvs).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenBackgroundPortraitUvs".to_string(), v)),
            ((self.capture_single_screen).inspect_value(path, offset, limit))
                .map(|v| ("captureSingleScreen".to_string(), v)),
            ((self.m_splash_screen_logos).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenLogos".to_string(), v)),
            ((self.override_i_pod_music).inspect_value(path, offset, limit))
                .map(|v| ("Override IPod Music".to_string(), v)),
            ((self.m_splash_screen_background_landscape).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenBackgroundLandscape".to_string(), v)),
            ((self.use_player_log).inspect_value(path, offset, limit))
                .map(|v| ("usePlayerLog".to_string(), v)),
            ((self.use24_bit_depth_buffer).inspect_value(path, offset, limit))
                .map(|v| ("use24BitDepthBuffer".to_string(), v)),
            ((self.ios_allow_http_download).inspect_value(path, offset, limit))
                .map(|v| ("iosAllowHTTPDownload".to_string(), v)),
            ((self.tizen_show_activity_indicator_on_loading).inspect_value(path, offset, limit))
                .map(|v| ("tizenShowActivityIndicatorOnLoading".to_string(), v)),
            ((self.m_splash_screen_background_portrait).inspect_value(path, offset, limit))
                .map(|v| ("m_SplashScreenBackgroundPortrait".to_string(), v)),
            ((self.use_mac_app_store_validation).inspect_value(path, offset, limit))
                .map(|v| ("useMacAppStoreValidation".to_string(), v)),
            ((self.xbox_skin_on_gpu).inspect_value(path, offset, limit))
                .map(|v| ("xboxSkinOnGPU".to_string(), v)),
            ((self.m_holographic_tracking_loss_screen).inspect_value(path, offset, limit))
                .map(|v| ("m_HolographicTrackingLossScreen".to_string(), v)),
            ((self.m_supported_aspect_ratios).inspect_value(path, offset, limit))
                .map(|v| ("m_SupportedAspectRatios".to_string(), v)),
            ((self.m_unity_splash_logo).inspect_value(path, offset, limit))
                .map(|v| ("m_UnitySplashLogo".to_string(), v)),
            ((self.default_is_native_resolution).inspect_value(path, offset, limit))
                .map(|v| ("defaultIsNativeResolution".to_string(), v)),
            ((self.disable_depth_and_stencil_buffers).inspect_value(path, offset, limit))
                .map(|v| ("disableDepthAndStencilBuffers".to_string(), v)),
            ((self.enable_hw_statistics).inspect_value(path, offset, limit))
                .map(|v| ("enableHWStatistics".to_string(), v)),
            ((self.m_stereo_rendering_path).inspect_value(path, offset, limit))
                .map(|v| ("m_StereoRenderingPath".to_string(), v)),
            ((self.xbox_enable_avatar).inspect_value(path, offset, limit))
                .map(|v| ("xboxEnableAvatar".to_string(), v)),
            ((self.strip_physics).inspect_value(path, offset, limit))
                .map(|v| ("stripPhysics".to_string(), v)),
            ((self.prepare_ios_for_recording).inspect_value(path, offset, limit))
                .map(|v| ("Prepare IOS For Recording".to_string(), v)),
            ((self.xbox_enable_kinect).inspect_value(path, offset, limit))
                .map(|v| ("xboxEnableKinect".to_string(), v)),
            ((self.resizable_window).inspect_value(path, offset, limit))
                .map(|v| ("resizableWindow".to_string(), v)),
            ((self.xbox_enable_kinect_auto_tracking).inspect_value(path, offset, limit))
                .map(|v| ("xboxEnableKinectAutoTracking".to_string(), v)),
            ((self.xbox_enable_speech).inspect_value(path, offset, limit))
                .map(|v| ("xboxEnableSpeech".to_string(), v)),
            ((self.mobile_mt_rendering_baked).inspect_value(path, offset, limit))
                .map(|v| ("mobileMTRenderingBaked".to_string(), v)),
            ((self.unsupported_msaa_fallback).inspect_value(path, offset, limit))
                .map(|v| ("unsupportedMSAAFallback".to_string(), v)),
            ((self.m_sprite_batch_vertex_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_SpriteBatchVertexThreshold".to_string(), v)),
            ((self.wii_hio2_usage).inspect_value(path, offset, limit))
                .map(|v| ("wiiHio2Usage".to_string(), v)),
            ((self.force_single_instance).inspect_value(path, offset, limit))
                .map(|v| ("forceSingleInstance".to_string(), v)),
            ((self.submit_analytics).inspect_value(path, offset, limit))
                .map(|v| ("submitAnalytics".to_string(), v)),
            ((self.m_sprite_batch_max_vertex_count).inspect_value(path, offset, limit))
                .map(|v| ("m_SpriteBatchMaxVertexCount".to_string(), v)),
            ((self.wii_loading_screen_rect_placement).inspect_value(path, offset, limit))
                .map(|v| ("wiiLoadingScreenRectPlacement".to_string(), v)),
            ((self.mute_other_audio_sources).inspect_value(path, offset, limit))
                .map(|v| ("muteOtherAudioSources".to_string(), v)),
            ((self.mip_stripping).inspect_value(path, offset, limit))
                .map(|v| ("mipStripping".to_string(), v)),
            ((self.aid).inspect_value(path, offset, limit)).map(|v| ("AID".to_string(), v)),
            ((self.wii_loading_screen_background).inspect_value(path, offset, limit))
                .map(|v| ("wiiLoadingScreenBackground".to_string(), v)),
            ((self.bake_collision_meshes).inspect_value(path, offset, limit))
                .map(|v| ("bakeCollisionMeshes".to_string(), v)),
            ((self.number_of_mips_stripped).inspect_value(path, offset, limit))
                .map(|v| ("numberOfMipsStripped".to_string(), v)),
            ((self.player_min_open_gles_version).inspect_value(path, offset, limit))
                .map(|v| ("playerMinOpenGLESVersion".to_string(), v)),
            ((self.wii_loading_screen_period).inspect_value(path, offset, limit))
                .map(|v| ("wiiLoadingScreenPeriod".to_string(), v)),
            ((self.xbox_enable_fitness).inspect_value(path, offset, limit))
                .map(|v| ("xboxEnableFitness".to_string(), v)),
            ((self.gpu_skinning).inspect_value(path, offset, limit))
                .map(|v| ("gpuSkinning".to_string(), v)),
            ((self.wii_loading_screen_file_name).inspect_value(path, offset, limit))
                .map(|v| ("wiiLoadingScreenFileName".to_string(), v)),
            ((self.mac_fullscreen_mode).inspect_value(path, offset, limit))
                .map(|v| ("macFullscreenMode".to_string(), v)),
            ((self.ios_use_custom_app_background_behavior).inspect_value(path, offset, limit))
                .map(|v| ("iosUseCustomAppBackgroundBehavior".to_string(), v)),
            ((self.wii_loading_screen_rect).inspect_value(path, offset, limit))
                .map(|v| ("wiiLoadingScreenRect".to_string(), v)),
            ((self.xbox_speech_db).inspect_value(path, offset, limit))
                .map(|v| ("xboxSpeechDB".to_string(), v)),
            ((self.xbox_pix_texture_capture).inspect_value(path, offset, limit))
                .map(|v| ("xboxPIXTextureCapture".to_string(), v)),
            ((self.debug_unload_mode).inspect_value(path, offset, limit))
                .map(|v| ("debugUnloadMode".to_string(), v)),
            ((self.xbox_enable_head_orientation).inspect_value(path, offset, limit))
                .map(|v| ("xboxEnableHeadOrientation".to_string(), v)),
            ((self.number_of_mips_stripped_per_mipmap_limit_group)
                .inspect_value(path, offset, limit))
            .map(|v| ("numberOfMipsStrippedPerMipmapLimitGroup".to_string(), v)),
            ((self.graphics_jobs).inspect_value(path, offset, limit))
                .map(|v| ("graphicsJobs".to_string(), v)),
            ((self.visible_in_background).inspect_value(path, offset, limit))
                .map(|v| ("visibleInBackground".to_string(), v)),
            ((self.preserve_framebuffer_alpha).inspect_value(path, offset, limit))
                .map(|v| ("preserveFramebufferAlpha".to_string(), v)),
            ((self.d3d9_fullscreen_mode).inspect_value(path, offset, limit))
                .map(|v| ("d3d9FullscreenMode".to_string(), v)),
            ((self.xbox_enable_guest).inspect_value(path, offset, limit))
                .map(|v| ("xboxEnableGuest".to_string(), v)),
            ((self.android_blit_type).inspect_value(path, offset, limit))
                .map(|v| ("androidBlitType".to_string(), v)),
            ((self.android_start_in_fullscreen).inspect_value(path, offset, limit))
                .map(|v| ("androidStartInFullscreen".to_string(), v)),
            ((self.i_phone_bundle_identifier).inspect_value(path, offset, limit))
                .map(|v| ("iPhoneBundleIdentifier".to_string(), v)),
            ((self.d3d11_force_exclusive_mode).inspect_value(path, offset, limit))
                .map(|v| ("d3d11ForceExclusiveMode".to_string(), v)),
            ((self.d3d11_fullscreen_mode).inspect_value(path, offset, limit))
                .map(|v| ("d3d11FullscreenMode".to_string(), v)),
            ((self.allow_fullscreen_switch).inspect_value(path, offset, limit))
                .map(|v| ("allowFullscreenSwitch".to_string(), v)),
            ((self.android_render_outside_safe_area).inspect_value(path, offset, limit))
                .map(|v| ("androidRenderOutsideSafeArea".to_string(), v)),
            ((self.video_memory_for_vertex_buffers).inspect_value(path, offset, limit))
                .map(|v| ("videoMemoryForVertexBuffers".to_string(), v)),
            ((self.android_use_swappy).inspect_value(path, offset, limit))
                .map(|v| ("androidUseSwappy".to_string(), v)),
            ((self.mac_retina_support).inspect_value(path, offset, limit))
                .map(|v| ("macRetinaSupport".to_string(), v)),
            ((self.android_resizable_window).inspect_value(path, offset, limit))
                .map(|v| ("androidResizableWindow".to_string(), v)),
            ((self.metro_enable_independent_input_source).inspect_value(path, offset, limit))
                .map(|v| ("metroEnableIndependentInputSource".to_string(), v)),
            ((self.xbox_one_resolution).inspect_value(path, offset, limit))
                .map(|v| ("xboxOneResolution".to_string(), v)),
            ((self.n3ds_disable_stereoscopic_view).inspect_value(path, offset, limit))
                .map(|v| ("n3dsDisableStereoscopicView".to_string(), v)),
            ((self.defer_system_gestures_mode).inspect_value(path, offset, limit))
                .map(|v| ("deferSystemGesturesMode".to_string(), v)),
            ((self.force_ios_speakers_when_recording).inspect_value(path, offset, limit))
                .map(|v| ("Force IOS Speakers When Recording".to_string(), v)),
            ((self.android_default_window_width).inspect_value(path, offset, limit))
                .map(|v| ("androidDefaultWindowWidth".to_string(), v)),
            ((self.metro_enable_low_latency_presentation_api).inspect_value(path, offset, limit))
                .map(|v| ("metroEnableLowLatencyPresentationAPI".to_string(), v)),
            ((self.ps3_splash_screen).inspect_value(path, offset, limit))
                .map(|v| ("ps3SplashScreen".to_string(), v)),
            ((self.n3ds_enable_shared_list_opt).inspect_value(path, offset, limit))
                .map(|v| ("n3dsEnableSharedListOpt".to_string(), v)),
            ((self.hide_home_button).inspect_value(path, offset, limit))
                .map(|v| ("hideHomeButton".to_string(), v)),
            ((self.android_default_window_height).inspect_value(path, offset, limit))
                .map(|v| ("androidDefaultWindowHeight".to_string(), v)),
            ((self.adjust_iosfps_using_thermal_state).inspect_value(path, offset, limit))
                .map(|v| ("adjustIOSFPSUsingThermalState".to_string(), v)),
            ((self.n3ds_enable_v_sync).inspect_value(path, offset, limit))
                .map(|v| ("n3dsEnableVSync".to_string(), v)),
            ((self.android_minimum_window_width).inspect_value(path, offset, limit))
                .map(|v| ("androidMinimumWindowWidth".to_string(), v)),
            ((self.thermal_state_serious_iosfps).inspect_value(path, offset, limit))
                .map(|v| ("thermalStateSeriousIOSFPS".to_string(), v)),
            ((self.psp2_power_mode).inspect_value(path, offset, limit))
                .map(|v| ("psp2PowerMode".to_string(), v)),
            ((self.xbox_enable_pix_sampling).inspect_value(path, offset, limit))
                .map(|v| ("xboxEnablePIXSampling".to_string(), v)),
            ((self.android_minimum_window_height).inspect_value(path, offset, limit))
                .map(|v| ("androidMinimumWindowHeight".to_string(), v)),
            ((self.thermal_state_critical_iosfps).inspect_value(path, offset, limit))
                .map(|v| ("thermalStateCriticalIOSFPS".to_string(), v)),
            ((self.psp2_acquire_bgm).inspect_value(path, offset, limit))
                .map(|v| ("psp2AcquireBGM".to_string(), v)),
            ((self.xbox_enable_enable_render_thread_runs_jobs).inspect_value(path, offset, limit))
                .map(|v| ("xboxEnableEnableRenderThreadRunsJobs".to_string(), v)),
            ((self.android_fullscreen_mode).inspect_value(path, offset, limit))
                .map(|v| ("androidFullscreenMode".to_string(), v)),
            ((self.android_display_options).inspect_value(path, offset, limit))
                .map(|v| ("androidDisplayOptions".to_string(), v)),
            ((self.bundle_identifier).inspect_value(path, offset, limit))
                .map(|v| ("bundleIdentifier".to_string(), v)),
            ((self.mac_app_store_category).inspect_value(path, offset, limit))
                .map(|v| ("macAppStoreCategory".to_string(), v)),
            ((self.android_resizeable_activity).inspect_value(path, offset, limit))
                .map(|v| ("androidResizeableActivity".to_string(), v)),
            ((self.bundle_version).inspect_value(path, offset, limit))
                .map(|v| ("bundleVersion".to_string(), v)),
            ((self.ui_use16_bit_depth_buffer).inspect_value(path, offset, limit))
                .map(|v| ("uiUse16BitDepthBuffer".to_string(), v)),
            ((self.preloaded_assets).inspect_value(path, offset, limit))
                .map(|v| ("preloadedAssets".to_string(), v)),
            ((self.wii_utv_resolution).inspect_value(path, offset, limit))
                .map(|v| ("wiiUTVResolution".to_string(), v)),
            ((self.ignore_alpha_clear).inspect_value(path, offset, limit))
                .map(|v| ("ignoreAlphaClear".to_string(), v)),
            ((self.android_requested_visible_insets).inspect_value(path, offset, limit))
                .map(|v| ("androidRequestedVisibleInsets".to_string(), v)),
            ((self.wii_u_game_pad_msaa).inspect_value(path, offset, limit))
                .map(|v| ("wiiUGamePadMSAA".to_string(), v)),
            ((self.android_system_bars_behavior).inspect_value(path, offset, limit))
                .map(|v| ("androidSystemBarsBehavior".to_string(), v)),
            ((self.wii_u_supports_nunchuk).inspect_value(path, offset, limit))
                .map(|v| ("wiiUSupportsNunchuk".to_string(), v)),
            ((self.android_auto_rotation_behavior).inspect_value(path, offset, limit))
                .map(|v| ("androidAutoRotationBehavior".to_string(), v)),
            ((self.xbox_one_disable_kinect_gpu_reservation).inspect_value(path, offset, limit))
                .map(|v| ("xboxOneDisableKinectGpuReservation".to_string(), v)),
            ((self.wii_u_supports_classic_controller).inspect_value(path, offset, limit))
                .map(|v| ("wiiUSupportsClassicController".to_string(), v)),
            ((self.xbox_one_mono_logging_level).inspect_value(path, offset, limit))
                .map(|v| ("xboxOneMonoLoggingLevel".to_string(), v)),
            ((self.android_predictive_back_support).inspect_value(path, offset, limit))
                .map(|v| ("androidPredictiveBackSupport".to_string(), v)),
            ((self.android_application_entry).inspect_value(path, offset, limit))
                .map(|v| ("androidApplicationEntry".to_string(), v)),
            ((self.virtual_reality_supported).inspect_value(path, offset, limit))
                .map(|v| ("virtualRealitySupported".to_string(), v)),
            ((self.wii_u_supports_balance_board).inspect_value(path, offset, limit))
                .map(|v| ("wiiUSupportsBalanceBoard".to_string(), v)),
            ((self.xbox_one_logging_level).inspect_value(path, offset, limit))
                .map(|v| ("xboxOneLoggingLevel".to_string(), v)),
            ((self.use_flip_model_swapchain).inspect_value(path, offset, limit))
                .map(|v| ("useFlipModelSwapchain".to_string(), v)),
            ((self.project_id).inspect_value(path, offset, limit))
                .map(|v| ("projectId".to_string(), v)),
            ((self.wii_u_supports_motion_plus).inspect_value(path, offset, limit))
                .map(|v| ("wiiUSupportsMotionPlus".to_string(), v)),
            ((self.project_name).inspect_value(path, offset, limit))
                .map(|v| ("projectName".to_string(), v)),
            ((self.wii_u_supports_pro_controller).inspect_value(path, offset, limit))
                .map(|v| ("wiiUSupportsProController".to_string(), v)),
            ((self.organization_id).inspect_value(path, offset, limit))
                .map(|v| ("organizationId".to_string(), v)),
            ((self.wii_u_allow_screen_capture).inspect_value(path, offset, limit))
                .map(|v| ("wiiUAllowScreenCapture".to_string(), v)),
            ((self.graphics_job_mode).inspect_value(path, offset, limit))
                .map(|v| ("graphicsJobMode".to_string(), v)),
            ((self.cloud_enabled).inspect_value(path, offset, limit))
                .map(|v| ("cloudEnabled".to_string(), v)),
            ((self.wii_u_controller_count).inspect_value(path, offset, limit))
                .map(|v| ("wiiUControllerCount".to_string(), v)),
            ((self.audio_spatial_experience).inspect_value(path, offset, limit))
                .map(|v| ("audioSpatialExperience".to_string(), v)),
            ((self.dedicated_server_optimizations).inspect_value(path, offset, limit))
                .map(|v| ("dedicatedServerOptimizations".to_string(), v)),
            ((self.fullscreen_mode).inspect_value(path, offset, limit))
                .map(|v| ("fullscreenMode".to_string(), v)),
            ((self.call_on_disable_on_asset_bundle_unload).inspect_value(path, offset, limit))
                .map(|v| ("callOnDisableOnAssetBundleUnload".to_string(), v)),
            ((self.metal_framebuffer_only).inspect_value(path, offset, limit))
                .map(|v| ("metalFramebufferOnly".to_string(), v)),
            ((self.xbox_one_s_resolution).inspect_value(path, offset, limit))
                .map(|v| ("xboxOneSResolution".to_string(), v)),
            ((self.xbox_one_x_resolution).inspect_value(path, offset, limit))
                .map(|v| ("xboxOneXResolution".to_string(), v)),
            ((self.mesh_deformation).inspect_value(path, offset, limit))
                .map(|v| ("meshDeformation".to_string(), v)),
            ((self.xbox_one_disable_esram).inspect_value(path, offset, limit))
                .map(|v| ("xboxOneDisableEsram".to_string(), v)),
            ((self.single_pass_stereo_rendering).inspect_value(path, offset, limit))
                .map(|v| ("singlePassStereoRendering".to_string(), v)),
            ((self.protect_graphics_memory).inspect_value(path, offset, limit))
                .map(|v| ("protectGraphicsMemory".to_string(), v)),
            ((self.xbox_one_present_immediate_threshold).inspect_value(path, offset, limit))
                .map(|v| ("xboxOnePresentImmediateThreshold".to_string(), v)),
            ((self.xbox_one_enable_type_optimization).inspect_value(path, offset, limit))
                .map(|v| ("xboxOneEnableTypeOptimization".to_string(), v)),
            ((self.switch_queue_command_memory).inspect_value(path, offset, limit))
                .map(|v| ("switchQueueCommandMemory".to_string(), v)),
            ((self.switch_queue_control_memory).inspect_value(path, offset, limit))
                .map(|v| ("switchQueueControlMemory".to_string(), v)),
            ((self.vulkan_enable_set_srgb_write).inspect_value(path, offset, limit))
                .map(|v| ("vulkanEnableSetSRGBWrite".to_string(), v)),
            ((self.switch_queue_compute_memory).inspect_value(path, offset, limit))
                .map(|v| ("switchQueueComputeMemory".to_string(), v)),
            ((self.switch_nvn_shader_pools_granularity).inspect_value(path, offset, limit))
                .map(|v| ("switchNVNShaderPoolsGranularity".to_string(), v)),
            ((self.switch_nvn_default_pools_granularity).inspect_value(path, offset, limit))
                .map(|v| ("switchNVNDefaultPoolsGranularity".to_string(), v)),
            ((self.switch_nvn_other_pools_granularity).inspect_value(path, offset, limit))
                .map(|v| ("switchNVNOtherPoolsGranularity".to_string(), v)),
            ((self.metro_input_source).inspect_value(path, offset, limit))
                .map(|v| ("metroInputSource".to_string(), v)),
            ((self.vulkan_num_swapchain_buffers).inspect_value(path, offset, limit))
                .map(|v| ("vulkanNumSwapchainBuffers".to_string(), v)),
            ((self.wsa_transparent_swapchain).inspect_value(path, offset, limit))
                .map(|v| ("wsaTransparentSwapchain".to_string(), v)),
            ((self.vulkan_use_sw_command_buffers).inspect_value(path, offset, limit))
                .map(|v| ("vulkanUseSWCommandBuffers".to_string(), v)),
            ((self.switch_nvn_max_public_texture_id_count).inspect_value(path, offset, limit))
                .map(|v| ("switchNVNMaxPublicTextureIDCount".to_string(), v)),
            ((self.stadia_present_mode).inspect_value(path, offset, limit))
                .map(|v| ("stadiaPresentMode".to_string(), v)),
            ((self.m_holographic_pause_on_tracking_loss).inspect_value(path, offset, limit))
                .map(|v| ("m_HolographicPauseOnTrackingLoss".to_string(), v)),
            ((self.switch_nvn_max_public_sampler_id_count).inspect_value(path, offset, limit))
                .map(|v| ("switchNVNMaxPublicSamplerIDCount".to_string(), v)),
            ((self.stadia_target_framerate).inspect_value(path, offset, limit))
                .map(|v| ("stadiaTargetFramerate".to_string(), v)),
            ((self.xbox_one_enable7th_core).inspect_value(path, offset, limit))
                .map(|v| ("xboxOneEnable7thCore".to_string(), v)),
            ((self.is_wsa_holographic_remoting_enabled).inspect_value(path, offset, limit))
                .map(|v| ("isWsaHolographicRemotingEnabled".to_string(), v)),
            ((self.vr_settings).inspect_value(path, offset, limit))
                .map(|v| ("vrSettings".to_string(), v)),
            ((self.metal_use_metal_display_link).inspect_value(path, offset, limit))
                .map(|v| ("metalUseMetalDisplayLink".to_string(), v)),
            ((self.vulkan_enable_late_acquire_next_image).inspect_value(path, offset, limit))
                .map(|v| ("vulkanEnableLateAcquireNextImage".to_string(), v)),
            ((self.use_hdr_display).inspect_value(path, offset, limit))
                .map(|v| ("useHDRDisplay".to_string(), v)),
            ((self.enable_frame_timing_stats).inspect_value(path, offset, limit))
                .map(|v| ("enableFrameTimingStats".to_string(), v)),
            ((self.m_color_gamuts).inspect_value(path, offset, limit))
                .map(|v| ("m_ColorGamuts".to_string(), v)),
            ((self.target_pixel_density).inspect_value(path, offset, limit))
                .map(|v| ("targetPixelDensity".to_string(), v)),
            ((self.vulkan_enable_pre_transform).inspect_value(path, offset, limit))
                .map(|v| ("vulkanEnablePreTransform".to_string(), v)),
            ((self.switch_gpu_scratch_pool_granularity).inspect_value(path, offset, limit))
                .map(|v| ("switchGpuScratchPoolGranularity".to_string(), v)),
            ((self.resolution_scaling_mode).inspect_value(path, offset, limit))
                .map(|v| ("resolutionScalingMode".to_string(), v)),
            ((self.switch_allow_gpu_scratch_shrinking).inspect_value(path, offset, limit))
                .map(|v| ("switchAllowGpuScratchShrinking".to_string(), v)),
            ((self.enable_new_input_system).inspect_value(path, offset, limit))
                .map(|v| ("enableNewInputSystem".to_string(), v)),
            ((self.android_supported_aspect_ratio).inspect_value(path, offset, limit))
                .map(|v| ("androidSupportedAspectRatio".to_string(), v)),
            ((self.vulkan_enable_command_buffer_recycling).inspect_value(path, offset, limit))
                .map(|v| ("vulkanEnableCommandBufferRecycling".to_string(), v)),
            ((self.switch_max_worker_multiple).inspect_value(path, offset, limit))
                .map(|v| ("switchMaxWorkerMultiple".to_string(), v)),
            ((self.android_max_aspect_ratio).inspect_value(path, offset, limit))
                .map(|v| ("androidMaxAspectRatio".to_string(), v)),
            ((self.d3dhdr_bit_depth).inspect_value(path, offset, limit))
                .map(|v| ("D3DHDRBitDepth".to_string(), v)),
            ((self.switch_nvn_graphics_firmware_memory).inspect_value(path, offset, limit))
                .map(|v| ("switchNVNGraphicsFirmwareMemory".to_string(), v)),
            ((self.framebuffer_depth_memoryless_mode).inspect_value(path, offset, limit))
                .map(|v| ("framebufferDepthMemorylessMode".to_string(), v)),
            ((self.enable_native_platform_backends_for_new_input_system)
                .inspect_value(path, offset, limit))
            .map(|v| {
                (
                    "enableNativePlatformBackendsForNewInputSystem".to_string(),
                    v,
                )
            }),
            ((self.disable_old_input_manager_support).inspect_value(path, offset, limit))
                .map(|v| ("disableOldInputManagerSupport".to_string(), v)),
            ((self.legacy_clamp_blend_shape_weights).inspect_value(path, offset, limit))
                .map(|v| ("legacyClampBlendShapeWeights".to_string(), v)),
            ((self.load_store_debug_mode_enabled).inspect_value(path, offset, limit))
                .map(|v| ("loadStoreDebugModeEnabled".to_string(), v)),
            ((self.switch_graphics_jobs_sync_after_kick).inspect_value(path, offset, limit))
                .map(|v| ("switchGraphicsJobsSyncAfterKick".to_string(), v)),
            ((self.invalidated_pattern_texture).inspect_value(path, offset, limit))
                .map(|v| ("invalidatedPatternTexture".to_string(), v)),
            ((self.active_input_handler).inspect_value(path, offset, limit))
                .map(|v| ("activeInputHandler".to_string(), v)),
            ((self.enable_open_gl_profiler_gpu_recorders).inspect_value(path, offset, limit))
                .map(|v| ("enableOpenGLProfilerGPURecorders".to_string(), v)),
            ((self.vision_os_bundle_version).inspect_value(path, offset, limit))
                .map(|v| ("visionOSBundleVersion".to_string(), v)),
            ((self.tv_os_bundle_version).inspect_value(path, offset, limit))
                .map(|v| ("tvOSBundleVersion".to_string(), v)),
            ((self.virtual_texturing_support_enabled).inspect_value(path, offset, limit))
                .map(|v| ("virtualTexturingSupportEnabled".to_string(), v)),
            ((self.quality_settings_names).inspect_value(path, offset, limit))
                .map(|v| ("qualitySettingsNames".to_string(), v)),
            ((self.reset_resolution_on_window_resize).inspect_value(path, offset, limit))
                .map(|v| ("resetResolutionOnWindowResize".to_string(), v)),
            ((self.hdr_bit_depth).inspect_value(path, offset, limit))
                .map(|v| ("hdrBitDepth".to_string(), v)),
            ((self.windows_gamepad_backend_hint).inspect_value(path, offset, limit))
                .map(|v| ("windowsGamepadBackendHint".to_string(), v)),
            ((self.allow_hdr_display_support).inspect_value(path, offset, limit))
                .map(|v| ("allowHDRDisplaySupport".to_string(), v)),
            ((self.player_data_path).inspect_value(path, offset, limit))
                .map(|v| ("playerDataPath".to_string(), v)),
            ((self.force_srgb_blit).inspect_value(path, offset, limit))
                .map(|v| ("forceSRGBBlit".to_string(), v)),
            ((self.upload_cleared_texture_data_after_creation_from_script)
                .inspect_value(path, offset, limit))
            .map(|v| {
                (
                    "uploadClearedTextureDataAfterCreationFromScript".to_string(),
                    v,
                )
            }),
            ((self.android_min_aspect_ratio).inspect_value(path, offset, limit))
                .map(|v| ("androidMinAspectRatio".to_string(), v)),
            ((self.insecure_http_option).inspect_value(path, offset, limit))
                .map(|v| ("insecureHttpOption".to_string(), v)),
            ((self.hmi_loading_image).inspect_value(path, offset, limit))
                .map(|v| ("hmiLoadingImage".to_string(), v)),
            ((self.platform_requires_readable_assets).inspect_value(path, offset, limit))
                .map(|v| ("platformRequiresReadableAssets".to_string(), v)),
            ((self.enable_direct_storage).inspect_value(path, offset, limit))
                .map(|v| ("enableDirectStorage".to_string(), v)),
            ((self.android_vulkan_deny_filter_list).inspect_value(path, offset, limit))
                .map(|v| ("androidVulkanDenyFilterList".to_string(), v)),
            ((self.android_vulkan_allow_filter_list).inspect_value(path, offset, limit))
                .map(|v| ("androidVulkanAllowFilterList".to_string(), v)),
            ((self.android_vulkan_device_filter_list_asset).inspect_value(path, offset, limit))
                .map(|v| ("androidVulkanDeviceFilterListAsset".to_string(), v)),
            ((self.d3d12_device_filter_list_asset).inspect_value(path, offset, limit))
                .map(|v| ("d3d12DeviceFilterListAsset".to_string(), v)),
            ((self.allowed_http_connections).inspect_value(path, offset, limit))
                .map(|v| ("allowedHttpConnections".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
