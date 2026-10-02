//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::aabb::Aabb;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::vector4f::Vector4f;

/// 版本枚举：SkinnedMeshRendererMEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMEnabledVariant {
    Default(bool),
    V5_0(bool),
    V5_4(bool),
}

impl crate::Parse for SkinnedMeshRendererMEnabledVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMEnabledVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：SkinnedMeshRendererMCastShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMCastShadowsVariant {
    Default(bool),
    V5_0(u8),
}

impl crate::Parse for SkinnedMeshRendererMCastShadowsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMCastShadowsVariant {
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

/// 版本枚举：SkinnedMeshRendererMReceiveShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMReceiveShadowsVariant {
    Default(bool),
    V5_0(bool),
    V5_4(u8),
}

impl crate::Parse for SkinnedMeshRendererMReceiveShadowsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V5_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMReceiveShadowsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：SkinnedMeshRendererMLightmapIndexVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMLightmapIndexVariant {
    Default(u8),
    V5_0(u16),
}

impl crate::Parse for SkinnedMeshRendererMLightmapIndexVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_u16()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_u8()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMLightmapIndexVariant {
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

/// 版本枚举：SkinnedMeshRendererMMaterialsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMMaterialsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for SkinnedMeshRendererMMaterialsVariant {
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

impl crate::ToValue for SkinnedMeshRendererMMaterialsVariant {
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

/// 版本枚举：SkinnedMeshRendererMReflectionProbeUsageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMReflectionProbeUsageVariant {
    Default(i32),
    V5_4(u8),
    V2019_3(u8),
}

impl crate::Parse for SkinnedMeshRendererMReflectionProbeUsageVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMReflectionProbeUsageVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：SkinnedMeshRendererMRayTracingModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMRayTracingModeVariant {
    Default(u8),
    V2020_1(u8),
}

impl crate::Parse for SkinnedMeshRendererMRayTracingModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMRayTracingModeVariant {
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

/// 版本枚举：SkinnedMeshRendererMRayTraceProceduralVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMRayTraceProceduralVariant {
    Default(u8),
    V2023_2(u8),
}

impl crate::Parse for SkinnedMeshRendererMRayTraceProceduralVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2023_2(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMRayTraceProceduralVariant {
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

/// 版本枚举：SkinnedMeshRendererMUpdateWhenOffscreenVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMUpdateWhenOffscreenVariant {
    Default(bool),
    V5_4(bool),
}

impl crate::Parse for SkinnedMeshRendererMUpdateWhenOffscreenVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMUpdateWhenOffscreenVariant {
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

/// 版本枚举：SkinnedMeshRendererMLightProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMLightProbeAnchorVariant {
    Default(PPtr),
    V4_3(PPtr),
}

impl crate::Parse for SkinnedMeshRendererMLightProbeAnchorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::V4_3(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMLightProbeAnchorVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：SkinnedMeshRendererMSortingLayerIdVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMSortingLayerIdVariant {
    Default(u32),
    V5_0(i32),
}

impl crate::Parse for SkinnedMeshRendererMSortingLayerIdVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMSortingLayerIdVariant {
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

/// 版本枚举：SkinnedMeshRendererMBonesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMBonesVariant {
    Default(Vec<PPtr>),
    V4_3(Vec<PPtr>),
}

impl crate::Parse for SkinnedMeshRendererMBonesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V4_3(v))
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

impl crate::ToValue for SkinnedMeshRendererMBonesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：SkinnedMeshRendererMSortingOrderVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMSortingOrderVariant {
    Default(i16),
    V4_5(i16),
}

impl crate::Parse for SkinnedMeshRendererMSortingOrderVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = rdr.read_i16()?;
            rdr.align(4)?;
            Ok(Self::V4_5(v))
        } else {
            let v = rdr.read_i16()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMSortingOrderVariant {
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

/// 版本枚举：SkinnedMeshRendererMRayTracingAccelStructBuildFlagsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMRayTracingAccelStructBuildFlagsVariant {
    Default(u8),
    V2023_3(u8),
}

impl crate::Parse for SkinnedMeshRendererMRayTracingAccelStructBuildFlagsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2023_3(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMRayTracingAccelStructBuildFlagsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：SkinnedMeshRendererMProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMProbeAnchorVariant {
    Default(PPtr),
    V5_4(PPtr),
}

impl crate::Parse for SkinnedMeshRendererMProbeAnchorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V5_4(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMProbeAnchorVariant {
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

/// 版本枚举：SkinnedMeshRendererMDirtyAabbVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMDirtyAabbVariant {
    Default(bool),
    V4_1(bool),
}

impl crate::Parse for SkinnedMeshRendererMDirtyAabbVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V4_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMDirtyAabbVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：SkinnedMeshRendererMBlendShapeWeightsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SkinnedMeshRendererMBlendShapeWeightsVariant {
    Default(Vec<f32>),
    V2017_1(Vec<f32>),
}

impl crate::Parse for SkinnedMeshRendererMBlendShapeWeightsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SkinnedMeshRendererMBlendShapeWeightsVariant {
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
#[derive(Debug, Clone)]
pub struct SkinnedMeshRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: SkinnedMeshRendererMEnabledVariant,
    pub m_cast_shadows: SkinnedMeshRendererMCastShadowsVariant,
    pub m_receive_shadows: SkinnedMeshRendererMReceiveShadowsVariant,
    pub m_lightmap_index: SkinnedMeshRendererMLightmapIndexVariant,
    pub m_motion_vectors: Option<u8>,
    pub m_dynamic_occludee: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_index_dynamic: Option<u16>,
    pub m_light_probe_usage: Option<u8>,
    pub m_static_shadow_caster: Option<u8>,
    pub m_materials: SkinnedMeshRendererMMaterialsVariant,
    pub m_reflection_probe_usage: Option<SkinnedMeshRendererMReflectionProbeUsageVariant>,
    pub m_subset_indices: Option<Vec<u32>>,
    pub m_lightmap_tiling_offset_dynamic: Option<Vector4f>,
    pub m_static_batch_root: PPtr,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_ray_tracing_mode: Option<SkinnedMeshRendererMRayTracingModeVariant>,
    pub m_quality: i32,
    pub m_use_light_probes: Option<bool>,
    pub m_renderer_priority: Option<i32>,
    pub m_ray_trace_procedural: Option<SkinnedMeshRendererMRayTraceProceduralVariant>,
    pub m_update_when_offscreen: SkinnedMeshRendererMUpdateWhenOffscreenVariant,
    pub m_light_probe_anchor: Option<SkinnedMeshRendererMLightProbeAnchorVariant>,
    pub m_mesh: PPtr,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<SkinnedMeshRendererMSortingLayerIdVariant>,
    pub m_ray_tracing_accel_struct_build_flags_override: Option<u8>,
    pub m_bones: SkinnedMeshRendererMBonesVariant,
    pub m_sorting_order: Option<SkinnedMeshRendererMSortingOrderVariant>,
    pub m_static_batch_info: Option<StaticBatchInfo>,
    pub m_ray_tracing_accel_struct_build_flags:
        Option<SkinnedMeshRendererMRayTracingAccelStructBuildFlagsVariant>,
    pub m_aabb: Aabb,
    pub m_probe_anchor: Option<SkinnedMeshRendererMProbeAnchorVariant>,
    pub m_small_mesh_culling: Option<u8>,
    pub m_dirty_aabb: SkinnedMeshRendererMDirtyAabbVariant,
    pub m_force_mesh_lod: Option<i16>,
    pub m_root_bone: Option<PPtr>,
    pub m_light_probe_volume_override: Option<PPtr>,
    pub m_mesh_lod_selection_bias: Option<f32>,
    pub m_blend_shape_weights: Option<SkinnedMeshRendererMBlendShapeWeightsVariant>,
    pub m_skinned_motion_vectors: Option<bool>,
    pub m_mask_interaction: Option<i32>,
}

impl crate::Parse for SkinnedMeshRenderer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_cast_shadows = None;
        let mut m_receive_shadows = None;
        let mut m_lightmap_index = None;
        let mut m_motion_vectors = None;
        let mut m_dynamic_occludee = None;
        let mut m_lightmap_tiling_offset = None;
        let mut m_lightmap_index_dynamic = None;
        let mut m_light_probe_usage = None;
        let mut m_static_shadow_caster = None;
        let mut m_materials = None;
        let mut m_reflection_probe_usage = None;
        let mut m_subset_indices = None;
        let mut m_lightmap_tiling_offset_dynamic = None;
        let mut m_static_batch_root = None;
        let mut m_rendering_layer_mask = None;
        let mut m_ray_tracing_mode = None;
        let mut m_quality = None;
        let mut m_use_light_probes = None;
        let mut m_renderer_priority = None;
        let mut m_ray_trace_procedural = None;
        let mut m_update_when_offscreen = None;
        let mut m_light_probe_anchor = None;
        let mut m_mesh = None;
        let mut m_sorting_layer = None;
        let mut m_sorting_layer_id = None;
        let mut m_ray_tracing_accel_struct_build_flags_override = None;
        let mut m_bones = None;
        let mut m_sorting_order = None;
        let mut m_static_batch_info = None;
        let mut m_ray_tracing_accel_struct_build_flags = None;
        let mut m_aabb = None;
        let mut m_probe_anchor = None;
        let mut m_small_mesh_culling = None;
        let mut m_dirty_aabb = None;
        let mut m_force_mesh_lod = None;
        let mut m_root_bone = None;
        let mut m_light_probe_volume_override = None;
        let mut m_mesh_lod_selection_bias = None;
        let mut m_blend_shape_weights = None;
        let mut m_skinned_motion_vectors = None;
        let mut m_mask_interaction = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(SkinnedMeshRendererMEnabledVariant::parse(rdr, ver)?);
        m_cast_shadows = Some(SkinnedMeshRendererMCastShadowsVariant::parse(rdr, ver)?);
        m_receive_shadows = Some(SkinnedMeshRendererMReceiveShadowsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(
                SkinnedMeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?,
            );
            m_ray_tracing_mode = Some(SkinnedMeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(SkinnedMeshRendererMRayTraceProceduralVariant::parse(
                rdr, ver,
            )?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags =
                Some(SkinnedMeshRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
            m_small_mesh_culling = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_force_mesh_lod = Some(rdr.read_i16()?);
            rdr.align(4)?;
            m_mesh_lod_selection_bias = Some(rdr.read_f32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(
                SkinnedMeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?,
            );
            m_ray_tracing_mode = Some(SkinnedMeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(SkinnedMeshRendererMRayTraceProceduralVariant::parse(
                rdr, ver,
            )?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags =
                Some(SkinnedMeshRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
            m_small_mesh_culling = Some(rdr.read_u8()?);
            rdr.align(4)?;
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(
                SkinnedMeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?,
            );
            m_ray_tracing_mode = Some(SkinnedMeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(SkinnedMeshRendererMRayTraceProceduralVariant::parse(
                rdr, ver,
            )?);
            m_ray_tracing_accel_struct_build_flags_override = Some(rdr.read_u8()?);
            m_ray_tracing_accel_struct_build_flags =
                Some(SkinnedMeshRendererMRayTracingAccelStructBuildFlagsVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_static_shadow_caster = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(
                SkinnedMeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?,
            );
            m_ray_tracing_mode = Some(SkinnedMeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(SkinnedMeshRendererMRayTraceProceduralVariant::parse(
                rdr, ver,
            )?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(
                SkinnedMeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?,
            );
            m_ray_tracing_mode = Some(SkinnedMeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_ray_trace_procedural = Some(SkinnedMeshRendererMRayTraceProceduralVariant::parse(
                rdr, ver,
            )?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(
                SkinnedMeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?,
            );
            m_ray_tracing_mode = Some(SkinnedMeshRendererMRayTracingModeVariant::parse(rdr, ver)?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(
                SkinnedMeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?,
            );
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_renderer_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(
                SkinnedMeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?,
            );
            m_rendering_layer_mask = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(
                SkinnedMeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(
                SkinnedMeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?,
            );
        } else {
        }
        m_lightmap_index = Some(SkinnedMeshRendererMLightmapIndexVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_index_dynamic = Some(rdr.read_u16()?);
        } else {
        }
        m_lightmap_tiling_offset = Some(Vector4f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(rdr, ver)?);
        } else {
        }
        m_materials = Some(SkinnedMeshRendererMMaterialsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_static_batch_info = Some(StaticBatchInfo::parse(rdr, ver)?);
        } else {
            m_subset_indices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }
        m_static_batch_root = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_probe_anchor = Some(SkinnedMeshRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(SkinnedMeshRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(SkinnedMeshRendererMSortingOrderVariant::parse(rdr, ver)?);
            m_mask_interaction = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_probe_anchor = Some(SkinnedMeshRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(SkinnedMeshRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(SkinnedMeshRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_probe_anchor = Some(SkinnedMeshRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(SkinnedMeshRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(SkinnedMeshRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_reflection_probe_usage = Some(
                SkinnedMeshRendererMReflectionProbeUsageVariant::parse(rdr, ver)?,
            );
            m_probe_anchor = Some(SkinnedMeshRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(SkinnedMeshRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(SkinnedMeshRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(SkinnedMeshRendererMLightProbeAnchorVariant::parse(
                rdr, ver,
            )?);
            m_sorting_layer_id = Some(SkinnedMeshRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(SkinnedMeshRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(SkinnedMeshRendererMLightProbeAnchorVariant::parse(
                rdr, ver,
            )?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(SkinnedMeshRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(SkinnedMeshRendererMLightProbeAnchorVariant::parse(
                rdr, ver,
            )?);
        } else {
        }
        m_quality = Some(rdr.read_i32()?);
        m_update_when_offscreen = Some(SkinnedMeshRendererMUpdateWhenOffscreenVariant::parse(
            rdr, ver,
        )?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_skinned_motion_vectors = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_mesh = Some(crate::PPtr::parse(rdr, ver)?);
        m_bones = Some(SkinnedMeshRendererMBonesVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_blend_shape_weights = Some(SkinnedMeshRendererMBlendShapeWeightsVariant::parse(
                rdr, ver,
            )?);
            m_root_bone = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_root_bone = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
        m_aabb = Some(Aabb::parse(rdr, ver)?);
        m_dirty_aabb = Some(SkinnedMeshRendererMDirtyAabbVariant::parse(rdr, ver)?);

        Ok(SkinnedMeshRenderer {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_cast_shadows: m_cast_shadows.unwrap(),
            m_receive_shadows: m_receive_shadows.unwrap(),
            m_lightmap_index: m_lightmap_index.unwrap(),
            m_motion_vectors,
            m_dynamic_occludee,
            m_lightmap_tiling_offset: m_lightmap_tiling_offset.unwrap(),
            m_lightmap_index_dynamic,
            m_light_probe_usage,
            m_static_shadow_caster,
            m_materials: m_materials.unwrap(),
            m_reflection_probe_usage,
            m_subset_indices,
            m_lightmap_tiling_offset_dynamic,
            m_static_batch_root: m_static_batch_root.unwrap(),
            m_rendering_layer_mask,
            m_ray_tracing_mode,
            m_quality: m_quality.unwrap(),
            m_use_light_probes,
            m_renderer_priority,
            m_ray_trace_procedural,
            m_update_when_offscreen: m_update_when_offscreen.unwrap(),
            m_light_probe_anchor,
            m_mesh: m_mesh.unwrap(),
            m_sorting_layer,
            m_sorting_layer_id,
            m_ray_tracing_accel_struct_build_flags_override,
            m_bones: m_bones.unwrap(),
            m_sorting_order,
            m_static_batch_info,
            m_ray_tracing_accel_struct_build_flags,
            m_aabb: m_aabb.unwrap(),
            m_probe_anchor,
            m_small_mesh_culling,
            m_dirty_aabb: m_dirty_aabb.unwrap(),
            m_force_mesh_lod,
            m_root_bone,
            m_light_probe_volume_override,
            m_mesh_lod_selection_bias,
            m_blend_shape_weights,
            m_skinned_motion_vectors,
            m_mask_interaction,
        })
    }
}

impl crate::ToValue for SkinnedMeshRenderer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_CastShadows".to_string(), self.m_cast_shadows.to_value())),
            Some((
                "m_ReceiveShadows".to_string(),
                self.m_receive_shadows.to_value(),
            )),
            Some((
                "m_LightmapIndex".to_string(),
                self.m_lightmap_index.to_value(),
            )),
            crate::opt_field("m_MotionVectors", &self.m_motion_vectors),
            crate::opt_field("m_DynamicOccludee", &self.m_dynamic_occludee),
            Some((
                "m_LightmapTilingOffset".to_string(),
                self.m_lightmap_tiling_offset.to_value(),
            )),
            crate::opt_field("m_LightmapIndexDynamic", &self.m_lightmap_index_dynamic),
            crate::opt_field("m_LightProbeUsage", &self.m_light_probe_usage),
            crate::opt_field("m_StaticShadowCaster", &self.m_static_shadow_caster),
            Some(("m_Materials".to_string(), self.m_materials.to_value())),
            crate::opt_field("m_ReflectionProbeUsage", &self.m_reflection_probe_usage),
            crate::opt_field("m_SubsetIndices", &self.m_subset_indices),
            crate::opt_field(
                "m_LightmapTilingOffsetDynamic",
                &self.m_lightmap_tiling_offset_dynamic,
            ),
            Some((
                "m_StaticBatchRoot".to_string(),
                self.m_static_batch_root.to_value(),
            )),
            crate::opt_field("m_RenderingLayerMask", &self.m_rendering_layer_mask),
            crate::opt_field("m_RayTracingMode", &self.m_ray_tracing_mode),
            Some(("m_Quality".to_string(), self.m_quality.to_value())),
            crate::opt_field("m_UseLightProbes", &self.m_use_light_probes),
            crate::opt_field("m_RendererPriority", &self.m_renderer_priority),
            crate::opt_field("m_RayTraceProcedural", &self.m_ray_trace_procedural),
            Some((
                "m_UpdateWhenOffscreen".to_string(),
                self.m_update_when_offscreen.to_value(),
            )),
            crate::opt_field("m_LightProbeAnchor", &self.m_light_probe_anchor),
            Some(("m_Mesh".to_string(), self.m_mesh.to_value())),
            crate::opt_field("m_SortingLayer", &self.m_sorting_layer),
            crate::opt_field("m_SortingLayerID", &self.m_sorting_layer_id),
            crate::opt_field(
                "m_RayTracingAccelStructBuildFlagsOverride",
                &self.m_ray_tracing_accel_struct_build_flags_override,
            ),
            Some(("m_Bones".to_string(), self.m_bones.to_value())),
            crate::opt_field("m_SortingOrder", &self.m_sorting_order),
            crate::opt_field("m_StaticBatchInfo", &self.m_static_batch_info),
            crate::opt_field(
                "m_RayTracingAccelStructBuildFlags",
                &self.m_ray_tracing_accel_struct_build_flags,
            ),
            Some(("m_AABB".to_string(), self.m_aabb.to_value())),
            crate::opt_field("m_ProbeAnchor", &self.m_probe_anchor),
            crate::opt_field("m_SmallMeshCulling", &self.m_small_mesh_culling),
            Some(("m_DirtyAABB".to_string(), self.m_dirty_aabb.to_value())),
            crate::opt_field("m_ForceMeshLod", &self.m_force_mesh_lod),
            crate::opt_field("m_RootBone", &self.m_root_bone),
            crate::opt_field(
                "m_LightProbeVolumeOverride",
                &self.m_light_probe_volume_override,
            ),
            crate::opt_field("m_MeshLodSelectionBias", &self.m_mesh_lod_selection_bias),
            crate::opt_field("m_BlendShapeWeights", &self.m_blend_shape_weights),
            crate::opt_field("m_SkinnedMotionVectors", &self.m_skinned_motion_vectors),
            crate::opt_field("m_MaskInteraction", &self.m_mask_interaction),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_CastShadows" => (self.m_cast_shadows).inspect_value(path, offset, limit),
                "m_ReceiveShadows" => (self.m_receive_shadows).inspect_value(path, offset, limit),
                "m_LightmapIndex" => (self.m_lightmap_index).inspect_value(path, offset, limit),
                "m_MotionVectors" => (self.m_motion_vectors).inspect_value(path, offset, limit),
                "m_DynamicOccludee" => (self.m_dynamic_occludee).inspect_value(path, offset, limit),
                "m_LightmapTilingOffset" => {
                    (self.m_lightmap_tiling_offset).inspect_value(path, offset, limit)
                }
                "m_LightmapIndexDynamic" => {
                    (self.m_lightmap_index_dynamic).inspect_value(path, offset, limit)
                }
                "m_LightProbeUsage" => {
                    (self.m_light_probe_usage).inspect_value(path, offset, limit)
                }
                "m_StaticShadowCaster" => {
                    (self.m_static_shadow_caster).inspect_value(path, offset, limit)
                }
                "m_Materials" => (self.m_materials).inspect_value(path, offset, limit),
                "m_ReflectionProbeUsage" => {
                    (self.m_reflection_probe_usage).inspect_value(path, offset, limit)
                }
                "m_SubsetIndices" => (self.m_subset_indices).inspect_value(path, offset, limit),
                "m_LightmapTilingOffsetDynamic" => {
                    (self.m_lightmap_tiling_offset_dynamic).inspect_value(path, offset, limit)
                }
                "m_StaticBatchRoot" => {
                    (self.m_static_batch_root).inspect_value(path, offset, limit)
                }
                "m_RenderingLayerMask" => {
                    (self.m_rendering_layer_mask).inspect_value(path, offset, limit)
                }
                "m_RayTracingMode" => (self.m_ray_tracing_mode).inspect_value(path, offset, limit),
                "m_Quality" => (self.m_quality).inspect_value(path, offset, limit),
                "m_UseLightProbes" => (self.m_use_light_probes).inspect_value(path, offset, limit),
                "m_RendererPriority" => {
                    (self.m_renderer_priority).inspect_value(path, offset, limit)
                }
                "m_RayTraceProcedural" => {
                    (self.m_ray_trace_procedural).inspect_value(path, offset, limit)
                }
                "m_UpdateWhenOffscreen" => {
                    (self.m_update_when_offscreen).inspect_value(path, offset, limit)
                }
                "m_LightProbeAnchor" => {
                    (self.m_light_probe_anchor).inspect_value(path, offset, limit)
                }
                "m_Mesh" => (self.m_mesh).inspect_value(path, offset, limit),
                "m_SortingLayer" => (self.m_sorting_layer).inspect_value(path, offset, limit),
                "m_SortingLayerID" => (self.m_sorting_layer_id).inspect_value(path, offset, limit),
                "m_RayTracingAccelStructBuildFlagsOverride" => (self
                    .m_ray_tracing_accel_struct_build_flags_override)
                    .inspect_value(path, offset, limit),
                "m_Bones" => (self.m_bones).inspect_value(path, offset, limit),
                "m_SortingOrder" => (self.m_sorting_order).inspect_value(path, offset, limit),
                "m_StaticBatchInfo" => {
                    (self.m_static_batch_info).inspect_value(path, offset, limit)
                }
                "m_RayTracingAccelStructBuildFlags" => {
                    (self.m_ray_tracing_accel_struct_build_flags).inspect_value(path, offset, limit)
                }
                "m_AABB" => (self.m_aabb).inspect_value(path, offset, limit),
                "m_ProbeAnchor" => (self.m_probe_anchor).inspect_value(path, offset, limit),
                "m_SmallMeshCulling" => {
                    (self.m_small_mesh_culling).inspect_value(path, offset, limit)
                }
                "m_DirtyAABB" => (self.m_dirty_aabb).inspect_value(path, offset, limit),
                "m_ForceMeshLod" => (self.m_force_mesh_lod).inspect_value(path, offset, limit),
                "m_RootBone" => (self.m_root_bone).inspect_value(path, offset, limit),
                "m_LightProbeVolumeOverride" => {
                    (self.m_light_probe_volume_override).inspect_value(path, offset, limit)
                }
                "m_MeshLodSelectionBias" => {
                    (self.m_mesh_lod_selection_bias).inspect_value(path, offset, limit)
                }
                "m_BlendShapeWeights" => {
                    (self.m_blend_shape_weights).inspect_value(path, offset, limit)
                }
                "m_SkinnedMotionVectors" => {
                    (self.m_skinned_motion_vectors).inspect_value(path, offset, limit)
                }
                "m_MaskInteraction" => (self.m_mask_interaction).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_cast_shadows).inspect_value(path, offset, limit))
                .map(|v| ("m_CastShadows".to_string(), v)),
            ((self.m_receive_shadows).inspect_value(path, offset, limit))
                .map(|v| ("m_ReceiveShadows".to_string(), v)),
            ((self.m_lightmap_index).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapIndex".to_string(), v)),
            ((self.m_motion_vectors).inspect_value(path, offset, limit))
                .map(|v| ("m_MotionVectors".to_string(), v)),
            ((self.m_dynamic_occludee).inspect_value(path, offset, limit))
                .map(|v| ("m_DynamicOccludee".to_string(), v)),
            ((self.m_lightmap_tiling_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapTilingOffset".to_string(), v)),
            ((self.m_lightmap_index_dynamic).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapIndexDynamic".to_string(), v)),
            ((self.m_light_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeUsage".to_string(), v)),
            ((self.m_static_shadow_caster).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticShadowCaster".to_string(), v)),
            ((self.m_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_Materials".to_string(), v)),
            ((self.m_reflection_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("m_ReflectionProbeUsage".to_string(), v)),
            ((self.m_subset_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_SubsetIndices".to_string(), v)),
            ((self.m_lightmap_tiling_offset_dynamic).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapTilingOffsetDynamic".to_string(), v)),
            ((self.m_static_batch_root).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticBatchRoot".to_string(), v)),
            ((self.m_rendering_layer_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderingLayerMask".to_string(), v)),
            ((self.m_ray_tracing_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_RayTracingMode".to_string(), v)),
            ((self.m_quality).inspect_value(path, offset, limit))
                .map(|v| ("m_Quality".to_string(), v)),
            ((self.m_use_light_probes).inspect_value(path, offset, limit))
                .map(|v| ("m_UseLightProbes".to_string(), v)),
            ((self.m_renderer_priority).inspect_value(path, offset, limit))
                .map(|v| ("m_RendererPriority".to_string(), v)),
            ((self.m_ray_trace_procedural).inspect_value(path, offset, limit))
                .map(|v| ("m_RayTraceProcedural".to_string(), v)),
            ((self.m_update_when_offscreen).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdateWhenOffscreen".to_string(), v)),
            ((self.m_light_probe_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeAnchor".to_string(), v)),
            ((self.m_mesh).inspect_value(path, offset, limit)).map(|v| ("m_Mesh".to_string(), v)),
            ((self.m_sorting_layer).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayer".to_string(), v)),
            ((self.m_sorting_layer_id).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayerID".to_string(), v)),
            ((self.m_ray_tracing_accel_struct_build_flags_override)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_RayTracingAccelStructBuildFlagsOverride".to_string(), v)),
            ((self.m_bones).inspect_value(path, offset, limit)).map(|v| ("m_Bones".to_string(), v)),
            ((self.m_sorting_order).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingOrder".to_string(), v)),
            ((self.m_static_batch_info).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticBatchInfo".to_string(), v)),
            ((self.m_ray_tracing_accel_struct_build_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_RayTracingAccelStructBuildFlags".to_string(), v)),
            ((self.m_aabb).inspect_value(path, offset, limit)).map(|v| ("m_AABB".to_string(), v)),
            ((self.m_probe_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_ProbeAnchor".to_string(), v)),
            ((self.m_small_mesh_culling).inspect_value(path, offset, limit))
                .map(|v| ("m_SmallMeshCulling".to_string(), v)),
            ((self.m_dirty_aabb).inspect_value(path, offset, limit))
                .map(|v| ("m_DirtyAABB".to_string(), v)),
            ((self.m_force_mesh_lod).inspect_value(path, offset, limit))
                .map(|v| ("m_ForceMeshLod".to_string(), v)),
            ((self.m_root_bone).inspect_value(path, offset, limit))
                .map(|v| ("m_RootBone".to_string(), v)),
            ((self.m_light_probe_volume_override).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeVolumeOverride".to_string(), v)),
            ((self.m_mesh_lod_selection_bias).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshLodSelectionBias".to_string(), v)),
            ((self.m_blend_shape_weights).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendShapeWeights".to_string(), v)),
            ((self.m_skinned_motion_vectors).inspect_value(path, offset, limit))
                .map(|v| ("m_SkinnedMotionVectors".to_string(), v)),
            ((self.m_mask_interaction).inspect_value(path, offset, limit))
                .map(|v| ("m_MaskInteraction".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
