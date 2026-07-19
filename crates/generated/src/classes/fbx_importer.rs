//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::clip_animation_info::ClipAnimationInfo;
use crate::subclasses::guid::Guid;
use crate::subclasses::human_description::HumanDescription;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::take_info::TakeInfo;

/// 版本枚举：FbxImporterMFileIdToRecycleNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMFileIdToRecycleNameVariant {
    Default(Vec<(i32, String)>),
    V5_0(Vec<(i64, String)>),
}

impl crate::Parse for FbxImporterMFileIdToRecycleNameVariant {
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

/// 版本枚举：FbxImporterMImportMaterialsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMImportMaterialsVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for FbxImporterMImportMaterialsVariant {
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

/// 版本枚举：FbxImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for FbxImporterMExternalObjectsVariant {
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

/// 版本枚举：FbxImporterMBakeSimulationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMBakeSimulationVariant {
    Default(bool),
    V4_0(bool),
    V4_3(bool),
}

impl crate::Parse for FbxImporterMBakeSimulationVariant {
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

/// 版本枚举：FbxImporterMSplitAnimationsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMSplitAnimationsVariant {
    Default(bool),
    V3_5(bool),
}

impl crate::Parse for FbxImporterMSplitAnimationsVariant {
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

/// 版本枚举：FbxImporterMOptimizeGameObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMOptimizeGameObjectsVariant {
    Default(bool),
    V2021_2(bool),
}

impl crate::Parse for FbxImporterMOptimizeGameObjectsVariant {
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

/// 版本枚举：FbxImporterMUseFileUnitsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMUseFileUnitsVariant {
    Default(bool),
    V3_5(bool),
}

impl crate::Parse for FbxImporterMUseFileUnitsVariant {
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

/// 版本枚举：FbxImporterMClipAnimationsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMClipAnimationsVariant {
    Default(Vec<ClipAnimationInfo>),
    V2017_1(Vec<ClipAnimationInfo>),
}

impl crate::Parse for FbxImporterMClipAnimationsVariant {
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

/// 版本枚举：FbxImporterMAnimationDoRetargetingWarningsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMAnimationDoRetargetingWarningsVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for FbxImporterMAnimationDoRetargetingWarningsVariant {
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

/// 版本枚举：FbxImporterMExtraExposedTransformPathsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMExtraExposedTransformPathsVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for FbxImporterMExtraExposedTransformPathsVariant {
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

/// 版本枚举：FbxImporterMLodScreenPercentagesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMLodScreenPercentagesVariant {
    Default(Vec<f32>),
    V2017_1(Vec<f32>),
}

impl crate::Parse for FbxImporterMLodScreenPercentagesVariant {
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

/// 版本枚举：FbxImporterMImportAnimatedCustomPropertiesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMImportAnimatedCustomPropertiesVariant {
    Default(bool),
    V2018_1(bool),
}

impl crate::Parse for FbxImporterMImportAnimatedCustomPropertiesVariant {
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

/// 版本枚举：FbxImporterOptimizeMeshForGpuVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterOptimizeMeshForGpuVariant {
    Default(bool),
    V4_1(bool),
}

impl crate::Parse for FbxImporterOptimizeMeshForGpuVariant {
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

/// 版本枚举：FbxImporterWeldVerticesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterWeldVerticesVariant {
    Default(bool),
    V2017_3(bool),
}

impl crate::Parse for FbxImporterWeldVerticesVariant {
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

/// 版本枚举：FbxImporterNormalSmoothAngleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterNormalSmoothAngleVariant {
    Default(f32),
    V5_3(f32),
}

impl crate::Parse for FbxImporterNormalSmoothAngleVariant {
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

/// 版本枚举：FbxImporterMImportedRootsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMImportedRootsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for FbxImporterMImportedRootsVariant {
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

/// 版本枚举：FbxImporterTangentImportModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterTangentImportModeVariant {
    Default(i32),
    V2017_1(i32),
}

impl crate::Parse for FbxImporterTangentImportModeVariant {
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

/// 版本枚举：FbxImporterMHasExtraRootVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMHasExtraRootVariant {
    Default(bool),
    V4_3(bool),
    V2017_2(bool),
}

impl crate::Parse for FbxImporterMHasExtraRootVariant {
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

/// 版本枚举：FbxImporterMImportedTakeInfosVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMImportedTakeInfosVariant {
    Default(Vec<TakeInfo>),
    V2017_1(Vec<TakeInfo>),
}

impl crate::Parse for FbxImporterMImportedTakeInfosVariant {
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

/// 版本枚举：FbxImporterMReferencedClipsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMReferencedClipsVariant {
    Default(Vec<Guid>),
    V2017_1(Vec<Guid>),
    V6000_0(Vec<Guid>),
}

