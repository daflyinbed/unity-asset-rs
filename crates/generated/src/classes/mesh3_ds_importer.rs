//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::clip_animation_info::ClipAnimationInfo;
use crate::subclasses::guid::Guid;
use crate::subclasses::human_description::HumanDescription;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::take_info::TakeInfo;

/// 版本枚举：Mesh3DsImporterMFileIdToRecycleNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMFileIdToRecycleNameVariant {
    Default(Vec<(i32, String)>),
    V5_0(Vec<(i64, String)>),
}

impl crate::Parse for Mesh3DsImporterMFileIdToRecycleNameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_i64()?;
                            let b = rdr.read_aligned_string()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_0(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_i32()?;
                            let b = rdr.read_aligned_string()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMFileIdToRecycleNameVariant {
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

/// 版本枚举：Mesh3DsImporterMImportMaterialsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMImportMaterialsVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for Mesh3DsImporterMImportMaterialsVariant {
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

impl crate::ToValue for Mesh3DsImporterMImportMaterialsVariant {
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

/// 版本枚举：Mesh3DsImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for Mesh3DsImporterMExternalObjectsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = SourceAssetIdentifier::parse(rdr, ver)?;
                            let b = crate::PPtr::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = SourceAssetIdentifier::parse(rdr, ver)?;
                            let b = crate::PPtr::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMExternalObjectsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Mesh3DsImporterMBakeSimulationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMBakeSimulationVariant {
    Default(bool),
    V4_0(bool),
    V4_3(bool),
}

impl crate::Parse for Mesh3DsImporterMBakeSimulationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_3(v))
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMBakeSimulationVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_0(v) => v.to_value(),
            Self::V4_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_0(v) => (v).inspect_value(path, offset, limit),
            Self::V4_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Mesh3DsImporterMSplitAnimationsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMSplitAnimationsVariant {
    Default(bool),
    V3_5(bool),
}

impl crate::Parse for Mesh3DsImporterMSplitAnimationsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V3_5(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMSplitAnimationsVariant {
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

/// 版本枚举：Mesh3DsImporterMOptimizeGameObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMOptimizeGameObjectsVariant {
    Default(bool),
    V2021_2(bool),
}

impl crate::Parse for Mesh3DsImporterMOptimizeGameObjectsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMOptimizeGameObjectsVariant {
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

/// 版本枚举：Mesh3DsImporterMUseFileUnitsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMUseFileUnitsVariant {
    Default(bool),
    V3_5(bool),
}

impl crate::Parse for Mesh3DsImporterMUseFileUnitsVariant {
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

impl crate::ToValue for Mesh3DsImporterMUseFileUnitsVariant {
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

/// 版本枚举：Mesh3DsImporterMClipAnimationsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMClipAnimationsVariant {
    Default(Vec<ClipAnimationInfo>),
    V2017_1(Vec<ClipAnimationInfo>),
}

impl crate::Parse for Mesh3DsImporterMClipAnimationsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ClipAnimationInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ClipAnimationInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMClipAnimationsVariant {
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

/// 版本枚举：Mesh3DsImporterMAnimationDoRetargetingWarningsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMAnimationDoRetargetingWarningsVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for Mesh3DsImporterMAnimationDoRetargetingWarningsVariant {
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

impl crate::ToValue for Mesh3DsImporterMAnimationDoRetargetingWarningsVariant {
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

/// 版本枚举：Mesh3DsImporterMExtraExposedTransformPathsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMExtraExposedTransformPathsVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for Mesh3DsImporterMExtraExposedTransformPathsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMExtraExposedTransformPathsVariant {
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

/// 版本枚举：Mesh3DsImporterMLodScreenPercentagesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMLodScreenPercentagesVariant {
    Default(Vec<f32>),
    V2017_1(Vec<f32>),
}

impl crate::Parse for Mesh3DsImporterMLodScreenPercentagesVariant {
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

impl crate::ToValue for Mesh3DsImporterMLodScreenPercentagesVariant {
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

/// 版本枚举：Mesh3DsImporterMImportAnimatedCustomPropertiesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMImportAnimatedCustomPropertiesVariant {
    Default(bool),
    V2018_1(bool),
}

impl crate::Parse for Mesh3DsImporterMImportAnimatedCustomPropertiesVariant {
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

impl crate::ToValue for Mesh3DsImporterMImportAnimatedCustomPropertiesVariant {
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

/// 版本枚举：Mesh3DsImporterOptimizeMeshForGpuVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterOptimizeMeshForGpuVariant {
    Default(bool),
    V4_1(bool),
}

impl crate::Parse for Mesh3DsImporterOptimizeMeshForGpuVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterOptimizeMeshForGpuVariant {
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

/// 版本枚举：Mesh3DsImporterWeldVerticesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterWeldVerticesVariant {
    Default(bool),
    V2017_3(bool),
}

impl crate::Parse for Mesh3DsImporterWeldVerticesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterWeldVerticesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Mesh3DsImporterNormalSmoothAngleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterNormalSmoothAngleVariant {
    Default(f32),
    V5_3(f32),
}

impl crate::Parse for Mesh3DsImporterNormalSmoothAngleVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = rdr.read_f32()?;
            rdr.align(4)?;
            Ok(Self::V5_3(v))
        } else {
            let v = rdr.read_f32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterNormalSmoothAngleVariant {
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

/// 版本枚举：Mesh3DsImporterMImportedRootsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMImportedRootsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for Mesh3DsImporterMImportedRootsVariant {
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

impl crate::ToValue for Mesh3DsImporterMImportedRootsVariant {
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

/// 版本枚举：Mesh3DsImporterTangentImportModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterTangentImportModeVariant {
    Default(i32),
    V2017_1(i32),
}

impl crate::Parse for Mesh3DsImporterTangentImportModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2017_1(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterTangentImportModeVariant {
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

/// 版本枚举：Mesh3DsImporterMHasExtraRootVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMHasExtraRootVariant {
    Default(bool),
    V4_3(bool),
    V2017_2(bool),
}

impl crate::Parse for Mesh3DsImporterMHasExtraRootVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_2(v))
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V4_3(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMHasExtraRootVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_3(v) => v.to_value(),
            Self::V2017_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Mesh3DsImporterMImportedTakeInfosVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMImportedTakeInfosVariant {
    Default(Vec<TakeInfo>),
    V2017_1(Vec<TakeInfo>),
}

impl crate::Parse for Mesh3DsImporterMImportedTakeInfosVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TakeInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TakeInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMImportedTakeInfosVariant {
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

/// 版本枚举：Mesh3DsImporterMReferencedClipsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMReferencedClipsVariant {
    Default(Vec<Guid>),
    V2017_1(Vec<Guid>),
    V6000_0(Vec<Guid>),
}

impl crate::Parse for Mesh3DsImporterMReferencedClipsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Guid::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V6000_0(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Guid::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Guid::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMReferencedClipsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
            Self::V6000_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Mesh3DsImporterMFileScaleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMFileScaleVariant {
    Default(f32),
    V2018_3(f32),
}

impl crate::Parse for Mesh3DsImporterMFileScaleVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_f32()?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_f32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMFileScaleVariant {
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

/// 版本枚举：Mesh3DsImporterMImportAnimationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMImportAnimationVariant {
    Default(bool),
    V2019_3(bool),
}

impl crate::Parse for Mesh3DsImporterMImportAnimationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2019_3(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMImportAnimationVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Mesh3DsImporterMAnimationTypeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMAnimationTypeVariant {
    Default(i32),
    V4_3_3(i32),
}

impl crate::Parse for Mesh3DsImporterMAnimationTypeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 3) {
            let v = rdr.read_i32()?;
            Ok(Self::V4_3_3(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMAnimationTypeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_3_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_3_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Mesh3DsImporterNormalCalculationModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterNormalCalculationModeVariant {
    Default(i32),
    V2018_3(i32),
}

impl crate::Parse for Mesh3DsImporterNormalCalculationModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterNormalCalculationModeVariant {
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

/// 版本枚举：Mesh3DsImporterOptimizeBonesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterOptimizeBonesVariant {
    Default(bool),
    V6000_2(bool),
}

impl crate::Parse for Mesh3DsImporterOptimizeBonesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterOptimizeBonesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Mesh3DsImporterMHasEmbeddedTexturesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMHasEmbeddedTexturesVariant {
    Default(bool),
    V2023_1(bool),
}

impl crate::Parse for Mesh3DsImporterMHasEmbeddedTexturesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2023_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMHasEmbeddedTexturesVariant {
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

/// 版本枚举：Mesh3DsImporterNormalSmoothingSourceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterNormalSmoothingSourceVariant {
    Default(i32),
    V6000_5(i32),
}

impl crate::Parse for Mesh3DsImporterNormalSmoothingSourceVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V6000_5(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterNormalSmoothingSourceVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant {
    Default(bool),
    V2020_3_37(bool),
    V2021_1(bool),
    V2021_3_5(bool),
}

impl crate::Parse for Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 3, 5) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_3_5(v))
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2021_1(v))
        } else if *ver >= crate::UnityVersion::new(2020, 3, 37) {
            let v = rdr.read_bool()?;
            Ok(Self::V2020_3_37(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_3_37(v) => v.to_value(),
            Self::V2021_1(v) => v.to_value(),
            Self::V2021_3_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_3_37(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2021_3_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：Mesh3DsImporterMImportBlendShapeDeformPercentVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum Mesh3DsImporterMImportBlendShapeDeformPercentVariant {
    Default(bool),
    V2022_1_5(bool),
}

impl crate::Parse for Mesh3DsImporterMImportBlendShapeDeformPercentVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 5) {
            let v = rdr.read_bool()?;
            Ok(Self::V2022_1_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for Mesh3DsImporterMImportBlendShapeDeformPercentVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_1_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Mesh3DsImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Mesh3DsImporterMFileIdToRecycleNameVariant>,
    pub m_internal_id_to_name_table: Option<Vec<((i32, i64), String)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_import_materials: Option<Mesh3DsImporterMImportMaterialsVariant>,
    pub m_external_objects: Option<Mesh3DsImporterMExternalObjectsVariant>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_material_name: Option<i32>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_generate_materials: Option<i32>,
    pub m_material_search: Option<i32>,
    pub m_supports_embedded_materials: Option<bool>,
    pub m_generate_animations: Option<i32>,
    pub m_legacy_generate_animations: Option<i32>,
    pub m_material_import_mode: Option<i32>,
    pub m_global_scale: f32,
    pub m_bake_simulation: Mesh3DsImporterMBakeSimulationVariant,
    pub m_split_animations: Option<Mesh3DsImporterMSplitAnimationsVariant>,
    pub m_animation_compression: i32,
    pub m_optimize_game_objects: Option<Mesh3DsImporterMOptimizeGameObjectsVariant>,
    pub m_resample_rotations: Option<bool>,
    pub m_resample_curves: Option<bool>,
    pub m_material_location: Option<i32>,
    pub m_animation_rotation_error: f32,
    pub m_motion_node_name: Option<String>,
    pub m_materials: Option<Vec<SourceAssetIdentifier>>,
    pub m_add_colliders: bool,
    pub m_animation_position_error: f32,
    pub m_animation_import_errors: Option<String>,
    pub m_search_textures_globally: Option<bool>,
    pub m_use_file_units: Mesh3DsImporterMUseFileUnitsVariant,
    pub m_animation_scale_error: f32,
    pub m_animation_import_warnings: Option<String>,
    pub m_rig_import_errors: Option<String>,
    pub m_animation_wrap_mode: i32,
    pub m_animation_retargeting_warnings: Option<String>,
    pub m_rig_import_warnings: Option<String>,
    pub m_clip_animations: Mesh3DsImporterMClipAnimationsVariant,
    pub m_animation_do_retargeting_warnings:
        Option<Mesh3DsImporterMAnimationDoRetargetingWarningsVariant>,
    pub m_is_readable: Option<bool>,
    pub m_extra_exposed_transform_paths: Option<Mesh3DsImporterMExtraExposedTransformPathsVariant>,
    pub m_lod_screen_percentages: Option<Mesh3DsImporterMLodScreenPercentagesVariant>,
    pub m_remove_constant_scale_curves: Option<bool>,
    pub m_mesh_compression: i32,
    pub m_mesh_settings_swap_uv_channels: Option<bool>,
    pub m_mesh_settings_generate_secondary_uv: Option<bool>,
    pub m_mesh_settings_secondary_uv_angle_distortion: Option<f32>,
    pub swap_uv_channels: Option<bool>,
    pub m_import_blend_shapes: Option<bool>,
    pub m_mesh_settings_secondary_uv_area_distortion: Option<f32>,
    pub generate_secondary_uv: Option<bool>,
    pub m_mesh_settings_secondary_uv_hard_angle: Option<f32>,
    pub m_import_animated_custom_properties:
        Option<Mesh3DsImporterMImportAnimatedCustomPropertiesVariant>,
    pub m_mesh_settings_secondary_uv_pack_margin: Option<f32>,
    pub optimize_mesh: Option<bool>,
    pub optimize_mesh_for_gpu: Option<Mesh3DsImporterOptimizeMeshForGpuVariant>,
    pub m_import_constraints: Option<bool>,
    pub m_mesh_settings_normal_import_mode: Option<i32>,
    pub secondary_uv_angle_distortion: Option<f32>,
    pub m_extra_user_properties: Option<Vec<String>>,
    pub m_mesh_settings_tangent_import_mode: Option<i32>,
    pub secondary_uv_area_distortion: Option<f32>,
    pub weld_vertices: Option<Mesh3DsImporterWeldVerticesVariant>,
    pub secondary_uv_hard_angle: Option<f32>,
    pub secondary_uv_pack_margin: Option<f32>,
    pub m_first_import_version: Option<i32>,
    pub normal_smooth_angle: Mesh3DsImporterNormalSmoothAngleVariant,
    pub keep_quads: Option<bool>,
    pub split_tangents_across_uv: Option<bool>,
    pub normal_import_mode: Option<i32>,
    pub m_imported_roots: Mesh3DsImporterMImportedRootsVariant,
    pub tangent_import_mode: Option<Mesh3DsImporterTangentImportModeVariant>,
    pub m_import_visibility: Option<bool>,
    pub m_has_extra_root: Mesh3DsImporterMHasExtraRootVariant,
    pub m_imported_take_infos: Option<Mesh3DsImporterMImportedTakeInfosVariant>,
    pub m_referenced_clips: Option<Mesh3DsImporterMReferencedClipsVariant>,
    pub m_import_cameras: Option<bool>,
    pub m_use_file_scale: Option<bool>,
    pub m_import_lights: Option<bool>,
    pub m_file_scale: Option<Mesh3DsImporterMFileScaleVariant>,
    pub m_import_animation: Option<Mesh3DsImporterMImportAnimationVariant>,
    pub m_copy_avatar: Option<bool>,
    pub m_use_srgb_material_color: Option<bool>,
    pub m_human_description: Option<HumanDescription>,
    pub m_sort_hierarchy_by_name: Option<bool>,
    pub m_last_human_description_avatar_source: Option<PPtr>,
    pub m_import_physical_cameras: Option<bool>,
    pub m_additional_bone: Option<bool>,
    pub m_animation_type: Option<Mesh3DsImporterMAnimationTypeVariant>,
    pub m_user_data: Option<String>,
    pub m_file_ids_generation: Option<i32>,
    pub m_node_name_collision_strategy: Option<i32>,
    pub m_preserve_hierarchy: Option<bool>,
    pub index_format: Option<i32>,
    pub skin_weights_mode: Option<i32>,
    pub normal_calculation_mode: Option<Mesh3DsImporterNormalCalculationModeVariant>,
    pub max_bones_per_vertex: Option<i32>,
    pub bake_axis_conversion: Option<bool>,
    pub m_asset_bundle_name: Option<String>,
    pub min_bone_weight: Option<f32>,
    pub m_asset_bundle_variant: Option<String>,
    pub mesh_optimization_flags: Option<i32>,
    pub m_humanoid_oversampling: Option<i32>,
    pub m_previous_calculated_global_scale: Option<f32>,
    pub m_has_previous_calculated_global_scale: Option<bool>,
    pub m_file_scale_unit: Option<String>,
    pub optimize_bones: Option<Mesh3DsImporterOptimizeBonesVariant>,
    pub m_file_scale_factor: Option<f32>,
    pub generate_mesh_lods: Option<bool>,
    pub mesh_lod_generation_flags: Option<i32>,
    pub maximum_mesh_lod: Option<i32>,
    pub m_has_embedded_textures: Option<Mesh3DsImporterMHasEmbeddedTexturesVariant>,
    pub secondary_uv_margin_method: Option<i32>,
    pub import_u_vs: Option<i32>,
    pub secondary_uv_min_lightmap_resolution: Option<f32>,
    pub import_vertex_colors: Option<bool>,
    pub secondary_uv_min_object_scale: Option<f32>,
    pub legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes: Option<bool>,
    pub blend_shape_normal_import_mode: Option<i32>,
    pub normal_smoothing_source: Option<Mesh3DsImporterNormalSmoothingSourceVariant>,
    pub m_strict_vertex_data_checks: Option<bool>,
    pub m_auto_generate_avatar_mapping_if_unspecified: Option<bool>,
    pub m_avatar_setup: Option<i32>,
    pub m_contains_animation: Option<bool>,
    pub calculate_blendshape_normals_delta_from_imported_normals: Option<bool>,
    pub m_add_humanoid_extra_root_only_when_using_avatar:
        Option<Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant>,
    pub m_remap_materials_if_material_import_mode_is_none: Option<bool>,
    pub m_import_blend_shape_deform_percent:
        Option<Mesh3DsImporterMImportBlendShapeDeformPercentVariant>,
}

impl crate::Parse for Mesh3DsImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_internal_id_to_name_table = None;
        let mut m_old_hash_identity = None;
        let mut m_import_materials = None;
        let mut m_external_objects = None;
        let mut m_new_hash_identity = None;
        let mut m_material_name = None;
        let mut m_used_file_i_ds = None;
        let mut m_generate_materials = None;
        let mut m_material_search = None;
        let mut m_supports_embedded_materials = None;
        let mut m_generate_animations = None;
        let mut m_legacy_generate_animations = None;
        let mut m_material_import_mode = None;
        let mut m_global_scale = None;
        let mut m_bake_simulation = None;
        let mut m_split_animations = None;
        let mut m_animation_compression = None;
        let mut m_optimize_game_objects = None;
        let mut m_resample_rotations = None;
        let mut m_resample_curves = None;
        let mut m_material_location = None;
        let mut m_animation_rotation_error = None;
        let mut m_motion_node_name = None;
        let mut m_materials = None;
        let mut m_add_colliders = None;
        let mut m_animation_position_error = None;
        let mut m_animation_import_errors = None;
        let mut m_search_textures_globally = None;
        let mut m_use_file_units = None;
        let mut m_animation_scale_error = None;
        let mut m_animation_import_warnings = None;
        let mut m_rig_import_errors = None;
        let mut m_animation_wrap_mode = None;
        let mut m_animation_retargeting_warnings = None;
        let mut m_rig_import_warnings = None;
        let mut m_clip_animations = None;
        let mut m_animation_do_retargeting_warnings = None;
        let mut m_is_readable = None;
        let mut m_extra_exposed_transform_paths = None;
        let mut m_lod_screen_percentages = None;
        let mut m_remove_constant_scale_curves = None;
        let mut m_mesh_compression = None;
        let mut m_mesh_settings_swap_uv_channels = None;
        let mut m_mesh_settings_generate_secondary_uv = None;
        let mut m_mesh_settings_secondary_uv_angle_distortion = None;
        let mut swap_uv_channels = None;
        let mut m_import_blend_shapes = None;
        let mut m_mesh_settings_secondary_uv_area_distortion = None;
        let mut generate_secondary_uv = None;
        let mut m_mesh_settings_secondary_uv_hard_angle = None;
        let mut m_import_animated_custom_properties = None;
        let mut m_mesh_settings_secondary_uv_pack_margin = None;
        let mut optimize_mesh = None;
        let mut optimize_mesh_for_gpu = None;
        let mut m_import_constraints = None;
        let mut m_mesh_settings_normal_import_mode = None;
        let mut secondary_uv_angle_distortion = None;
        let mut m_extra_user_properties = None;
        let mut m_mesh_settings_tangent_import_mode = None;
        let mut secondary_uv_area_distortion = None;
        let mut weld_vertices = None;
        let mut secondary_uv_hard_angle = None;
        let mut secondary_uv_pack_margin = None;
        let mut m_first_import_version = None;
        let mut normal_smooth_angle = None;
        let mut keep_quads = None;
        let mut split_tangents_across_uv = None;
        let mut normal_import_mode = None;
        let mut m_imported_roots = None;
        let mut tangent_import_mode = None;
        let mut m_import_visibility = None;
        let mut m_has_extra_root = None;
        let mut m_imported_take_infos = None;
        let mut m_referenced_clips = None;
        let mut m_import_cameras = None;
        let mut m_use_file_scale = None;
        let mut m_import_lights = None;
        let mut m_file_scale = None;
        let mut m_import_animation = None;
        let mut m_copy_avatar = None;
        let mut m_use_srgb_material_color = None;
        let mut m_human_description = None;
        let mut m_sort_hierarchy_by_name = None;
        let mut m_last_human_description_avatar_source = None;
        let mut m_import_physical_cameras = None;
        let mut m_additional_bone = None;
        let mut m_animation_type = None;
        let mut m_user_data = None;
        let mut m_file_ids_generation = None;
        let mut m_node_name_collision_strategy = None;
        let mut m_preserve_hierarchy = None;
        let mut index_format = None;
        let mut skin_weights_mode = None;
        let mut normal_calculation_mode = None;
        let mut max_bones_per_vertex = None;
        let mut bake_axis_conversion = None;
        let mut m_asset_bundle_name = None;
        let mut min_bone_weight = None;
        let mut m_asset_bundle_variant = None;
        let mut mesh_optimization_flags = None;
        let mut m_humanoid_oversampling = None;
        let mut m_previous_calculated_global_scale = None;
        let mut m_has_previous_calculated_global_scale = None;
        let mut m_file_scale_unit = None;
        let mut optimize_bones = None;
        let mut m_file_scale_factor = None;
        let mut generate_mesh_lods = None;
        let mut mesh_lod_generation_flags = None;
        let mut maximum_mesh_lod = None;
        let mut m_has_embedded_textures = None;
        let mut secondary_uv_margin_method = None;
        let mut import_u_vs = None;
        let mut secondary_uv_min_lightmap_resolution = None;
        let mut import_vertex_colors = None;
        let mut secondary_uv_min_object_scale = None;
        let mut legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes = None;
        let mut blend_shape_normal_import_mode = None;
        let mut normal_smoothing_source = None;
        let mut m_strict_vertex_data_checks = None;
        let mut m_auto_generate_avatar_mapping_if_unspecified = None;
        let mut m_avatar_setup = None;
        let mut m_contains_animation = None;
        let mut calculate_blendshape_normals_delta_from_imported_normals = None;
        let mut m_add_humanoid_extra_root_only_when_using_avatar = None;
        let mut m_remap_materials_if_material_import_mode_is_none = None;
        let mut m_import_blend_shape_deform_percent = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_internal_id_to_name_table = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = ({
                                    let a = rdr.read_i32()?;
                                    let b = rdr.read_i64()?;
                                    Ok((a, b))
                                })?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_external_objects = Some(Mesh3DsImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_material_import_mode = Some(rdr.read_i32()?);
            m_material_name = Some(rdr.read_i32()?);
            m_material_search = Some(rdr.read_i32()?);
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_legacy_generate_animations = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_internal_id_to_name_table = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = ({
                                    let a = rdr.read_i32()?;
                                    let b = rdr.read_i64()?;
                                    Ok((a, b))
                                })?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_external_objects = Some(Mesh3DsImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_material_import_mode = Some(rdr.read_i32()?);
            m_material_name = Some(rdr.read_i32()?);
            m_material_search = Some(rdr.read_i32()?);
            m_material_location = Some(rdr.read_i32()?);
            m_search_textures_globally = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_legacy_generate_animations = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_internal_id_to_name_table = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = ({
                                    let a = rdr.read_i32()?;
                                    let b = rdr.read_i64()?;
                                    Ok((a, b))
                                })?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_external_objects = Some(Mesh3DsImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_material_import_mode = Some(rdr.read_i32()?);
            m_material_name = Some(rdr.read_i32()?);
            m_material_search = Some(rdr.read_i32()?);
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_legacy_generate_animations = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_internal_id_to_name_table = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = ({
                                    let a = rdr.read_i32()?;
                                    let b = rdr.read_i64()?;
                                    Ok((a, b))
                                })?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_external_objects = Some(Mesh3DsImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_import_materials = Some(Mesh3DsImporterMImportMaterialsVariant::parse(rdr, ver)?);
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_material_name = Some(rdr.read_i32()?);
            m_material_search = Some(rdr.read_i32()?);
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_legacy_generate_animations = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_file_id_to_recycle_name =
                Some(Mesh3DsImporterMFileIdToRecycleNameVariant::parse(rdr, ver)?);
            m_external_objects = Some(Mesh3DsImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_import_materials = Some(Mesh3DsImporterMImportMaterialsVariant::parse(rdr, ver)?);
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_material_name = Some(rdr.read_i32()?);
            m_material_search = Some(rdr.read_i32()?);
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SourceAssetIdentifier::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_legacy_generate_animations = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_file_id_to_recycle_name =
                Some(Mesh3DsImporterMFileIdToRecycleNameVariant::parse(rdr, ver)?);
            m_import_materials = Some(Mesh3DsImporterMImportMaterialsVariant::parse(rdr, ver)?);
            m_material_name = Some(rdr.read_i32()?);
            m_material_search = Some(rdr.read_i32()?);
            m_legacy_generate_animations = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_file_id_to_recycle_name =
                Some(Mesh3DsImporterMFileIdToRecycleNameVariant::parse(rdr, ver)?);
            m_import_materials = Some(Mesh3DsImporterMImportMaterialsVariant::parse(rdr, ver)?);
            m_material_name = Some(rdr.read_i32()?);
            m_material_search = Some(rdr.read_i32()?);
            m_generate_animations = Some(rdr.read_i32()?);
        } else {
            m_file_id_to_recycle_name =
                Some(Mesh3DsImporterMFileIdToRecycleNameVariant::parse(rdr, ver)?);
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_generate_materials = Some(rdr.read_i32()?);
            m_generate_animations = Some(rdr.read_i32()?);
            m_global_scale = Some(rdr.read_f32()?);
        }
        m_bake_simulation = Some(Mesh3DsImporterMBakeSimulationVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(Mesh3DsImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_remove_constant_scale_curves = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings =
                Some(Mesh3DsImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?);
            m_import_animated_custom_properties =
                Some(Mesh3DsImporterMImportAnimatedCustomPropertiesVariant::parse(rdr, ver)?);
            m_import_constraints = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(Mesh3DsImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_remove_constant_scale_curves = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_rig_import_errors = Some(rdr.read_aligned_string()?);
            m_rig_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings =
                Some(Mesh3DsImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?);
            m_import_animated_custom_properties =
                Some(Mesh3DsImporterMImportAnimatedCustomPropertiesVariant::parse(rdr, ver)?);
            m_import_constraints = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(Mesh3DsImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_rig_import_errors = Some(rdr.read_aligned_string()?);
            m_rig_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings =
                Some(Mesh3DsImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?);
            m_import_animated_custom_properties =
                Some(Mesh3DsImporterMImportAnimatedCustomPropertiesVariant::parse(rdr, ver)?);
            m_import_constraints = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(Mesh3DsImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_rig_import_errors = Some(rdr.read_aligned_string()?);
            m_rig_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings =
                Some(Mesh3DsImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?);
            m_import_animated_custom_properties =
                Some(Mesh3DsImporterMImportAnimatedCustomPropertiesVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(Mesh3DsImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_rig_import_errors = Some(rdr.read_aligned_string()?);
            m_rig_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings =
                Some(Mesh3DsImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(Mesh3DsImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings =
                Some(Mesh3DsImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_resample_rotations = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(Mesh3DsImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings =
                Some(Mesh3DsImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_optimize_game_objects =
                Some(Mesh3DsImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings =
                Some(Mesh3DsImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_optimize_game_objects =
                Some(Mesh3DsImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_optimize_game_objects =
                Some(Mesh3DsImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_split_animations = Some(Mesh3DsImporterMSplitAnimationsVariant::parse(rdr, ver)?);
        } else {
            m_split_animations = Some(Mesh3DsImporterMSplitAnimationsVariant::parse(rdr, ver)?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
        }
        m_animation_compression = Some(rdr.read_i32()?);
        m_animation_rotation_error = Some(rdr.read_f32()?);
        m_animation_position_error = Some(rdr.read_f32()?);
        m_animation_scale_error = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_mesh_compression = Some(rdr.read_i32()?);
            m_mesh_settings_swap_uv_channels = Some(rdr.read_bool()?);
            m_mesh_settings_generate_secondary_uv = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_mesh_settings_secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            m_mesh_settings_secondary_uv_area_distortion = Some(rdr.read_f32()?);
            m_mesh_settings_secondary_uv_hard_angle = Some(rdr.read_f32()?);
            m_mesh_settings_secondary_uv_pack_margin = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_mesh_settings_normal_import_mode = Some(rdr.read_i32()?);
            m_mesh_settings_tangent_import_mode = Some(rdr.read_i32()?);
        }
        m_animation_wrap_mode = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_extra_exposed_transform_paths = Some(
                Mesh3DsImporterMExtraExposedTransformPathsVariant::parse(rdr, ver)?,
            );
            m_extra_user_properties = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_extra_exposed_transform_paths = Some(
                Mesh3DsImporterMExtraExposedTransformPathsVariant::parse(rdr, ver)?,
            );
        } else {
        }
        m_clip_animations = Some(Mesh3DsImporterMClipAnimationsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_sort_hierarchy_by_name = Some(rdr.read_bool()?);
            m_import_physical_cameras = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_node_name_collision_strategy = Some(rdr.read_i32()?);
            m_file_ids_generation = Some(rdr.read_i32()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(Mesh3DsImporterOptimizeBonesVariant::parse(rdr, ver)?);
            generate_mesh_lods = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mesh_lod_generation_flags = Some(rdr.read_i32()?);
            maximum_mesh_lod = Some(rdr.read_i32()?);
            import_u_vs = Some(rdr.read_i32()?);
            import_vertex_colors = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_margin_method = Some(rdr.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(rdr.read_f32()?);
            secondary_uv_min_object_scale = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_strict_vertex_data_checks = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_sort_hierarchy_by_name = Some(rdr.read_bool()?);
            m_import_physical_cameras = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_node_name_collision_strategy = Some(rdr.read_i32()?);
            m_file_ids_generation = Some(rdr.read_i32()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(Mesh3DsImporterOptimizeBonesVariant::parse(rdr, ver)?);
            generate_mesh_lods = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mesh_lod_generation_flags = Some(rdr.read_i32()?);
            maximum_mesh_lod = Some(rdr.read_i32()?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_margin_method = Some(rdr.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(rdr.read_f32()?);
            secondary_uv_min_object_scale = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_strict_vertex_data_checks = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 19) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_sort_hierarchy_by_name = Some(rdr.read_bool()?);
            m_import_physical_cameras = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_node_name_collision_strategy = Some(rdr.read_i32()?);
            m_file_ids_generation = Some(rdr.read_i32()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(Mesh3DsImporterOptimizeBonesVariant::parse(rdr, ver)?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_margin_method = Some(rdr.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(rdr.read_f32()?);
            secondary_uv_min_object_scale = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_strict_vertex_data_checks = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 36) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_sort_hierarchy_by_name = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_node_name_collision_strategy = Some(rdr.read_i32()?);
            m_file_ids_generation = Some(rdr.read_i32()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(Mesh3DsImporterOptimizeBonesVariant::parse(rdr, ver)?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_margin_method = Some(rdr.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(rdr.read_f32()?);
            secondary_uv_min_object_scale = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_strict_vertex_data_checks = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_sort_hierarchy_by_name = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_node_name_collision_strategy = Some(rdr.read_i32()?);
            m_file_ids_generation = Some(rdr.read_i32()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(Mesh3DsImporterOptimizeBonesVariant::parse(rdr, ver)?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_margin_method = Some(rdr.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(rdr.read_f32()?);
            secondary_uv_min_object_scale = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_sort_hierarchy_by_name = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_ids_generation = Some(rdr.read_i32()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_margin_method = Some(rdr.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(rdr.read_f32()?);
            secondary_uv_min_object_scale = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 17) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_sort_hierarchy_by_name = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_ids_generation = Some(rdr.read_i32()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_sort_hierarchy_by_name = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu =
                Some(Mesh3DsImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_previous_calculated_global_scale = Some(rdr.read_f32()?);
            m_has_previous_calculated_global_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu =
                Some(Mesh3DsImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
            m_previous_calculated_global_scale = Some(rdr.read_f32()?);
            m_has_previous_calculated_global_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu =
                Some(Mesh3DsImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu =
                Some(Mesh3DsImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu =
                Some(Mesh3DsImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(Mesh3DsImporterMFileScaleVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu =
                Some(Mesh3DsImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            weld_vertices = Some(Mesh3DsImporterWeldVerticesVariant::parse(rdr, ver)?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu =
                Some(Mesh3DsImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_lod_screen_percentages = Some(Mesh3DsImporterMLodScreenPercentagesVariant::parse(
                rdr, ver,
            )?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(Mesh3DsImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh = Some(rdr.read_bool()?);
            rdr.align(4)?;
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
        } else {
            m_first_import_version = Some(rdr.read_i32()?);
        }
        normal_smooth_angle = Some(Mesh3DsImporterNormalSmoothAngleVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(Mesh3DsImporterTangentImportModeVariant::parse(rdr, ver)?);
            normal_calculation_mode = Some(Mesh3DsImporterNormalCalculationModeVariant::parse(
                rdr, ver,
            )?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes =
                Some(rdr.read_bool()?);
            rdr.align(4)?;
            blend_shape_normal_import_mode = Some(rdr.read_i32()?);
            normal_smoothing_source = Some(Mesh3DsImporterNormalSmoothingSourceVariant::parse(
                rdr, ver,
            )?);
            calculate_blendshape_normals_delta_from_imported_normals = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_imported_take_infos =
                Some(Mesh3DsImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(Mesh3DsImporterTangentImportModeVariant::parse(rdr, ver)?);
            normal_calculation_mode = Some(Mesh3DsImporterNormalCalculationModeVariant::parse(
                rdr, ver,
            )?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes =
                Some(rdr.read_bool()?);
            rdr.align(4)?;
            blend_shape_normal_import_mode = Some(rdr.read_i32()?);
            normal_smoothing_source = Some(Mesh3DsImporterNormalSmoothingSourceVariant::parse(
                rdr, ver,
            )?);
            m_imported_take_infos =
                Some(Mesh3DsImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(Mesh3DsImporterTangentImportModeVariant::parse(rdr, ver)?);
            normal_calculation_mode = Some(Mesh3DsImporterNormalCalculationModeVariant::parse(
                rdr, ver,
            )?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes =
                Some(rdr.read_bool()?);
            rdr.align(4)?;
            blend_shape_normal_import_mode = Some(rdr.read_i32()?);
            normal_smoothing_source = Some(Mesh3DsImporterNormalSmoothingSourceVariant::parse(
                rdr, ver,
            )?);
            m_imported_take_infos =
                Some(Mesh3DsImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(Mesh3DsImporterTangentImportModeVariant::parse(rdr, ver)?);
            normal_calculation_mode = Some(Mesh3DsImporterNormalCalculationModeVariant::parse(
                rdr, ver,
            )?);
            m_imported_take_infos =
                Some(Mesh3DsImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(Mesh3DsImporterTangentImportModeVariant::parse(rdr, ver)?);
            m_imported_take_infos =
                Some(Mesh3DsImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            split_tangents_across_uv = Some(rdr.read_bool()?);
            rdr.align(4)?;
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(Mesh3DsImporterTangentImportModeVariant::parse(rdr, ver)?);
            m_imported_take_infos =
                Some(Mesh3DsImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            split_tangents_across_uv = Some(rdr.read_bool()?);
            rdr.align(4)?;
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(Mesh3DsImporterTangentImportModeVariant::parse(rdr, ver)?);
        } else {
            split_tangents_across_uv = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_imported_roots = Some(Mesh3DsImporterMImportedRootsVariant::parse(rdr, ver)?);
        m_has_extra_root = Some(Mesh3DsImporterMHasExtraRootVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_has_embedded_textures =
                Some(Mesh3DsImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_contains_animation = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(
                Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?,
            );
            m_import_blend_shape_deform_percent = Some(
                Mesh3DsImporterMImportBlendShapeDeformPercentVariant::parse(rdr, ver)?,
            );
            m_remap_materials_if_material_import_mode_is_none = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 5) {
            m_has_embedded_textures =
                Some(Mesh3DsImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(
                Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?,
            );
            m_import_blend_shape_deform_percent = Some(
                Mesh3DsImporterMImportBlendShapeDeformPercentVariant::parse(rdr, ver)?,
            );
            m_remap_materials_if_material_import_mode_is_none = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_has_embedded_textures =
                Some(Mesh3DsImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(
                Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?,
            );
            m_import_blend_shape_deform_percent = Some(
                Mesh3DsImporterMImportBlendShapeDeformPercentVariant::parse(rdr, ver)?,
            );
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 5) {
            m_has_embedded_textures =
                Some(Mesh3DsImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(
                Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?,
            );
            m_remap_materials_if_material_import_mode_is_none = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_has_embedded_textures =
                Some(Mesh3DsImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(
                Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?,
            );
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 37) {
            m_has_embedded_textures =
                Some(Mesh3DsImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(
                Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?,
            );
            m_remap_materials_if_material_import_mode_is_none = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_has_embedded_textures =
                Some(Mesh3DsImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(
                Mesh3DsImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?,
            );
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_has_embedded_textures =
                Some(Mesh3DsImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_has_embedded_textures =
                Some(Mesh3DsImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(Mesh3DsImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_has_embedded_textures =
                Some(Mesh3DsImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 3) {
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_import_animation = Some(Mesh3DsImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(Mesh3DsImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
        } else {
        }

        Ok(Mesh3DsImporter {
            m_name: m_name.unwrap(),
            m_file_id_to_recycle_name,
            m_internal_id_to_name_table,
            m_old_hash_identity,
            m_import_materials,
            m_external_objects,
            m_new_hash_identity,
            m_material_name,
            m_used_file_i_ds,
            m_generate_materials,
            m_material_search,
            m_supports_embedded_materials,
            m_generate_animations,
            m_legacy_generate_animations,
            m_material_import_mode,
            m_global_scale: m_global_scale.unwrap(),
            m_bake_simulation: m_bake_simulation.unwrap(),
            m_split_animations,
            m_animation_compression: m_animation_compression.unwrap(),
            m_optimize_game_objects,
            m_resample_rotations,
            m_resample_curves,
            m_material_location,
            m_animation_rotation_error: m_animation_rotation_error.unwrap(),
            m_motion_node_name,
            m_materials,
            m_add_colliders: m_add_colliders.unwrap(),
            m_animation_position_error: m_animation_position_error.unwrap(),
            m_animation_import_errors,
            m_search_textures_globally,
            m_use_file_units: m_use_file_units.unwrap(),
            m_animation_scale_error: m_animation_scale_error.unwrap(),
            m_animation_import_warnings,
            m_rig_import_errors,
            m_animation_wrap_mode: m_animation_wrap_mode.unwrap(),
            m_animation_retargeting_warnings,
            m_rig_import_warnings,
            m_clip_animations: m_clip_animations.unwrap(),
            m_animation_do_retargeting_warnings,
            m_is_readable,
            m_extra_exposed_transform_paths,
            m_lod_screen_percentages,
            m_remove_constant_scale_curves,
            m_mesh_compression: m_mesh_compression.unwrap(),
            m_mesh_settings_swap_uv_channels,
            m_mesh_settings_generate_secondary_uv,
            m_mesh_settings_secondary_uv_angle_distortion,
            swap_uv_channels,
            m_import_blend_shapes,
            m_mesh_settings_secondary_uv_area_distortion,
            generate_secondary_uv,
            m_mesh_settings_secondary_uv_hard_angle,
            m_import_animated_custom_properties,
            m_mesh_settings_secondary_uv_pack_margin,
            optimize_mesh,
            optimize_mesh_for_gpu,
            m_import_constraints,
            m_mesh_settings_normal_import_mode,
            secondary_uv_angle_distortion,
            m_extra_user_properties,
            m_mesh_settings_tangent_import_mode,
            secondary_uv_area_distortion,
            weld_vertices,
            secondary_uv_hard_angle,
            secondary_uv_pack_margin,
            m_first_import_version,
            normal_smooth_angle: normal_smooth_angle.unwrap(),
            keep_quads,
            split_tangents_across_uv,
            normal_import_mode,
            m_imported_roots: m_imported_roots.unwrap(),
            tangent_import_mode,
            m_import_visibility,
            m_has_extra_root: m_has_extra_root.unwrap(),
            m_imported_take_infos,
            m_referenced_clips,
            m_import_cameras,
            m_use_file_scale,
            m_import_lights,
            m_file_scale,
            m_import_animation,
            m_copy_avatar,
            m_use_srgb_material_color,
            m_human_description,
            m_sort_hierarchy_by_name,
            m_last_human_description_avatar_source,
            m_import_physical_cameras,
            m_additional_bone,
            m_animation_type,
            m_user_data,
            m_file_ids_generation,
            m_node_name_collision_strategy,
            m_preserve_hierarchy,
            index_format,
            skin_weights_mode,
            normal_calculation_mode,
            max_bones_per_vertex,
            bake_axis_conversion,
            m_asset_bundle_name,
            min_bone_weight,
            m_asset_bundle_variant,
            mesh_optimization_flags,
            m_humanoid_oversampling,
            m_previous_calculated_global_scale,
            m_has_previous_calculated_global_scale,
            m_file_scale_unit,
            optimize_bones,
            m_file_scale_factor,
            generate_mesh_lods,
            mesh_lod_generation_flags,
            maximum_mesh_lod,
            m_has_embedded_textures,
            secondary_uv_margin_method,
            import_u_vs,
            secondary_uv_min_lightmap_resolution,
            import_vertex_colors,
            secondary_uv_min_object_scale,
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes,
            blend_shape_normal_import_mode,
            normal_smoothing_source,
            m_strict_vertex_data_checks,
            m_auto_generate_avatar_mapping_if_unspecified,
            m_avatar_setup,
            m_contains_animation,
            calculate_blendshape_normals_delta_from_imported_normals,
            m_add_humanoid_extra_root_only_when_using_avatar,
            m_remap_materials_if_material_import_mode_is_none,
            m_import_blend_shape_deform_percent,
        })
    }
}

impl crate::ToValue for Mesh3DsImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_FileIDToRecycleName", &self.m_file_id_to_recycle_name),
            crate::opt_field("m_InternalIDToNameTable", &self.m_internal_id_to_name_table),
            crate::opt_field("m_OldHashIdentity", &self.m_old_hash_identity),
            crate::opt_field("m_ImportMaterials", &self.m_import_materials),
            crate::opt_field("m_ExternalObjects", &self.m_external_objects),
            crate::opt_field("m_NewHashIdentity", &self.m_new_hash_identity),
            crate::opt_field("m_MaterialName", &self.m_material_name),
            crate::opt_field("m_UsedFileIDs", &self.m_used_file_i_ds),
            crate::opt_field("m_GenerateMaterials", &self.m_generate_materials),
            crate::opt_field("m_MaterialSearch", &self.m_material_search),
            crate::opt_field(
                "m_SupportsEmbeddedMaterials",
                &self.m_supports_embedded_materials,
            ),
            crate::opt_field("m_GenerateAnimations", &self.m_generate_animations),
            crate::opt_field(
                "m_LegacyGenerateAnimations",
                &self.m_legacy_generate_animations,
            ),
            crate::opt_field("m_MaterialImportMode", &self.m_material_import_mode),
            Some(("m_GlobalScale".to_string(), self.m_global_scale.to_value())),
            Some((
                "m_BakeSimulation".to_string(),
                self.m_bake_simulation.to_value(),
            )),
            crate::opt_field("m_SplitAnimations", &self.m_split_animations),
            Some((
                "m_AnimationCompression".to_string(),
                self.m_animation_compression.to_value(),
            )),
            crate::opt_field("m_OptimizeGameObjects", &self.m_optimize_game_objects),
            crate::opt_field("m_ResampleRotations", &self.m_resample_rotations),
            crate::opt_field("m_ResampleCurves", &self.m_resample_curves),
            crate::opt_field("m_MaterialLocation", &self.m_material_location),
            Some((
                "m_AnimationRotationError".to_string(),
                self.m_animation_rotation_error.to_value(),
            )),
            crate::opt_field("m_MotionNodeName", &self.m_motion_node_name),
            crate::opt_field("m_Materials", &self.m_materials),
            Some((
                "m_AddColliders".to_string(),
                self.m_add_colliders.to_value(),
            )),
            Some((
                "m_AnimationPositionError".to_string(),
                self.m_animation_position_error.to_value(),
            )),
            crate::opt_field("m_AnimationImportErrors", &self.m_animation_import_errors),
            crate::opt_field("m_SearchTexturesGlobally", &self.m_search_textures_globally),
            Some((
                "m_UseFileUnits".to_string(),
                self.m_use_file_units.to_value(),
            )),
            Some((
                "m_AnimationScaleError".to_string(),
                self.m_animation_scale_error.to_value(),
            )),
            crate::opt_field(
                "m_AnimationImportWarnings",
                &self.m_animation_import_warnings,
            ),
            crate::opt_field("m_RigImportErrors", &self.m_rig_import_errors),
            Some((
                "m_AnimationWrapMode".to_string(),
                self.m_animation_wrap_mode.to_value(),
            )),
            crate::opt_field(
                "m_AnimationRetargetingWarnings",
                &self.m_animation_retargeting_warnings,
            ),
            crate::opt_field("m_RigImportWarnings", &self.m_rig_import_warnings),
            Some((
                "m_ClipAnimations".to_string(),
                self.m_clip_animations.to_value(),
            )),
            crate::opt_field(
                "m_AnimationDoRetargetingWarnings",
                &self.m_animation_do_retargeting_warnings,
            ),
            crate::opt_field("m_IsReadable", &self.m_is_readable),
            crate::opt_field(
                "m_ExtraExposedTransformPaths",
                &self.m_extra_exposed_transform_paths,
            ),
            crate::opt_field("m_LODScreenPercentages", &self.m_lod_screen_percentages),
            crate::opt_field(
                "m_RemoveConstantScaleCurves",
                &self.m_remove_constant_scale_curves,
            ),
            Some((
                "m_MeshCompression".to_string(),
                self.m_mesh_compression.to_value(),
            )),
            crate::opt_field(
                "m_MeshSettings.swapUVChannels",
                &self.m_mesh_settings_swap_uv_channels,
            ),
            crate::opt_field(
                "m_MeshSettings.generateSecondaryUV",
                &self.m_mesh_settings_generate_secondary_uv,
            ),
            crate::opt_field(
                "m_MeshSettings.secondaryUVAngleDistortion",
                &self.m_mesh_settings_secondary_uv_angle_distortion,
            ),
            crate::opt_field("swapUVChannels", &self.swap_uv_channels),
            crate::opt_field("m_ImportBlendShapes", &self.m_import_blend_shapes),
            crate::opt_field(
                "m_MeshSettings.secondaryUVAreaDistortion",
                &self.m_mesh_settings_secondary_uv_area_distortion,
            ),
            crate::opt_field("generateSecondaryUV", &self.generate_secondary_uv),
            crate::opt_field(
                "m_MeshSettings.secondaryUVHardAngle",
                &self.m_mesh_settings_secondary_uv_hard_angle,
            ),
            crate::opt_field(
                "m_ImportAnimatedCustomProperties",
                &self.m_import_animated_custom_properties,
            ),
            crate::opt_field(
                "m_MeshSettings.secondaryUVPackMargin",
                &self.m_mesh_settings_secondary_uv_pack_margin,
            ),
            crate::opt_field("optimizeMesh", &self.optimize_mesh),
            crate::opt_field("optimizeMeshForGPU", &self.optimize_mesh_for_gpu),
            crate::opt_field("m_ImportConstraints", &self.m_import_constraints),
            crate::opt_field(
                "m_MeshSettings.normalImportMode",
                &self.m_mesh_settings_normal_import_mode,
            ),
            crate::opt_field(
                "secondaryUVAngleDistortion",
                &self.secondary_uv_angle_distortion,
            ),
            crate::opt_field("m_ExtraUserProperties", &self.m_extra_user_properties),
            crate::opt_field(
                "m_MeshSettings.tangentImportMode",
                &self.m_mesh_settings_tangent_import_mode,
            ),
            crate::opt_field(
                "secondaryUVAreaDistortion",
                &self.secondary_uv_area_distortion,
            ),
            crate::opt_field("weldVertices", &self.weld_vertices),
            crate::opt_field("secondaryUVHardAngle", &self.secondary_uv_hard_angle),
            crate::opt_field("secondaryUVPackMargin", &self.secondary_uv_pack_margin),
            crate::opt_field("m_FirstImportVersion", &self.m_first_import_version),
            Some((
                "normalSmoothAngle".to_string(),
                self.normal_smooth_angle.to_value(),
            )),
            crate::opt_field("keepQuads", &self.keep_quads),
            crate::opt_field("splitTangentsAcrossUV", &self.split_tangents_across_uv),
            crate::opt_field("normalImportMode", &self.normal_import_mode),
            Some((
                "m_ImportedRoots".to_string(),
                self.m_imported_roots.to_value(),
            )),
            crate::opt_field("tangentImportMode", &self.tangent_import_mode),
            crate::opt_field("m_ImportVisibility", &self.m_import_visibility),
            Some((
                "m_HasExtraRoot".to_string(),
                self.m_has_extra_root.to_value(),
            )),
            crate::opt_field("m_ImportedTakeInfos", &self.m_imported_take_infos),
            crate::opt_field("m_ReferencedClips", &self.m_referenced_clips),
            crate::opt_field("m_ImportCameras", &self.m_import_cameras),
            crate::opt_field("m_UseFileScale", &self.m_use_file_scale),
            crate::opt_field("m_ImportLights", &self.m_import_lights),
            crate::opt_field("m_FileScale", &self.m_file_scale),
            crate::opt_field("m_ImportAnimation", &self.m_import_animation),
            crate::opt_field("m_CopyAvatar", &self.m_copy_avatar),
            crate::opt_field("m_UseSRGBMaterialColor", &self.m_use_srgb_material_color),
            crate::opt_field("m_HumanDescription", &self.m_human_description),
            crate::opt_field("m_SortHierarchyByName", &self.m_sort_hierarchy_by_name),
            crate::opt_field(
                "m_LastHumanDescriptionAvatarSource",
                &self.m_last_human_description_avatar_source,
            ),
            crate::opt_field("m_ImportPhysicalCameras", &self.m_import_physical_cameras),
            crate::opt_field("m_AdditionalBone", &self.m_additional_bone),
            crate::opt_field("m_AnimationType", &self.m_animation_type),
            crate::opt_field("m_UserData", &self.m_user_data),
            crate::opt_field("m_FileIdsGeneration", &self.m_file_ids_generation),
            crate::opt_field(
                "m_NodeNameCollisionStrategy",
                &self.m_node_name_collision_strategy,
            ),
            crate::opt_field("m_PreserveHierarchy", &self.m_preserve_hierarchy),
            crate::opt_field("indexFormat", &self.index_format),
            crate::opt_field("skinWeightsMode", &self.skin_weights_mode),
            crate::opt_field("normalCalculationMode", &self.normal_calculation_mode),
            crate::opt_field("maxBonesPerVertex", &self.max_bones_per_vertex),
            crate::opt_field("bakeAxisConversion", &self.bake_axis_conversion),
            crate::opt_field("m_AssetBundleName", &self.m_asset_bundle_name),
            crate::opt_field("minBoneWeight", &self.min_bone_weight),
            crate::opt_field("m_AssetBundleVariant", &self.m_asset_bundle_variant),
            crate::opt_field("meshOptimizationFlags", &self.mesh_optimization_flags),
            crate::opt_field("m_HumanoidOversampling", &self.m_humanoid_oversampling),
            crate::opt_field(
                "m_PreviousCalculatedGlobalScale",
                &self.m_previous_calculated_global_scale,
            ),
            crate::opt_field(
                "m_HasPreviousCalculatedGlobalScale",
                &self.m_has_previous_calculated_global_scale,
            ),
            crate::opt_field("m_FileScaleUnit", &self.m_file_scale_unit),
            crate::opt_field("optimizeBones", &self.optimize_bones),
            crate::opt_field("m_FileScaleFactor", &self.m_file_scale_factor),
            crate::opt_field("generateMeshLods", &self.generate_mesh_lods),
            crate::opt_field("meshLodGenerationFlags", &self.mesh_lod_generation_flags),
            crate::opt_field("maximumMeshLod", &self.maximum_mesh_lod),
            crate::opt_field("m_HasEmbeddedTextures", &self.m_has_embedded_textures),
            crate::opt_field("secondaryUVMarginMethod", &self.secondary_uv_margin_method),
            crate::opt_field("importUVs", &self.import_u_vs),
            crate::opt_field(
                "secondaryUVMinLightmapResolution",
                &self.secondary_uv_min_lightmap_resolution,
            ),
            crate::opt_field("importVertexColors", &self.import_vertex_colors),
            crate::opt_field(
                "secondaryUVMinObjectScale",
                &self.secondary_uv_min_object_scale,
            ),
            crate::opt_field(
                "legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes",
                &self.legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes,
            ),
            crate::opt_field(
                "blendShapeNormalImportMode",
                &self.blend_shape_normal_import_mode,
            ),
            crate::opt_field("normalSmoothingSource", &self.normal_smoothing_source),
            crate::opt_field(
                "m_StrictVertexDataChecks",
                &self.m_strict_vertex_data_checks,
            ),
            crate::opt_field(
                "m_AutoGenerateAvatarMappingIfUnspecified",
                &self.m_auto_generate_avatar_mapping_if_unspecified,
            ),
            crate::opt_field("m_AvatarSetup", &self.m_avatar_setup),
            crate::opt_field("m_ContainsAnimation", &self.m_contains_animation),
            crate::opt_field(
                "calculateBlendshapeNormalsDeltaFromImportedNormals",
                &self.calculate_blendshape_normals_delta_from_imported_normals,
            ),
            crate::opt_field(
                "m_AddHumanoidExtraRootOnlyWhenUsingAvatar",
                &self.m_add_humanoid_extra_root_only_when_using_avatar,
            ),
            crate::opt_field(
                "m_RemapMaterialsIfMaterialImportModeIsNone",
                &self.m_remap_materials_if_material_import_mode_is_none,
            ),
            crate::opt_field(
                "m_ImportBlendShapeDeformPercent",
                &self.m_import_blend_shape_deform_percent,
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
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_FileIDToRecycleName" => {
                    (self.m_file_id_to_recycle_name).inspect_value(path, offset, limit)
                }
                "m_InternalIDToNameTable" => {
                    (self.m_internal_id_to_name_table).inspect_value(path, offset, limit)
                }
                "m_OldHashIdentity" => {
                    (self.m_old_hash_identity).inspect_value(path, offset, limit)
                }
                "m_ImportMaterials" => (self.m_import_materials).inspect_value(path, offset, limit),
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_NewHashIdentity" => {
                    (self.m_new_hash_identity).inspect_value(path, offset, limit)
                }
                "m_MaterialName" => (self.m_material_name).inspect_value(path, offset, limit),
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
                "m_GenerateMaterials" => {
                    (self.m_generate_materials).inspect_value(path, offset, limit)
                }
                "m_MaterialSearch" => (self.m_material_search).inspect_value(path, offset, limit),
                "m_SupportsEmbeddedMaterials" => {
                    (self.m_supports_embedded_materials).inspect_value(path, offset, limit)
                }
                "m_GenerateAnimations" => {
                    (self.m_generate_animations).inspect_value(path, offset, limit)
                }
                "m_LegacyGenerateAnimations" => {
                    (self.m_legacy_generate_animations).inspect_value(path, offset, limit)
                }
                "m_MaterialImportMode" => {
                    (self.m_material_import_mode).inspect_value(path, offset, limit)
                }
                "m_GlobalScale" => (self.m_global_scale).inspect_value(path, offset, limit),
                "m_BakeSimulation" => (self.m_bake_simulation).inspect_value(path, offset, limit),
                "m_SplitAnimations" => (self.m_split_animations).inspect_value(path, offset, limit),
                "m_AnimationCompression" => {
                    (self.m_animation_compression).inspect_value(path, offset, limit)
                }
                "m_OptimizeGameObjects" => {
                    (self.m_optimize_game_objects).inspect_value(path, offset, limit)
                }
                "m_ResampleRotations" => {
                    (self.m_resample_rotations).inspect_value(path, offset, limit)
                }
                "m_ResampleCurves" => (self.m_resample_curves).inspect_value(path, offset, limit),
                "m_MaterialLocation" => {
                    (self.m_material_location).inspect_value(path, offset, limit)
                }
                "m_AnimationRotationError" => {
                    (self.m_animation_rotation_error).inspect_value(path, offset, limit)
                }
                "m_MotionNodeName" => (self.m_motion_node_name).inspect_value(path, offset, limit),
                "m_Materials" => (self.m_materials).inspect_value(path, offset, limit),
                "m_AddColliders" => (self.m_add_colliders).inspect_value(path, offset, limit),
                "m_AnimationPositionError" => {
                    (self.m_animation_position_error).inspect_value(path, offset, limit)
                }
                "m_AnimationImportErrors" => {
                    (self.m_animation_import_errors).inspect_value(path, offset, limit)
                }
                "m_SearchTexturesGlobally" => {
                    (self.m_search_textures_globally).inspect_value(path, offset, limit)
                }
                "m_UseFileUnits" => (self.m_use_file_units).inspect_value(path, offset, limit),
                "m_AnimationScaleError" => {
                    (self.m_animation_scale_error).inspect_value(path, offset, limit)
                }
                "m_AnimationImportWarnings" => {
                    (self.m_animation_import_warnings).inspect_value(path, offset, limit)
                }
                "m_RigImportErrors" => {
                    (self.m_rig_import_errors).inspect_value(path, offset, limit)
                }
                "m_AnimationWrapMode" => {
                    (self.m_animation_wrap_mode).inspect_value(path, offset, limit)
                }
                "m_AnimationRetargetingWarnings" => {
                    (self.m_animation_retargeting_warnings).inspect_value(path, offset, limit)
                }
                "m_RigImportWarnings" => {
                    (self.m_rig_import_warnings).inspect_value(path, offset, limit)
                }
                "m_ClipAnimations" => (self.m_clip_animations).inspect_value(path, offset, limit),
                "m_AnimationDoRetargetingWarnings" => {
                    (self.m_animation_do_retargeting_warnings).inspect_value(path, offset, limit)
                }
                "m_IsReadable" => (self.m_is_readable).inspect_value(path, offset, limit),
                "m_ExtraExposedTransformPaths" => {
                    (self.m_extra_exposed_transform_paths).inspect_value(path, offset, limit)
                }
                "m_LODScreenPercentages" => {
                    (self.m_lod_screen_percentages).inspect_value(path, offset, limit)
                }
                "m_RemoveConstantScaleCurves" => {
                    (self.m_remove_constant_scale_curves).inspect_value(path, offset, limit)
                }
                "m_MeshCompression" => (self.m_mesh_compression).inspect_value(path, offset, limit),
                "m_MeshSettings.swapUVChannels" => {
                    (self.m_mesh_settings_swap_uv_channels).inspect_value(path, offset, limit)
                }
                "m_MeshSettings.generateSecondaryUV" => {
                    (self.m_mesh_settings_generate_secondary_uv).inspect_value(path, offset, limit)
                }
                "m_MeshSettings.secondaryUVAngleDistortion" => (self
                    .m_mesh_settings_secondary_uv_angle_distortion)
                    .inspect_value(path, offset, limit),
                "swapUVChannels" => (self.swap_uv_channels).inspect_value(path, offset, limit),
                "m_ImportBlendShapes" => {
                    (self.m_import_blend_shapes).inspect_value(path, offset, limit)
                }
                "m_MeshSettings.secondaryUVAreaDistortion" => (self
                    .m_mesh_settings_secondary_uv_area_distortion)
                    .inspect_value(path, offset, limit),
                "generateSecondaryUV" => {
                    (self.generate_secondary_uv).inspect_value(path, offset, limit)
                }
                "m_MeshSettings.secondaryUVHardAngle" => (self
                    .m_mesh_settings_secondary_uv_hard_angle)
                    .inspect_value(path, offset, limit),
                "m_ImportAnimatedCustomProperties" => {
                    (self.m_import_animated_custom_properties).inspect_value(path, offset, limit)
                }
                "m_MeshSettings.secondaryUVPackMargin" => (self
                    .m_mesh_settings_secondary_uv_pack_margin)
                    .inspect_value(path, offset, limit),
                "optimizeMesh" => (self.optimize_mesh).inspect_value(path, offset, limit),
                "optimizeMeshForGPU" => {
                    (self.optimize_mesh_for_gpu).inspect_value(path, offset, limit)
                }
                "m_ImportConstraints" => {
                    (self.m_import_constraints).inspect_value(path, offset, limit)
                }
                "m_MeshSettings.normalImportMode" => {
                    (self.m_mesh_settings_normal_import_mode).inspect_value(path, offset, limit)
                }
                "secondaryUVAngleDistortion" => {
                    (self.secondary_uv_angle_distortion).inspect_value(path, offset, limit)
                }
                "m_ExtraUserProperties" => {
                    (self.m_extra_user_properties).inspect_value(path, offset, limit)
                }
                "m_MeshSettings.tangentImportMode" => {
                    (self.m_mesh_settings_tangent_import_mode).inspect_value(path, offset, limit)
                }
                "secondaryUVAreaDistortion" => {
                    (self.secondary_uv_area_distortion).inspect_value(path, offset, limit)
                }
                "weldVertices" => (self.weld_vertices).inspect_value(path, offset, limit),
                "secondaryUVHardAngle" => {
                    (self.secondary_uv_hard_angle).inspect_value(path, offset, limit)
                }
                "secondaryUVPackMargin" => {
                    (self.secondary_uv_pack_margin).inspect_value(path, offset, limit)
                }
                "m_FirstImportVersion" => {
                    (self.m_first_import_version).inspect_value(path, offset, limit)
                }
                "normalSmoothAngle" => {
                    (self.normal_smooth_angle).inspect_value(path, offset, limit)
                }
                "keepQuads" => (self.keep_quads).inspect_value(path, offset, limit),
                "splitTangentsAcrossUV" => {
                    (self.split_tangents_across_uv).inspect_value(path, offset, limit)
                }
                "normalImportMode" => (self.normal_import_mode).inspect_value(path, offset, limit),
                "m_ImportedRoots" => (self.m_imported_roots).inspect_value(path, offset, limit),
                "tangentImportMode" => {
                    (self.tangent_import_mode).inspect_value(path, offset, limit)
                }
                "m_ImportVisibility" => {
                    (self.m_import_visibility).inspect_value(path, offset, limit)
                }
                "m_HasExtraRoot" => (self.m_has_extra_root).inspect_value(path, offset, limit),
                "m_ImportedTakeInfos" => {
                    (self.m_imported_take_infos).inspect_value(path, offset, limit)
                }
                "m_ReferencedClips" => (self.m_referenced_clips).inspect_value(path, offset, limit),
                "m_ImportCameras" => (self.m_import_cameras).inspect_value(path, offset, limit),
                "m_UseFileScale" => (self.m_use_file_scale).inspect_value(path, offset, limit),
                "m_ImportLights" => (self.m_import_lights).inspect_value(path, offset, limit),
                "m_FileScale" => (self.m_file_scale).inspect_value(path, offset, limit),
                "m_ImportAnimation" => (self.m_import_animation).inspect_value(path, offset, limit),
                "m_CopyAvatar" => (self.m_copy_avatar).inspect_value(path, offset, limit),
                "m_UseSRGBMaterialColor" => {
                    (self.m_use_srgb_material_color).inspect_value(path, offset, limit)
                }
                "m_HumanDescription" => {
                    (self.m_human_description).inspect_value(path, offset, limit)
                }
                "m_SortHierarchyByName" => {
                    (self.m_sort_hierarchy_by_name).inspect_value(path, offset, limit)
                }
                "m_LastHumanDescriptionAvatarSource" => {
                    (self.m_last_human_description_avatar_source).inspect_value(path, offset, limit)
                }
                "m_ImportPhysicalCameras" => {
                    (self.m_import_physical_cameras).inspect_value(path, offset, limit)
                }
                "m_AdditionalBone" => (self.m_additional_bone).inspect_value(path, offset, limit),
                "m_AnimationType" => (self.m_animation_type).inspect_value(path, offset, limit),
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_FileIdsGeneration" => {
                    (self.m_file_ids_generation).inspect_value(path, offset, limit)
                }
                "m_NodeNameCollisionStrategy" => {
                    (self.m_node_name_collision_strategy).inspect_value(path, offset, limit)
                }
                "m_PreserveHierarchy" => {
                    (self.m_preserve_hierarchy).inspect_value(path, offset, limit)
                }
                "indexFormat" => (self.index_format).inspect_value(path, offset, limit),
                "skinWeightsMode" => (self.skin_weights_mode).inspect_value(path, offset, limit),
                "normalCalculationMode" => {
                    (self.normal_calculation_mode).inspect_value(path, offset, limit)
                }
                "maxBonesPerVertex" => {
                    (self.max_bones_per_vertex).inspect_value(path, offset, limit)
                }
                "bakeAxisConversion" => {
                    (self.bake_axis_conversion).inspect_value(path, offset, limit)
                }
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "minBoneWeight" => (self.min_bone_weight).inspect_value(path, offset, limit),
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                "meshOptimizationFlags" => {
                    (self.mesh_optimization_flags).inspect_value(path, offset, limit)
                }
                "m_HumanoidOversampling" => {
                    (self.m_humanoid_oversampling).inspect_value(path, offset, limit)
                }
                "m_PreviousCalculatedGlobalScale" => {
                    (self.m_previous_calculated_global_scale).inspect_value(path, offset, limit)
                }
                "m_HasPreviousCalculatedGlobalScale" => {
                    (self.m_has_previous_calculated_global_scale).inspect_value(path, offset, limit)
                }
                "m_FileScaleUnit" => (self.m_file_scale_unit).inspect_value(path, offset, limit),
                "optimizeBones" => (self.optimize_bones).inspect_value(path, offset, limit),
                "m_FileScaleFactor" => {
                    (self.m_file_scale_factor).inspect_value(path, offset, limit)
                }
                "generateMeshLods" => (self.generate_mesh_lods).inspect_value(path, offset, limit),
                "meshLodGenerationFlags" => {
                    (self.mesh_lod_generation_flags).inspect_value(path, offset, limit)
                }
                "maximumMeshLod" => (self.maximum_mesh_lod).inspect_value(path, offset, limit),
                "m_HasEmbeddedTextures" => {
                    (self.m_has_embedded_textures).inspect_value(path, offset, limit)
                }
                "secondaryUVMarginMethod" => {
                    (self.secondary_uv_margin_method).inspect_value(path, offset, limit)
                }
                "importUVs" => (self.import_u_vs).inspect_value(path, offset, limit),
                "secondaryUVMinLightmapResolution" => {
                    (self.secondary_uv_min_lightmap_resolution).inspect_value(path, offset, limit)
                }
                "importVertexColors" => {
                    (self.import_vertex_colors).inspect_value(path, offset, limit)
                }
                "secondaryUVMinObjectScale" => {
                    (self.secondary_uv_min_object_scale).inspect_value(path, offset, limit)
                }
                "legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes" => (self
                    .legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes)
                    .inspect_value(path, offset, limit),
                "blendShapeNormalImportMode" => {
                    (self.blend_shape_normal_import_mode).inspect_value(path, offset, limit)
                }
                "normalSmoothingSource" => {
                    (self.normal_smoothing_source).inspect_value(path, offset, limit)
                }
                "m_StrictVertexDataChecks" => {
                    (self.m_strict_vertex_data_checks).inspect_value(path, offset, limit)
                }
                "m_AutoGenerateAvatarMappingIfUnspecified" => (self
                    .m_auto_generate_avatar_mapping_if_unspecified)
                    .inspect_value(path, offset, limit),
                "m_AvatarSetup" => (self.m_avatar_setup).inspect_value(path, offset, limit),
                "m_ContainsAnimation" => {
                    (self.m_contains_animation).inspect_value(path, offset, limit)
                }
                "calculateBlendshapeNormalsDeltaFromImportedNormals" => (self
                    .calculate_blendshape_normals_delta_from_imported_normals)
                    .inspect_value(path, offset, limit),
                "m_AddHumanoidExtraRootOnlyWhenUsingAvatar" => (self
                    .m_add_humanoid_extra_root_only_when_using_avatar)
                    .inspect_value(path, offset, limit),
                "m_RemapMaterialsIfMaterialImportModeIsNone" => (self
                    .m_remap_materials_if_material_import_mode_is_none)
                    .inspect_value(path, offset, limit),
                "m_ImportBlendShapeDeformPercent" => {
                    (self.m_import_blend_shape_deform_percent).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_file_id_to_recycle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_FileIDToRecycleName".to_string(), v)),
            ((self.m_internal_id_to_name_table).inspect_value(path, offset, limit))
                .map(|v| ("m_InternalIDToNameTable".to_string(), v)),
            ((self.m_old_hash_identity).inspect_value(path, offset, limit))
                .map(|v| ("m_OldHashIdentity".to_string(), v)),
            ((self.m_import_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportMaterials".to_string(), v)),
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_new_hash_identity).inspect_value(path, offset, limit))
                .map(|v| ("m_NewHashIdentity".to_string(), v)),
            ((self.m_material_name).inspect_value(path, offset, limit))
                .map(|v| ("m_MaterialName".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
            ((self.m_generate_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateMaterials".to_string(), v)),
            ((self.m_material_search).inspect_value(path, offset, limit))
                .map(|v| ("m_MaterialSearch".to_string(), v)),
            ((self.m_supports_embedded_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_SupportsEmbeddedMaterials".to_string(), v)),
            ((self.m_generate_animations).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateAnimations".to_string(), v)),
            ((self.m_legacy_generate_animations).inspect_value(path, offset, limit))
                .map(|v| ("m_LegacyGenerateAnimations".to_string(), v)),
            ((self.m_material_import_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_MaterialImportMode".to_string(), v)),
            ((self.m_global_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_GlobalScale".to_string(), v)),
            ((self.m_bake_simulation).inspect_value(path, offset, limit))
                .map(|v| ("m_BakeSimulation".to_string(), v)),
            ((self.m_split_animations).inspect_value(path, offset, limit))
                .map(|v| ("m_SplitAnimations".to_string(), v)),
            ((self.m_animation_compression).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationCompression".to_string(), v)),
            ((self.m_optimize_game_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_OptimizeGameObjects".to_string(), v)),
            ((self.m_resample_rotations).inspect_value(path, offset, limit))
                .map(|v| ("m_ResampleRotations".to_string(), v)),
            ((self.m_resample_curves).inspect_value(path, offset, limit))
                .map(|v| ("m_ResampleCurves".to_string(), v)),
            ((self.m_material_location).inspect_value(path, offset, limit))
                .map(|v| ("m_MaterialLocation".to_string(), v)),
            ((self.m_animation_rotation_error).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationRotationError".to_string(), v)),
            ((self.m_motion_node_name).inspect_value(path, offset, limit))
                .map(|v| ("m_MotionNodeName".to_string(), v)),
            ((self.m_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_Materials".to_string(), v)),
            ((self.m_add_colliders).inspect_value(path, offset, limit))
                .map(|v| ("m_AddColliders".to_string(), v)),
            ((self.m_animation_position_error).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationPositionError".to_string(), v)),
            ((self.m_animation_import_errors).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationImportErrors".to_string(), v)),
            ((self.m_search_textures_globally).inspect_value(path, offset, limit))
                .map(|v| ("m_SearchTexturesGlobally".to_string(), v)),
            ((self.m_use_file_units).inspect_value(path, offset, limit))
                .map(|v| ("m_UseFileUnits".to_string(), v)),
            ((self.m_animation_scale_error).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationScaleError".to_string(), v)),
            ((self.m_animation_import_warnings).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationImportWarnings".to_string(), v)),
            ((self.m_rig_import_errors).inspect_value(path, offset, limit))
                .map(|v| ("m_RigImportErrors".to_string(), v)),
            ((self.m_animation_wrap_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationWrapMode".to_string(), v)),
            ((self.m_animation_retargeting_warnings).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationRetargetingWarnings".to_string(), v)),
            ((self.m_rig_import_warnings).inspect_value(path, offset, limit))
                .map(|v| ("m_RigImportWarnings".to_string(), v)),
            ((self.m_clip_animations).inspect_value(path, offset, limit))
                .map(|v| ("m_ClipAnimations".to_string(), v)),
            ((self.m_animation_do_retargeting_warnings).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationDoRetargetingWarnings".to_string(), v)),
            ((self.m_is_readable).inspect_value(path, offset, limit))
                .map(|v| ("m_IsReadable".to_string(), v)),
            ((self.m_extra_exposed_transform_paths).inspect_value(path, offset, limit))
                .map(|v| ("m_ExtraExposedTransformPaths".to_string(), v)),
            ((self.m_lod_screen_percentages).inspect_value(path, offset, limit))
                .map(|v| ("m_LODScreenPercentages".to_string(), v)),
            ((self.m_remove_constant_scale_curves).inspect_value(path, offset, limit))
                .map(|v| ("m_RemoveConstantScaleCurves".to_string(), v)),
            ((self.m_mesh_compression).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshCompression".to_string(), v)),
            ((self.m_mesh_settings_swap_uv_channels).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshSettings.swapUVChannels".to_string(), v)),
            ((self.m_mesh_settings_generate_secondary_uv).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshSettings.generateSecondaryUV".to_string(), v)),
            ((self.m_mesh_settings_secondary_uv_angle_distortion)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_MeshSettings.secondaryUVAngleDistortion".to_string(), v)),
            ((self.swap_uv_channels).inspect_value(path, offset, limit))
                .map(|v| ("swapUVChannels".to_string(), v)),
            ((self.m_import_blend_shapes).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportBlendShapes".to_string(), v)),
            ((self.m_mesh_settings_secondary_uv_area_distortion)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_MeshSettings.secondaryUVAreaDistortion".to_string(), v)),
            ((self.generate_secondary_uv).inspect_value(path, offset, limit))
                .map(|v| ("generateSecondaryUV".to_string(), v)),
            ((self.m_mesh_settings_secondary_uv_hard_angle).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshSettings.secondaryUVHardAngle".to_string(), v)),
            ((self.m_import_animated_custom_properties).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportAnimatedCustomProperties".to_string(), v)),
            ((self.m_mesh_settings_secondary_uv_pack_margin).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshSettings.secondaryUVPackMargin".to_string(), v)),
            ((self.optimize_mesh).inspect_value(path, offset, limit))
                .map(|v| ("optimizeMesh".to_string(), v)),
            ((self.optimize_mesh_for_gpu).inspect_value(path, offset, limit))
                .map(|v| ("optimizeMeshForGPU".to_string(), v)),
            ((self.m_import_constraints).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportConstraints".to_string(), v)),
            ((self.m_mesh_settings_normal_import_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshSettings.normalImportMode".to_string(), v)),
            ((self.secondary_uv_angle_distortion).inspect_value(path, offset, limit))
                .map(|v| ("secondaryUVAngleDistortion".to_string(), v)),
            ((self.m_extra_user_properties).inspect_value(path, offset, limit))
                .map(|v| ("m_ExtraUserProperties".to_string(), v)),
            ((self.m_mesh_settings_tangent_import_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshSettings.tangentImportMode".to_string(), v)),
            ((self.secondary_uv_area_distortion).inspect_value(path, offset, limit))
                .map(|v| ("secondaryUVAreaDistortion".to_string(), v)),
            ((self.weld_vertices).inspect_value(path, offset, limit))
                .map(|v| ("weldVertices".to_string(), v)),
            ((self.secondary_uv_hard_angle).inspect_value(path, offset, limit))
                .map(|v| ("secondaryUVHardAngle".to_string(), v)),
            ((self.secondary_uv_pack_margin).inspect_value(path, offset, limit))
                .map(|v| ("secondaryUVPackMargin".to_string(), v)),
            ((self.m_first_import_version).inspect_value(path, offset, limit))
                .map(|v| ("m_FirstImportVersion".to_string(), v)),
            ((self.normal_smooth_angle).inspect_value(path, offset, limit))
                .map(|v| ("normalSmoothAngle".to_string(), v)),
            ((self.keep_quads).inspect_value(path, offset, limit))
                .map(|v| ("keepQuads".to_string(), v)),
            ((self.split_tangents_across_uv).inspect_value(path, offset, limit))
                .map(|v| ("splitTangentsAcrossUV".to_string(), v)),
            ((self.normal_import_mode).inspect_value(path, offset, limit))
                .map(|v| ("normalImportMode".to_string(), v)),
            ((self.m_imported_roots).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportedRoots".to_string(), v)),
            ((self.tangent_import_mode).inspect_value(path, offset, limit))
                .map(|v| ("tangentImportMode".to_string(), v)),
            ((self.m_import_visibility).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportVisibility".to_string(), v)),
            ((self.m_has_extra_root).inspect_value(path, offset, limit))
                .map(|v| ("m_HasExtraRoot".to_string(), v)),
            ((self.m_imported_take_infos).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportedTakeInfos".to_string(), v)),
            ((self.m_referenced_clips).inspect_value(path, offset, limit))
                .map(|v| ("m_ReferencedClips".to_string(), v)),
            ((self.m_import_cameras).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportCameras".to_string(), v)),
            ((self.m_use_file_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_UseFileScale".to_string(), v)),
            ((self.m_import_lights).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportLights".to_string(), v)),
            ((self.m_file_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_FileScale".to_string(), v)),
            ((self.m_import_animation).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportAnimation".to_string(), v)),
            ((self.m_copy_avatar).inspect_value(path, offset, limit))
                .map(|v| ("m_CopyAvatar".to_string(), v)),
            ((self.m_use_srgb_material_color).inspect_value(path, offset, limit))
                .map(|v| ("m_UseSRGBMaterialColor".to_string(), v)),
            ((self.m_human_description).inspect_value(path, offset, limit))
                .map(|v| ("m_HumanDescription".to_string(), v)),
            ((self.m_sort_hierarchy_by_name).inspect_value(path, offset, limit))
                .map(|v| ("m_SortHierarchyByName".to_string(), v)),
            ((self.m_last_human_description_avatar_source).inspect_value(path, offset, limit))
                .map(|v| ("m_LastHumanDescriptionAvatarSource".to_string(), v)),
            ((self.m_import_physical_cameras).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportPhysicalCameras".to_string(), v)),
            ((self.m_additional_bone).inspect_value(path, offset, limit))
                .map(|v| ("m_AdditionalBone".to_string(), v)),
            ((self.m_animation_type).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationType".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_file_ids_generation).inspect_value(path, offset, limit))
                .map(|v| ("m_FileIdsGeneration".to_string(), v)),
            ((self.m_node_name_collision_strategy).inspect_value(path, offset, limit))
                .map(|v| ("m_NodeNameCollisionStrategy".to_string(), v)),
            ((self.m_preserve_hierarchy).inspect_value(path, offset, limit))
                .map(|v| ("m_PreserveHierarchy".to_string(), v)),
            ((self.index_format).inspect_value(path, offset, limit))
                .map(|v| ("indexFormat".to_string(), v)),
            ((self.skin_weights_mode).inspect_value(path, offset, limit))
                .map(|v| ("skinWeightsMode".to_string(), v)),
            ((self.normal_calculation_mode).inspect_value(path, offset, limit))
                .map(|v| ("normalCalculationMode".to_string(), v)),
            ((self.max_bones_per_vertex).inspect_value(path, offset, limit))
                .map(|v| ("maxBonesPerVertex".to_string(), v)),
            ((self.bake_axis_conversion).inspect_value(path, offset, limit))
                .map(|v| ("bakeAxisConversion".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.min_bone_weight).inspect_value(path, offset, limit))
                .map(|v| ("minBoneWeight".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
            ((self.mesh_optimization_flags).inspect_value(path, offset, limit))
                .map(|v| ("meshOptimizationFlags".to_string(), v)),
            ((self.m_humanoid_oversampling).inspect_value(path, offset, limit))
                .map(|v| ("m_HumanoidOversampling".to_string(), v)),
            ((self.m_previous_calculated_global_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_PreviousCalculatedGlobalScale".to_string(), v)),
            ((self.m_has_previous_calculated_global_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_HasPreviousCalculatedGlobalScale".to_string(), v)),
            ((self.m_file_scale_unit).inspect_value(path, offset, limit))
                .map(|v| ("m_FileScaleUnit".to_string(), v)),
            ((self.optimize_bones).inspect_value(path, offset, limit))
                .map(|v| ("optimizeBones".to_string(), v)),
            ((self.m_file_scale_factor).inspect_value(path, offset, limit))
                .map(|v| ("m_FileScaleFactor".to_string(), v)),
            ((self.generate_mesh_lods).inspect_value(path, offset, limit))
                .map(|v| ("generateMeshLods".to_string(), v)),
            ((self.mesh_lod_generation_flags).inspect_value(path, offset, limit))
                .map(|v| ("meshLodGenerationFlags".to_string(), v)),
            ((self.maximum_mesh_lod).inspect_value(path, offset, limit))
                .map(|v| ("maximumMeshLod".to_string(), v)),
            ((self.m_has_embedded_textures).inspect_value(path, offset, limit))
                .map(|v| ("m_HasEmbeddedTextures".to_string(), v)),
            ((self.secondary_uv_margin_method).inspect_value(path, offset, limit))
                .map(|v| ("secondaryUVMarginMethod".to_string(), v)),
            ((self.import_u_vs).inspect_value(path, offset, limit))
                .map(|v| ("importUVs".to_string(), v)),
            ((self.secondary_uv_min_lightmap_resolution).inspect_value(path, offset, limit))
                .map(|v| ("secondaryUVMinLightmapResolution".to_string(), v)),
            ((self.import_vertex_colors).inspect_value(path, offset, limit))
                .map(|v| ("importVertexColors".to_string(), v)),
            ((self.secondary_uv_min_object_scale).inspect_value(path, offset, limit))
                .map(|v| ("secondaryUVMinObjectScale".to_string(), v)),
            ((self.legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes)
                .inspect_value(path, offset, limit))
            .map(|v| {
                (
                    "legacyComputeAllNormalsFromSmoothingGroupsWhenMeshHasBlendShapes".to_string(),
                    v,
                )
            }),
            ((self.blend_shape_normal_import_mode).inspect_value(path, offset, limit))
                .map(|v| ("blendShapeNormalImportMode".to_string(), v)),
            ((self.normal_smoothing_source).inspect_value(path, offset, limit))
                .map(|v| ("normalSmoothingSource".to_string(), v)),
            ((self.m_strict_vertex_data_checks).inspect_value(path, offset, limit))
                .map(|v| ("m_StrictVertexDataChecks".to_string(), v)),
            ((self.m_auto_generate_avatar_mapping_if_unspecified)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_AutoGenerateAvatarMappingIfUnspecified".to_string(), v)),
            ((self.m_avatar_setup).inspect_value(path, offset, limit))
                .map(|v| ("m_AvatarSetup".to_string(), v)),
            ((self.m_contains_animation).inspect_value(path, offset, limit))
                .map(|v| ("m_ContainsAnimation".to_string(), v)),
            ((self.calculate_blendshape_normals_delta_from_imported_normals)
                .inspect_value(path, offset, limit))
            .map(|v| {
                (
                    "calculateBlendshapeNormalsDeltaFromImportedNormals".to_string(),
                    v,
                )
            }),
            ((self.m_add_humanoid_extra_root_only_when_using_avatar)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_AddHumanoidExtraRootOnlyWhenUsingAvatar".to_string(), v)),
            ((self.m_remap_materials_if_material_import_mode_is_none)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_RemapMaterialsIfMaterialImportModeIsNone".to_string(), v)),
            ((self.m_import_blend_shape_deform_percent).inspect_value(path, offset, limit))
                .map(|v| ("m_ImportBlendShapeDeformPercent".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
