//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::android_device_filter_data::AndroidDeviceFilterData;
use crate::subclasses::aspect_ratios::AspectRatios;
use crate::subclasses::color_rgba::ColorRgba;
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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
#[derive(Debug, Clone)]
pub struct PlayerSettings {
    pub android_profiler: PlayerSettingsAndroidProfilerVariant,
    pub product_guid: Option<Guid>,
    pub default_screen_orientation: i32,
    pub target_device: i32,
    pub android_filter_touches_when_obscured: Option<PlayerSettingsAndroidFilterTouchesWhenObscuredVariant>,
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
    pub m_splash_screen_background_color: Option<ColorRgba>,
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
    pub m_splash_screen_logos: Option<Vec<SplashScreenLogo>>,
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
    pub disable_depth_and_stencil_buffers: Option<PlayerSettingsDisableDepthAndStencilBuffersVariant>,
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
    pub wii_loading_screen_background: Option<ColorRgba>,
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
    pub force_ios_speakers_when_recording: Option<PlayerSettingsForceIosSpeakersWhenRecordingVariant>,
    pub android_default_window_width: Option<i32>,
    pub metro_enable_low_latency_presentation_api: Option<PlayerSettingsMetroEnableLowLatencyPresentationApiVariant>,
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
    pub xbox_one_disable_kinect_gpu_reservation: Option<PlayerSettingsXboxOneDisableKinectGpuReservationVariant>,
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
    pub switch_nvn_other_pools_granularity: Option<PlayerSettingsSwitchNvnOtherPoolsGranularityVariant>,
    pub metro_input_source: Option<i32>,
    pub vulkan_num_swapchain_buffers: Option<u32>,
    pub wsa_transparent_swapchain: Option<bool>,
    pub vulkan_use_sw_command_buffers: Option<bool>,
    pub switch_nvn_max_public_texture_id_count: Option<i32>,
    pub stadia_present_mode: Option<i32>,
    pub m_holographic_pause_on_tracking_loss: Option<bool>,
    pub switch_nvn_max_public_sampler_id_count: Option<PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant>,
    pub stadia_target_framerate: Option<i32>,
    pub xbox_one_enable7th_core: Option<bool>,
    pub is_wsa_holographic_remoting_enabled: Option<bool>,
    pub vr_settings: Option<VrSettings>,
    pub metal_use_metal_display_link: Option<bool>,
    pub vulkan_enable_late_acquire_next_image: Option<PlayerSettingsVulkanEnableLateAcquireNextImageVariant>,
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
    pub switch_nvn_graphics_firmware_memory: Option<PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant>,
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
    pub virtual_texturing_support_enabled: Option<PlayerSettingsVirtualTexturingSupportEnabledVariant>,
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
            android_filter_touches_when_obscured = Some(PlayerSettingsAndroidFilterTouchesWhenObscuredVariant::parse(rdr, ver)?);
            android_enable_sustained_performance_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            android_filter_touches_when_obscured = Some(PlayerSettingsAndroidFilterTouchesWhenObscuredVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            product_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_splash_screen_background_color = Some(ColorRgba::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(rdr, ver)?);
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
            m_splash_screen_logos = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SplashScreenLogo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_splash_screen_background_landscape = Some(crate::PPtr::parse(rdr, ver)?);
            m_splash_screen_background_portrait = Some(crate::PPtr::parse(rdr, ver)?);
            m_unity_splash_logo = Some(crate::PPtr::parse(rdr, ver)?);
            m_virtual_reality_splash_screen = Some(PlayerSettingsMVirtualRealitySplashScreenVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_splash_screen_background_color = Some(ColorRgba::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(rdr, ver)?);
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
            m_splash_screen_logos = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SplashScreenLogo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_splash_screen_background_landscape = Some(crate::PPtr::parse(rdr, ver)?);
            m_splash_screen_background_portrait = Some(crate::PPtr::parse(rdr, ver)?);
            m_unity_splash_logo = Some(crate::PPtr::parse(rdr, ver)?);
            m_virtual_reality_splash_screen = Some(PlayerSettingsMVirtualRealitySplashScreenVariant::parse(rdr, ver)?);
            m_holographic_tracking_loss_screen = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_splash_screen_background_color = Some(ColorRgba::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(rdr, ver)?);
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
            m_splash_screen_logos = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SplashScreenLogo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_splash_screen_background_landscape = Some(crate::PPtr::parse(rdr, ver)?);
            m_splash_screen_background_portrait = Some(crate::PPtr::parse(rdr, ver)?);
            m_virtual_reality_splash_screen = Some(PlayerSettingsMVirtualRealitySplashScreenVariant::parse(rdr, ver)?);
            m_holographic_tracking_loss_screen = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_splash_screen_style = Some(rdr.read_i32()?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(rdr, ver)?);
            m_virtual_reality_splash_screen = Some(PlayerSettingsMVirtualRealitySplashScreenVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(rdr, ver)?);
            m_virtual_reality_splash_screen = Some(PlayerSettingsMVirtualRealitySplashScreenVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 1) {
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
            m_show_unity_splash_screen = Some(PlayerSettingsMShowUnitySplashScreenVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            default_cursor = Some(crate::PPtr::parse(rdr, ver)?);
            cursor_hotspot = Some(Vector2f::parse(rdr, ver)?);
        } else {
            android_license_public_key = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            number_of_mips_stripped_per_mipmap_limit_group = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            number_of_mips_stripped_per_mipmap_limit_group = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_sprite_batch_vertex_threshold = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            number_of_mips_stripped_per_mipmap_limit_group = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            number_of_mips_stripped_per_mipmap_limit_group = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 3) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_sprite_batch_vertex_threshold = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
            aid = Some(Hash128::parse(rdr, ver)?);
            rdr.align(4)?;
            player_min_open_gles_version = Some(rdr.read_i32()?);
            mip_stripping = Some(rdr.read_bool()?);
            rdr.align(4)?;
            number_of_mips_stripped = Some(rdr.read_i32()?);
            number_of_mips_stripped_per_mipmap_limit_group = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_sprite_batch_vertex_threshold = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
            m_stack_trace_types = Some(PlayerSettingsMStackTraceTypesVariant::parse(rdr, ver)?);
            ios_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            android_show_activity_indicator_on_loading = Some(rdr.read_i32()?);
            ios_use_custom_app_background_behavior = Some(rdr.read_bool()?);
            ios_allow_http_download = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_stereo_rendering_path = Some(rdr.read_i32()?);
            m_active_color_space = Some(rdr.read_i32()?);
            m_mt_rendering = Some(PlayerSettingsMMtRenderingVariant::parse(rdr, ver)?);
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            mobile_mt_rendering_baked = Some(PlayerSettingsMobileMtRenderingBakedVariant::parse(rdr, ver)?);
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
            use32_bit_display_buffer = Some(PlayerSettingsUse32BitDisplayBufferVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            adjust_iosfps_using_thermal_state = Some(rdr.read_bool()?);
            rdr.align(4)?;
            thermal_state_serious_iosfps = Some(rdr.read_i32()?);
            thermal_state_critical_iosfps = Some(rdr.read_i32()?);
            rdr.align(4)?;
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            android_predictive_back_support = Some(PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?);
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 2, 8) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            android_predictive_back_support = Some(PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?);
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            android_predictive_back_support = Some(PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?);
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 60) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            android_predictive_back_support = Some(PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?);
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 5) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            android_predictive_back_support = Some(PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?);
            android_application_entry = Some(rdr.read_i32()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 4) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            android_predictive_back_support = Some(PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            android_predictive_back_support = Some(PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 5) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            android_predictive_back_support = Some(PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 61) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            android_predictive_back_support = Some(PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 17) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            android_predictive_back_support = Some(PlayerSettingsAndroidPredictiveBackSupportVariant::parse(rdr, ver)?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 35) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 17) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 4, 31) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
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
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
            android_use_swappy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_start_in_fullscreen = Some(rdr.read_bool()?);
            android_render_outside_safe_area = Some(PlayerSettingsAndroidRenderOutsideSafeAreaVariant::parse(rdr, ver)?);
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            preserve_framebuffer_alpha = Some(rdr.read_bool()?);
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_full_screen = Some(rdr.read_bool()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
            android_blit_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
            default_is_full_screen = Some(rdr.read_bool()?);
            default_is_native_resolution = Some(rdr.read_bool()?);
            mac_retina_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            disable_depth_and_stencil_buffers = Some(PlayerSettingsDisableDepthAndStencilBuffersVariant::parse(rdr, ver)?);
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
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            force_ios_speakers_when_recording = Some(PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?);
            audio_spatial_experience = Some(rdr.read_i32()?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 2, 5) {
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            force_ios_speakers_when_recording = Some(PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?);
            audio_spatial_experience = Some(rdr.read_i32()?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            force_ios_speakers_when_recording = Some(PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 58) {
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            force_ios_speakers_when_recording = Some(PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?);
            audio_spatial_experience = Some(rdr.read_i32()?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 15) {
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            force_ios_speakers_when_recording = Some(PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            force_ios_speakers_when_recording = Some(PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 49) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            force_ios_speakers_when_recording = Some(PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?);
            audio_spatial_experience = Some(rdr.read_i32()?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 2) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            force_ios_speakers_when_recording = Some(PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            force_ios_speakers_when_recording = Some(PlayerSettingsForceIosSpeakersWhenRecordingVariant::parse(rdr, ver)?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 5) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            defer_system_gestures_mode = Some(rdr.read_i32()?);
            hide_home_button = Some(rdr.read_bool()?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 3) {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 8) {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
            mute_other_audio_sources = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
            submit_analytics = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            capture_single_screen = Some(rdr.read_bool()?);
            override_i_pod_music = Some(rdr.read_bool()?);
            prepare_ios_for_recording = Some(PlayerSettingsPrepareIosForRecordingVariant::parse(rdr, ver)?);
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
            use_mac_app_store_validation = Some(PlayerSettingsUseMacAppStoreValidationVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 4, 1) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            enable_direct_storage = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
            d3d12_device_filter_list_asset = Some(PlayerSettingsD3d12DeviceFilterListAssetVariant::parse(rdr, ver)?);
            allowed_http_connections = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
            d3d12_device_filter_list_asset = Some(PlayerSettingsD3d12DeviceFilterListAssetVariant::parse(rdr, ver)?);
            allowed_http_connections = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 3, 11) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
            d3d12_device_filter_list_asset = Some(PlayerSettingsD3d12DeviceFilterListAssetVariant::parse(rdr, ver)?);
            allowed_http_connections = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
            d3d12_device_filter_list_asset = Some(PlayerSettingsD3d12DeviceFilterListAssetVariant::parse(rdr, ver)?);
            allowed_http_connections = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 2, 7) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
            d3d12_device_filter_list_asset = Some(PlayerSettingsD3d12DeviceFilterListAssetVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            switch_graphics_jobs_sync_after_kick = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_device_filter_list_asset = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 5) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
            android_vulkan_deny_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            android_vulkan_allow_filter_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            android_min_aspect_ratio = Some(rdr.read_f32()?);
            rdr.align(4)?;
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 12) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 10) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            mesh_deformation = Some(rdr.read_i32()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 18) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            vision_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            tv_os_bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 11) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            allow_hdr_display_support = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 8) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 19) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            platform_requires_readable_assets = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 15) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            hdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 3) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            invalidated_pattern_texture = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            load_store_debug_mode_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            hmi_loading_image = Some(crate::PPtr::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 2) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            player_data_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_gpu_scratch_pool_granularity = Some(rdr.read_i32()?);
            switch_allow_gpu_scratch_shrinking = Some(rdr.read_bool()?);
            rdr.align(4)?;
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_nvn_graphics_firmware_memory = Some(PlayerSettingsSwitchNvnGraphicsFirmwareMemoryVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            player_data_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            insecure_http_option = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 30) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            switch_max_worker_multiple = Some(rdr.read_i32()?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            player_data_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 13) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            player_data_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 3) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            player_data_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 2) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            enable_open_gl_profiler_gpu_recorders = Some(rdr.read_bool()?);
            rdr.align(4)?;
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            player_data_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 4) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            player_data_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            player_data_path = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            force_srgb_blit = Some(rdr.read_bool()?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
            upload_cleared_texture_data_after_creation_from_script = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 14) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 42) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            windows_gamepad_backend_hint = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 35) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            reset_resolution_on_window_resize = Some(rdr.read_bool()?);
            rdr.align(4)?;
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 18) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            vulkan_enable_command_buffer_recycling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_pre_transform = Some(rdr.read_bool()?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            active_input_handler = Some(rdr.read_i32()?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            quality_settings_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 7) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 1) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
            virtual_texturing_support_enabled = Some(PlayerSettingsVirtualTexturingSupportEnabledVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 4, 6) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_enable_late_acquire_next_image = Some(PlayerSettingsVulkanEnableLateAcquireNextImageVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 15) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            stadia_present_mode = Some(rdr.read_i32()?);
            stadia_target_framerate = Some(rdr.read_i32()?);
            rdr.align(4)?;
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 2) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            vulkan_num_swapchain_buffers = Some(rdr.read_u32()?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            d3dhdr_bit_depth = Some(rdr.read_i32()?);
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 26) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            switch_nvn_max_public_texture_id_count = Some(rdr.read_i32()?);
            switch_nvn_max_public_sampler_id_count = Some(PlayerSettingsSwitchNvnMaxPublicSamplerIdCountVariant::parse(rdr, ver)?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 17) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 5) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            switch_queue_control_memory = Some(rdr.read_i32()?);
            switch_queue_compute_memory = Some(rdr.read_i32()?);
            switch_nvn_shader_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_default_pools_granularity = Some(rdr.read_i32()?);
            switch_nvn_other_pools_granularity = Some(PlayerSettingsSwitchNvnOtherPoolsGranularityVariant::parse(rdr, ver)?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            xbox_one_s_resolution = Some(rdr.read_i32()?);
            xbox_one_x_resolution = Some(rdr.read_i32()?);
            xbox_one_mono_logging_level = Some(rdr.read_i32()?);
            xbox_one_logging_level = Some(rdr.read_i32()?);
            xbox_one_disable_esram = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_present_immediate_threshold = Some(rdr.read_u32()?);
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            enable_frame_timing_stats = Some(PlayerSettingsEnableFrameTimingStatsVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            framebuffer_depth_memoryless_mode = Some(rdr.read_i32()?);
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
            legacy_clamp_blend_shape_weights = Some(PlayerSettingsLegacyClampBlendShapeWeightsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 9) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_use_sw_command_buffers = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            is_wsa_holographic_remoting_enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vulkan_enable_set_srgb_write = Some(PlayerSettingsVulkanEnableSetSrgbWriteVariant::parse(rdr, ver)?);
            vulkan_use_sw_command_buffers = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            switch_queue_command_memory = Some(PlayerSettingsSwitchQueueCommandMemoryVariant::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            wsa_transparent_swapchain = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_enable_pix_sampling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            metal_framebuffer_only = Some(PlayerSettingsMetalFramebufferOnlyVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_color_gamuts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            android_supported_aspect_ratio = Some(rdr.read_i32()?);
            android_max_aspect_ratio = Some(PlayerSettingsAndroidMaxAspectRatioVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 2) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            target_pixel_density = Some(rdr.read_i32()?);
            resolution_scaling_mode = Some(PlayerSettingsResolutionScalingModeVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_native_platform_backends_for_new_input_system = Some(rdr.read_bool()?);
            disable_old_input_manager_support = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 5) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
            enable_new_input_system = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 1) {
            mac_app_store_category = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
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
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            xbox_one_enable7th_core = Some(rdr.read_bool()?);
            rdr.align(4)?;
            vr_settings = Some(VrSettings::parse(rdr, ver)?);
            rdr.align(4)?;
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
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
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 4) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            use_hdr_display = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 2) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            graphics_job_mode = Some(rdr.read_i32()?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_input_source = Some(rdr.read_i32()?);
            m_holographic_pause_on_tracking_loss = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 1) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            single_pass_stereo_rendering = Some(rdr.read_bool()?);
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            graphics_jobs = Some(rdr.read_bool()?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            single_pass_stereo_rendering = Some(rdr.read_bool()?);
            protect_graphics_memory = Some(PlayerSettingsProtectGraphicsMemoryVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 7) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 6) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 1) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            allow_fullscreen_switch = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
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
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 1) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            virtual_reality_supported = Some(rdr.read_bool()?);
            rdr.align(4)?;
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
            cloud_project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            project_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            organization_id = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            cloud_enabled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_fullscreen_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_one_resolution = Some(PlayerSettingsXboxOneResolutionVariant::parse(rdr, ver)?);
            ps3_splash_screen = Some(crate::PPtr::parse(rdr, ver)?);
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            psp2_power_mode = Some(rdr.read_i32()?);
            psp2_acquire_bgm = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            bundle_version = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            preloaded_assets = Some(PlayerSettingsPreloadedAssetsVariant::parse(rdr, ver)?);
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
            xbox_one_disable_kinect_gpu_reservation = Some(PlayerSettingsXboxOneDisableKinectGpuReservationVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 5) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            d3d11_force_exclusive_mode = Some(rdr.read_bool()?);
            rdr.align(4)?;
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 4) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            visible_in_background = Some(PlayerSettingsVisibleInBackgroundVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            d3d9_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
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
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            video_memory_for_vertex_buffers = Some(rdr.read_i32()?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 3) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgba::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            metro_enable_independent_input_source = Some(rdr.read_bool()?);
            metro_enable_low_latency_presentation_api = Some(PlayerSettingsMetroEnableLowLatencyPresentationApiVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_pix_texture_capture = Some(rdr.read_bool()?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            xbox_enable_guest = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgba::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            gpu_skinning = Some(PlayerSettingsGpuSkinningVariant::parse(rdr, ver)?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgba::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 1, 2) {
            xbox_skin_on_gpu = Some(PlayerSettingsXboxSkinOnGpuVariant::parse(rdr, ver)?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgba::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
            i_phone_bundle_identifier = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            xbox_skin_on_gpu = Some(PlayerSettingsXboxSkinOnGpuVariant::parse(rdr, ver)?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_fitness = Some(PlayerSettingsXboxEnableFitnessVariant::parse(rdr, ver)?);
            mac_fullscreen_mode = Some(rdr.read_i32()?);
            xbox_speech_db = Some(rdr.read_u32()?);
            xbox_enable_head_orientation = Some(PlayerSettingsXboxEnableHeadOrientationVariant::parse(rdr, ver)?);
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgba::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
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
            wii_loading_screen_background = Some(ColorRgba::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 5) {
            xbox_skin_on_gpu = Some(PlayerSettingsXboxSkinOnGpuVariant::parse(rdr, ver)?);
            xbox_enable_avatar = Some(rdr.read_bool()?);
            xbox_enable_kinect = Some(rdr.read_bool()?);
            xbox_enable_kinect_auto_tracking = Some(rdr.read_bool()?);
            xbox_enable_speech = Some(rdr.read_bool()?);
            rdr.align(4)?;
            wii_hio2_usage = Some(rdr.read_i32()?);
            wii_loading_screen_rect_placement = Some(rdr.read_i32()?);
            wii_loading_screen_background = Some(ColorRgba::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
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
            wii_loading_screen_background = Some(ColorRgba::parse(rdr, ver)?);
            wii_loading_screen_period = Some(rdr.read_i32()?);
            wii_loading_screen_file_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            wii_loading_screen_rect = Some(Rectf::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
        } else {
            xbox_skin_on_gpu = Some(PlayerSettingsXboxSkinOnGpuVariant::parse(rdr, ver)?);
            m_supported_aspect_ratios = Some(AspectRatios::parse(rdr, ver)?);
            rdr.align(4)?;
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
            allowed_autorotate_to_portrait_upside_down: allowed_autorotate_to_portrait_upside_down.unwrap(),
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