impl crate::Parse for FbxImporterMReferencedClipsVariant {
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

/// 版本枚举：FbxImporterMFileScaleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMFileScaleVariant {
    Default(f32),
    V2018_3(f32),
}

impl crate::Parse for FbxImporterMFileScaleVariant {
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

/// 版本枚举：FbxImporterMImportAnimationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMImportAnimationVariant {
    Default(bool),
    V2019_3(bool),
}

impl crate::Parse for FbxImporterMImportAnimationVariant {
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

/// 版本枚举：FbxImporterMAnimationTypeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMAnimationTypeVariant {
    Default(i32),
    V4_3_3(i32),
}

impl crate::Parse for FbxImporterMAnimationTypeVariant {
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

/// 版本枚举：FbxImporterNormalCalculationModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterNormalCalculationModeVariant {
    Default(i32),
    V2018_3(i32),
}

impl crate::Parse for FbxImporterNormalCalculationModeVariant {
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

/// 版本枚举：FbxImporterOptimizeBonesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterOptimizeBonesVariant {
    Default(bool),
    V6000_2(bool),
}

impl crate::Parse for FbxImporterOptimizeBonesVariant {
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

/// 版本枚举：FbxImporterMHasEmbeddedTexturesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMHasEmbeddedTexturesVariant {
    Default(bool),
    V2023_1(bool),
}

impl crate::Parse for FbxImporterMHasEmbeddedTexturesVariant {
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

/// 版本枚举：FbxImporterNormalSmoothingSourceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterNormalSmoothingSourceVariant {
    Default(i32),
    V6000_5(i32),
}

impl crate::Parse for FbxImporterNormalSmoothingSourceVariant {
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

/// 版本枚举：FbxImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant {
    Default(bool),
    V2020_3_37(bool),
    V2021_1(bool),
    V2021_3_5(bool),
}

impl crate::Parse for FbxImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant {
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

/// 版本枚举：FbxImporterMImportBlendShapeDeformPercentVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FbxImporterMImportBlendShapeDeformPercentVariant {
    Default(bool),
    V2022_1_5(bool),
}

impl crate::Parse for FbxImporterMImportBlendShapeDeformPercentVariant {
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
#[derive(Debug, Clone)]
pub struct FbxImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<FbxImporterMFileIdToRecycleNameVariant>,
    pub m_internal_id_to_name_table: Option<Vec<((i32, i64), String)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_import_materials: Option<FbxImporterMImportMaterialsVariant>,
    pub m_external_objects: Option<FbxImporterMExternalObjectsVariant>,
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
    pub m_bake_simulation: FbxImporterMBakeSimulationVariant,
    pub m_split_animations: Option<FbxImporterMSplitAnimationsVariant>,
    pub m_animation_compression: i32,
    pub m_optimize_game_objects: Option<FbxImporterMOptimizeGameObjectsVariant>,
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
    pub m_use_file_units: FbxImporterMUseFileUnitsVariant,
    pub m_animation_scale_error: f32,
    pub m_animation_import_warnings: Option<String>,
    pub m_rig_import_errors: Option<String>,
    pub m_animation_wrap_mode: i32,
    pub m_animation_retargeting_warnings: Option<String>,
    pub m_rig_import_warnings: Option<String>,
    pub m_clip_animations: FbxImporterMClipAnimationsVariant,
    pub m_animation_do_retargeting_warnings:
        Option<FbxImporterMAnimationDoRetargetingWarningsVariant>,
    pub m_is_readable: Option<bool>,
    pub m_extra_exposed_transform_paths: Option<FbxImporterMExtraExposedTransformPathsVariant>,
    pub m_lod_screen_percentages: Option<FbxImporterMLodScreenPercentagesVariant>,
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
        Option<FbxImporterMImportAnimatedCustomPropertiesVariant>,
    pub m_mesh_settings_secondary_uv_pack_margin: Option<f32>,
    pub optimize_mesh: Option<bool>,
    pub optimize_mesh_for_gpu: Option<FbxImporterOptimizeMeshForGpuVariant>,
    pub m_import_constraints: Option<bool>,
    pub m_mesh_settings_normal_import_mode: Option<i32>,
    pub secondary_uv_angle_distortion: Option<f32>,
    pub m_extra_user_properties: Option<Vec<String>>,
    pub m_mesh_settings_tangent_import_mode: Option<i32>,
    pub secondary_uv_area_distortion: Option<f32>,
    pub weld_vertices: Option<FbxImporterWeldVerticesVariant>,
    pub secondary_uv_hard_angle: Option<f32>,
    pub secondary_uv_pack_margin: Option<f32>,
    pub m_first_import_version: Option<i32>,
    pub normal_smooth_angle: FbxImporterNormalSmoothAngleVariant,
    pub keep_quads: Option<bool>,
    pub split_tangents_across_uv: Option<bool>,
    pub normal_import_mode: Option<i32>,
    pub m_imported_roots: FbxImporterMImportedRootsVariant,
    pub tangent_import_mode: Option<FbxImporterTangentImportModeVariant>,
    pub m_import_visibility: Option<bool>,
    pub m_has_extra_root: FbxImporterMHasExtraRootVariant,
    pub m_imported_take_infos: Option<FbxImporterMImportedTakeInfosVariant>,
    pub m_referenced_clips: Option<FbxImporterMReferencedClipsVariant>,
    pub m_import_cameras: Option<bool>,
    pub m_use_file_scale: Option<bool>,
    pub m_import_lights: Option<bool>,
    pub m_file_scale: Option<FbxImporterMFileScaleVariant>,
    pub m_import_animation: Option<FbxImporterMImportAnimationVariant>,
    pub m_copy_avatar: Option<bool>,
    pub m_use_srgb_material_color: Option<bool>,
    pub m_human_description: Option<HumanDescription>,
    pub m_sort_hierarchy_by_name: Option<bool>,
    pub m_last_human_description_avatar_source: Option<PPtr>,
    pub m_import_physical_cameras: Option<bool>,
    pub m_additional_bone: Option<bool>,
    pub m_animation_type: Option<FbxImporterMAnimationTypeVariant>,
    pub m_user_data: Option<String>,
    pub m_file_ids_generation: Option<i32>,
    pub m_node_name_collision_strategy: Option<i32>,
    pub m_preserve_hierarchy: Option<bool>,
    pub index_format: Option<i32>,
    pub skin_weights_mode: Option<i32>,
    pub normal_calculation_mode: Option<FbxImporterNormalCalculationModeVariant>,
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
    pub optimize_bones: Option<FbxImporterOptimizeBonesVariant>,
    pub m_file_scale_factor: Option<f32>,
    pub generate_mesh_lods: Option<bool>,
    pub mesh_lod_generation_flags: Option<i32>,
    pub maximum_mesh_lod: Option<i32>,
    pub m_has_embedded_textures: Option<FbxImporterMHasEmbeddedTexturesVariant>,
    pub secondary_uv_margin_method: Option<i32>,
    pub import_u_vs: Option<i32>,
    pub secondary_uv_min_lightmap_resolution: Option<f32>,
    pub import_vertex_colors: Option<bool>,
    pub secondary_uv_min_object_scale: Option<f32>,
    pub legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes: Option<bool>,
    pub blend_shape_normal_import_mode: Option<i32>,
    pub normal_smoothing_source: Option<FbxImporterNormalSmoothingSourceVariant>,
    pub m_strict_vertex_data_checks: Option<bool>,
    pub m_auto_generate_avatar_mapping_if_unspecified: Option<bool>,
    pub m_avatar_setup: Option<i32>,
    pub m_contains_animation: Option<bool>,
    pub calculate_blendshape_normals_delta_from_imported_normals: Option<bool>,
    pub m_add_humanoid_extra_root_only_when_using_avatar:
        Option<FbxImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant>,
    pub m_remap_materials_if_material_import_mode_is_none: Option<bool>,
    pub m_import_blend_shape_deform_percent:
        Option<FbxImporterMImportBlendShapeDeformPercentVariant>,
}

impl crate::Parse for FbxImporter {
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
            m_external_objects = Some(FbxImporterMExternalObjectsVariant::parse(rdr, ver)?);
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
            m_external_objects = Some(FbxImporterMExternalObjectsVariant::parse(rdr, ver)?);
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
            m_external_objects = Some(FbxImporterMExternalObjectsVariant::parse(rdr, ver)?);
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
            m_external_objects = Some(FbxImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_import_materials = Some(FbxImporterMImportMaterialsVariant::parse(rdr, ver)?);
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
                Some(FbxImporterMFileIdToRecycleNameVariant::parse(rdr, ver)?);
            m_external_objects = Some(FbxImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_import_materials = Some(FbxImporterMImportMaterialsVariant::parse(rdr, ver)?);
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
                Some(FbxImporterMFileIdToRecycleNameVariant::parse(rdr, ver)?);
            m_import_materials = Some(FbxImporterMImportMaterialsVariant::parse(rdr, ver)?);
            m_material_name = Some(rdr.read_i32()?);
            m_material_search = Some(rdr.read_i32()?);
            m_legacy_generate_animations = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_file_id_to_recycle_name =
                Some(FbxImporterMFileIdToRecycleNameVariant::parse(rdr, ver)?);
            m_import_materials = Some(FbxImporterMImportMaterialsVariant::parse(rdr, ver)?);
            m_material_name = Some(rdr.read_i32()?);
            m_material_search = Some(rdr.read_i32()?);
            m_generate_animations = Some(rdr.read_i32()?);
        } else {
            m_file_id_to_recycle_name =
                Some(FbxImporterMFileIdToRecycleNameVariant::parse(rdr, ver)?);
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_generate_materials = Some(rdr.read_i32()?);
            m_generate_animations = Some(rdr.read_i32()?);
            m_global_scale = Some(rdr.read_f32()?);
        }
        m_bake_simulation = Some(FbxImporterMBakeSimulationVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(FbxImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_remove_constant_scale_curves = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings = Some(
                FbxImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?,
            );
            m_import_animated_custom_properties = Some(
                FbxImporterMImportAnimatedCustomPropertiesVariant::parse(rdr, ver)?,
            );
            m_import_constraints = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(FbxImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_remove_constant_scale_curves = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_rig_import_errors = Some(rdr.read_aligned_string()?);
            m_rig_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings = Some(
                FbxImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?,
            );
            m_import_animated_custom_properties = Some(
                FbxImporterMImportAnimatedCustomPropertiesVariant::parse(rdr, ver)?,
            );
            m_import_constraints = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(FbxImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_rig_import_errors = Some(rdr.read_aligned_string()?);
            m_rig_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings = Some(
                FbxImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?,
            );
            m_import_animated_custom_properties = Some(
                FbxImporterMImportAnimatedCustomPropertiesVariant::parse(rdr, ver)?,
            );
            m_import_constraints = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(FbxImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_rig_import_errors = Some(rdr.read_aligned_string()?);
            m_rig_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings = Some(
                FbxImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?,
            );
            m_import_animated_custom_properties = Some(
                FbxImporterMImportAnimatedCustomPropertiesVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(FbxImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_rig_import_errors = Some(rdr.read_aligned_string()?);
            m_rig_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings = Some(
                FbxImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(FbxImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings = Some(
                FbxImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_resample_rotations = Some(rdr.read_bool()?);
            m_optimize_game_objects =
                Some(FbxImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings = Some(
                FbxImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_optimize_game_objects =
                Some(FbxImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings = Some(
                FbxImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_optimize_game_objects =
                Some(FbxImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
            m_motion_node_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_optimize_game_objects =
                Some(FbxImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_split_animations = Some(FbxImporterMSplitAnimationsVariant::parse(rdr, ver)?);
        } else {
            m_split_animations = Some(FbxImporterMSplitAnimationsVariant::parse(rdr, ver)?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
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
                FbxImporterMExtraExposedTransformPathsVariant::parse(rdr, ver)?,
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
                FbxImporterMExtraExposedTransformPathsVariant::parse(rdr, ver)?,
            );
        } else {
        }
        m_clip_animations = Some(FbxImporterMClipAnimationsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(FbxImporterOptimizeBonesVariant::parse(rdr, ver)?);
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
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_strict_vertex_data_checks = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(FbxImporterOptimizeBonesVariant::parse(rdr, ver)?);
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
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_strict_vertex_data_checks = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 19) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(FbxImporterOptimizeBonesVariant::parse(rdr, ver)?);
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
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_strict_vertex_data_checks = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 36) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(FbxImporterOptimizeBonesVariant::parse(rdr, ver)?);
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
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_strict_vertex_data_checks = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(FbxImporterOptimizeBonesVariant::parse(rdr, ver)?);
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
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
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
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 17) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
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
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
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
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
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
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu = Some(FbxImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_previous_calculated_global_scale = Some(rdr.read_f32()?);
            m_has_previous_calculated_global_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu = Some(FbxImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
            m_previous_calculated_global_scale = Some(rdr.read_f32()?);
            m_has_previous_calculated_global_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu = Some(FbxImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            index_format = Some(rdr.read_i32()?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
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
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu = Some(FbxImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu = Some(FbxImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(FbxImporterMFileScaleVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            m_import_blend_shapes = Some(rdr.read_bool()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu = Some(FbxImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            weld_vertices = Some(FbxImporterWeldVerticesVariant::parse(rdr, ver)?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh_for_gpu = Some(FbxImporterOptimizeMeshForGpuVariant::parse(rdr, ver)?);
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_lod_screen_percentages =
                Some(FbxImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(FbxImporterMUseFileUnitsVariant::parse(rdr, ver)?);
            optimize_mesh = Some(rdr.read_bool()?);
            rdr.align(4)?;
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
        } else {
            m_first_import_version = Some(rdr.read_i32()?);
        }
        normal_smooth_angle = Some(FbxImporterNormalSmoothAngleVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(FbxImporterTangentImportModeVariant::parse(rdr, ver)?);
            normal_calculation_mode =
                Some(FbxImporterNormalCalculationModeVariant::parse(rdr, ver)?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes =
                Some(rdr.read_bool()?);
            rdr.align(4)?;
            blend_shape_normal_import_mode = Some(rdr.read_i32()?);
            normal_smoothing_source =
                Some(FbxImporterNormalSmoothingSourceVariant::parse(rdr, ver)?);
            calculate_blendshape_normals_delta_from_imported_normals = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_imported_take_infos = Some(FbxImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(FbxImporterTangentImportModeVariant::parse(rdr, ver)?);
            normal_calculation_mode =
                Some(FbxImporterNormalCalculationModeVariant::parse(rdr, ver)?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes =
                Some(rdr.read_bool()?);
            rdr.align(4)?;
            blend_shape_normal_import_mode = Some(rdr.read_i32()?);
            normal_smoothing_source =
                Some(FbxImporterNormalSmoothingSourceVariant::parse(rdr, ver)?);
            m_imported_take_infos = Some(FbxImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(FbxImporterTangentImportModeVariant::parse(rdr, ver)?);
            normal_calculation_mode =
                Some(FbxImporterNormalCalculationModeVariant::parse(rdr, ver)?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes =
                Some(rdr.read_bool()?);
            rdr.align(4)?;
            blend_shape_normal_import_mode = Some(rdr.read_i32()?);
            normal_smoothing_source =
                Some(FbxImporterNormalSmoothingSourceVariant::parse(rdr, ver)?);
            m_imported_take_infos = Some(FbxImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(FbxImporterTangentImportModeVariant::parse(rdr, ver)?);
            normal_calculation_mode =
                Some(FbxImporterNormalCalculationModeVariant::parse(rdr, ver)?);
            m_imported_take_infos = Some(FbxImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(FbxImporterTangentImportModeVariant::parse(rdr, ver)?);
            m_imported_take_infos = Some(FbxImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            split_tangents_across_uv = Some(rdr.read_bool()?);
            rdr.align(4)?;
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(FbxImporterTangentImportModeVariant::parse(rdr, ver)?);
            m_imported_take_infos = Some(FbxImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            split_tangents_across_uv = Some(rdr.read_bool()?);
            rdr.align(4)?;
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(FbxImporterTangentImportModeVariant::parse(rdr, ver)?);
        } else {
            split_tangents_across_uv = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_imported_roots = Some(FbxImporterMImportedRootsVariant::parse(rdr, ver)?);
        m_has_extra_root = Some(FbxImporterMHasExtraRootVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_has_embedded_textures =
                Some(FbxImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_contains_animation = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar =
                Some(FbxImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
            m_import_blend_shape_deform_percent = Some(
                FbxImporterMImportBlendShapeDeformPercentVariant::parse(rdr, ver)?,
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
                Some(FbxImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar =
                Some(FbxImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
            m_import_blend_shape_deform_percent = Some(
                FbxImporterMImportBlendShapeDeformPercentVariant::parse(rdr, ver)?,
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
                Some(FbxImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar =
                Some(FbxImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
            m_import_blend_shape_deform_percent = Some(
                FbxImporterMImportBlendShapeDeformPercentVariant::parse(rdr, ver)?,
            );
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 5) {
            m_has_embedded_textures =
                Some(FbxImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar =
                Some(FbxImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
            m_remap_materials_if_material_import_mode_is_none = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_has_embedded_textures =
                Some(FbxImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar =
                Some(FbxImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 37) {
            m_has_embedded_textures =
                Some(FbxImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar =
                Some(FbxImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
            m_remap_materials_if_material_import_mode_is_none = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_has_embedded_textures =
                Some(FbxImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar =
                Some(FbxImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_has_embedded_textures =
                Some(FbxImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_has_embedded_textures =
                Some(FbxImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(FbxImporterMReferencedClipsVariant::parse(rdr, ver)?);
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_has_embedded_textures =
                Some(FbxImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 3) {
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_import_animation = Some(FbxImporterMImportAnimationVariant::parse(rdr, ver)?);
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_animation_type = Some(FbxImporterMAnimationTypeVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
        } else {
        }

        Ok(FbxImporter {
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
