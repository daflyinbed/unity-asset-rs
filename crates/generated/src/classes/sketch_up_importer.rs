//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::clip_animation_info::ClipAnimationInfo;
use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::human_description::HumanDescription;
use crate::subclasses::sketch_up_import_data::SketchUpImportData;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::take_info::TakeInfo;


/// 版本枚举：SketchUpImporterMSelectedNodesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMSelectedNodesVariant {
    Default(Vec<i32>),
    V2017_1(Vec<i32>),
}

impl crate::Parse for SketchUpImporterMSelectedNodesVariant {
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

/// 版本枚举：SketchUpImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for SketchUpImporterMExternalObjectsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SketchUpImporterMImportMaterialsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMImportMaterialsVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for SketchUpImporterMImportMaterialsVariant {
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

/// 版本枚举：SketchUpImporterMOptimizeGameObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMOptimizeGameObjectsVariant {
    Default(bool),
    V2021_2(bool),
}

impl crate::Parse for SketchUpImporterMOptimizeGameObjectsVariant {
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

/// 版本枚举：SketchUpImporterMAnimationDoRetargetingWarningsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMAnimationDoRetargetingWarningsVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for SketchUpImporterMAnimationDoRetargetingWarningsVariant {
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

/// 版本枚举：SketchUpImporterMImportAnimatedCustomPropertiesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMImportAnimatedCustomPropertiesVariant {
    Default(bool),
    V2018_1(bool),
}

impl crate::Parse for SketchUpImporterMImportAnimatedCustomPropertiesVariant {
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

/// 版本枚举：SketchUpImporterMExtraExposedTransformPathsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMExtraExposedTransformPathsVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for SketchUpImporterMExtraExposedTransformPathsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SketchUpImporterMClipAnimationsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMClipAnimationsVariant {
    Default(Vec<ClipAnimationInfo>),
    V2017_1(Vec<ClipAnimationInfo>),
}

impl crate::Parse for SketchUpImporterMClipAnimationsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClipAnimationInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClipAnimationInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SketchUpImporterMLodScreenPercentagesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMLodScreenPercentagesVariant {
    Default(Vec<f32>),
    V2017_1(Vec<f32>),
}

impl crate::Parse for SketchUpImporterMLodScreenPercentagesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SketchUpImporterWeldVerticesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterWeldVerticesVariant {
    Default(bool),
    V2017_3(bool),
}

impl crate::Parse for SketchUpImporterWeldVerticesVariant {
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

/// 版本枚举：SketchUpImporterMFileScaleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMFileScaleVariant {
    Default(f32),
    V2018_3(f32),
}

impl crate::Parse for SketchUpImporterMFileScaleVariant {
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

/// 版本枚举：SketchUpImporterNormalSmoothAngleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterNormalSmoothAngleVariant {
    Default(f32),
    V5_3(f32),
}

impl crate::Parse for SketchUpImporterNormalSmoothAngleVariant {
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

/// 版本枚举：SketchUpImporterTangentImportModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterTangentImportModeVariant {
    Default(i32),
    V2017_1(i32),
}

impl crate::Parse for SketchUpImporterTangentImportModeVariant {
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

/// 版本枚举：SketchUpImporterMImportedTakeInfosVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMImportedTakeInfosVariant {
    Default(Vec<TakeInfo>),
    V2017_1(Vec<TakeInfo>),
}

impl crate::Parse for SketchUpImporterMImportedTakeInfosVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TakeInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TakeInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SketchUpImporterMReferencedClipsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMReferencedClipsVariant {
    Default(Vec<Guid>),
    V2017_1(Vec<Guid>),
    V6000_0(Vec<Guid>),
}

impl crate::Parse for SketchUpImporterMReferencedClipsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::V6000_0(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SketchUpImporterMImportedRootsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMImportedRootsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for SketchUpImporterMImportedRootsVariant {
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

/// 版本枚举：SketchUpImporterMHasExtraRootVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMHasExtraRootVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for SketchUpImporterMHasExtraRootVariant {
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

/// 版本枚举：SketchUpImporterOptimizeBonesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterOptimizeBonesVariant {
    Default(bool),
    V6000_2(bool),
}

impl crate::Parse for SketchUpImporterOptimizeBonesVariant {
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

/// 版本枚举：SketchUpImporterMImportAnimationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMImportAnimationVariant {
    Default(bool),
    V2019_3(bool),
}

impl crate::Parse for SketchUpImporterMImportAnimationVariant {
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

/// 版本枚举：SketchUpImporterNormalCalculationModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterNormalCalculationModeVariant {
    Default(i32),
    V2018_3(i32),
}

impl crate::Parse for SketchUpImporterNormalCalculationModeVariant {
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

/// 版本枚举：SketchUpImporterMHasEmbeddedTexturesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMHasEmbeddedTexturesVariant {
    Default(bool),
    V2023_1(bool),
}

impl crate::Parse for SketchUpImporterMHasEmbeddedTexturesVariant {
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

/// 版本枚举：SketchUpImporterNormalSmoothingSourceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterNormalSmoothingSourceVariant {
    Default(i32),
    V6000_5(i32),
}

impl crate::Parse for SketchUpImporterNormalSmoothingSourceVariant {
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

/// 版本枚举：SketchUpImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant {
    Default(bool),
    V2020_3_37(bool),
    V2021_1(bool),
    V2021_3_5(bool),
}

impl crate::Parse for SketchUpImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant {
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

/// 版本枚举：SketchUpImporterMImportBlendShapeDeformPercentVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImporterMImportBlendShapeDeformPercentVariant {
    Default(bool),
    V2022_1_5(bool),
}

impl crate::Parse for SketchUpImporterMImportBlendShapeDeformPercentVariant {
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
pub struct SketchUpImporter {
    pub m_generate_back_face: bool,
    pub m_name: String,
    pub m_merge_coplanar_faces: bool,
    pub m_internal_id_to_name_table: Option<Vec<((i32, i64), String)>>,
    pub m_selected_nodes: SketchUpImporterMSelectedNodesVariant,
    pub m_external_objects: Option<SketchUpImporterMExternalObjectsVariant>,
    pub m_asset_hash: Hash128,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_longitude: f64,
    pub m_import_materials: Option<SketchUpImporterMImportMaterialsVariant>,
    pub m_supports_embedded_materials: Option<bool>,
    pub m_latitude: f64,
    pub m_material_import_mode: Option<i32>,
    pub m_north_correction: f64,
    pub m_material_name: i32,
    pub m_file_unit: i32,
    pub m_material_search: i32,
    pub m_sketch_up_import_data: SketchUpImportData,
    pub m_material_location: Option<i32>,
    pub m_materials: Option<Vec<SourceAssetIdentifier>>,
    pub m_search_textures_globally: Option<bool>,
    pub m_file_id_to_recycle_name: Option<Vec<(i64, String)>>,
    pub m_legacy_generate_animations: i32,
    pub m_bake_simulation: bool,
    pub m_resample_curves: Option<bool>,
    pub m_optimize_game_objects: SketchUpImporterMOptimizeGameObjectsVariant,
    pub m_motion_node_name: String,
    pub m_remove_constant_scale_curves: Option<bool>,
    pub m_rig_import_errors: Option<String>,
    pub m_animation_import_errors: String,
    pub m_resample_rotations: Option<bool>,
    pub m_rig_import_warnings: Option<String>,
    pub m_animation_import_warnings: String,
    pub m_animation_retargeting_warnings: String,
    pub m_animation_do_retargeting_warnings: SketchUpImporterMAnimationDoRetargetingWarningsVariant,
    pub m_import_animated_custom_properties: Option<SketchUpImporterMImportAnimatedCustomPropertiesVariant>,
    pub m_import_constraints: Option<bool>,
    pub m_animation_compression: i32,
    pub m_animation_rotation_error: f32,
    pub m_animation_position_error: f32,
    pub m_animation_scale_error: f32,
    pub m_animation_wrap_mode: i32,
    pub m_extra_exposed_transform_paths: SketchUpImporterMExtraExposedTransformPathsVariant,
    pub m_clip_animations: SketchUpImporterMClipAnimationsVariant,
    pub m_extra_user_properties: Option<Vec<String>>,
    pub m_is_readable: bool,
    pub m_lod_screen_percentages: SketchUpImporterMLodScreenPercentagesVariant,
    pub m_global_scale: f32,
    pub m_mesh_compression: i32,
    pub m_add_colliders: bool,
    pub m_import_blend_shapes: bool,
    pub swap_uv_channels: bool,
    pub m_use_srgb_material_color: Option<bool>,
    pub generate_secondary_uv: bool,
    pub m_sort_hierarchy_by_name: Option<bool>,
    pub m_use_file_units: bool,
    pub m_import_physical_cameras: Option<bool>,
    pub optimize_mesh_for_gpu: Option<bool>,
    pub m_import_visibility: Option<bool>,
    pub keep_quads: bool,
    pub weld_vertices: SketchUpImporterWeldVerticesVariant,
    pub m_import_cameras: Option<bool>,
    pub secondary_uv_angle_distortion: f32,
    pub m_import_lights: Option<bool>,
    pub secondary_uv_area_distortion: f32,
    pub m_file_ids_generation: Option<i32>,
    pub m_node_name_collision_strategy: Option<i32>,
    pub secondary_uv_hard_angle: f32,
    pub secondary_uv_pack_margin: f32,
    pub m_use_file_scale: bool,
    pub m_file_scale: SketchUpImporterMFileScaleVariant,
    pub normal_smooth_angle: SketchUpImporterNormalSmoothAngleVariant,
    pub m_preserve_hierarchy: Option<bool>,
    pub split_tangents_across_uv: Option<bool>,
    pub skin_weights_mode: Option<i32>,
    pub bake_axis_conversion: Option<bool>,
    pub normal_import_mode: i32,
    pub max_bones_per_vertex: Option<i32>,
    pub tangent_import_mode: SketchUpImporterTangentImportModeVariant,
    pub min_bone_weight: Option<f32>,
    pub m_imported_take_infos: SketchUpImporterMImportedTakeInfosVariant,
    pub mesh_optimization_flags: Option<i32>,
    pub m_referenced_clips: SketchUpImporterMReferencedClipsVariant,
    pub index_format: Option<i32>,
    pub m_imported_roots: SketchUpImporterMImportedRootsVariant,
    pub m_has_extra_root: SketchUpImporterMHasExtraRootVariant,
    pub optimize_bones: Option<SketchUpImporterOptimizeBonesVariant>,
    pub m_import_animation: SketchUpImporterMImportAnimationVariant,
    pub generate_mesh_lods: Option<bool>,
    pub m_copy_avatar: Option<bool>,
    pub mesh_lod_generation_flags: Option<i32>,
    pub m_human_description: HumanDescription,
    pub normal_calculation_mode: Option<SketchUpImporterNormalCalculationModeVariant>,
    pub maximum_mesh_lod: Option<i32>,
    pub m_last_human_description_avatar_source: PPtr,
    pub secondary_uv_margin_method: Option<i32>,
    pub import_u_vs: Option<i32>,
    pub m_animation_type: i32,
    pub m_file_scale_unit: Option<String>,
    pub secondary_uv_min_lightmap_resolution: Option<f32>,
    pub import_vertex_colors: Option<bool>,
    pub m_additional_bone: bool,
    pub m_humanoid_oversampling: Option<i32>,
    pub m_file_scale_factor: Option<f32>,
    pub secondary_uv_min_object_scale: Option<f32>,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_previous_calculated_global_scale: Option<f32>,
    pub m_asset_bundle_variant: String,
    pub m_has_previous_calculated_global_scale: Option<bool>,
    pub legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes: Option<bool>,
    pub blend_shape_normal_import_mode: Option<i32>,
    pub m_has_embedded_textures: Option<SketchUpImporterMHasEmbeddedTexturesVariant>,
    pub normal_smoothing_source: Option<SketchUpImporterNormalSmoothingSourceVariant>,
    pub m_strict_vertex_data_checks: Option<bool>,
    pub m_auto_generate_avatar_mapping_if_unspecified: Option<bool>,
    pub m_avatar_setup: Option<i32>,
    pub m_contains_animation: Option<bool>,
    pub calculate_blendshape_normals_delta_from_imported_normals: Option<bool>,
    pub m_add_humanoid_extra_root_only_when_using_avatar: Option<SketchUpImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant>,
    pub m_remap_materials_if_material_import_mode_is_none: Option<bool>,
    pub m_import_blend_shape_deform_percent: Option<SketchUpImporterMImportBlendShapeDeformPercentVariant>,
}

impl crate::Parse for SketchUpImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_generate_back_face = None;
        let mut m_name = None;
        let mut m_merge_coplanar_faces = None;
        let mut m_internal_id_to_name_table = None;
        let mut m_selected_nodes = None;
        let mut m_external_objects = None;
        let mut m_asset_hash = None;
        let mut m_used_file_i_ds = None;
        let mut m_longitude = None;
        let mut m_import_materials = None;
        let mut m_supports_embedded_materials = None;
        let mut m_latitude = None;
        let mut m_material_import_mode = None;
        let mut m_north_correction = None;
        let mut m_material_name = None;
        let mut m_file_unit = None;
        let mut m_material_search = None;
        let mut m_sketch_up_import_data = None;
        let mut m_material_location = None;
        let mut m_materials = None;
        let mut m_search_textures_globally = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_legacy_generate_animations = None;
        let mut m_bake_simulation = None;
        let mut m_resample_curves = None;
        let mut m_optimize_game_objects = None;
        let mut m_motion_node_name = None;
        let mut m_remove_constant_scale_curves = None;
        let mut m_rig_import_errors = None;
        let mut m_animation_import_errors = None;
        let mut m_resample_rotations = None;
        let mut m_rig_import_warnings = None;
        let mut m_animation_import_warnings = None;
        let mut m_animation_retargeting_warnings = None;
        let mut m_animation_do_retargeting_warnings = None;
        let mut m_import_animated_custom_properties = None;
        let mut m_import_constraints = None;
        let mut m_animation_compression = None;
        let mut m_animation_rotation_error = None;
        let mut m_animation_position_error = None;
        let mut m_animation_scale_error = None;
        let mut m_animation_wrap_mode = None;
        let mut m_extra_exposed_transform_paths = None;
        let mut m_clip_animations = None;
        let mut m_extra_user_properties = None;
        let mut m_is_readable = None;
        let mut m_lod_screen_percentages = None;
        let mut m_global_scale = None;
        let mut m_mesh_compression = None;
        let mut m_add_colliders = None;
        let mut m_import_blend_shapes = None;
        let mut swap_uv_channels = None;
        let mut m_use_srgb_material_color = None;
        let mut generate_secondary_uv = None;
        let mut m_sort_hierarchy_by_name = None;
        let mut m_use_file_units = None;
        let mut m_import_physical_cameras = None;
        let mut optimize_mesh_for_gpu = None;
        let mut m_import_visibility = None;
        let mut keep_quads = None;
        let mut weld_vertices = None;
        let mut m_import_cameras = None;
        let mut secondary_uv_angle_distortion = None;
        let mut m_import_lights = None;
        let mut secondary_uv_area_distortion = None;
        let mut m_file_ids_generation = None;
        let mut m_node_name_collision_strategy = None;
        let mut secondary_uv_hard_angle = None;
        let mut secondary_uv_pack_margin = None;
        let mut m_use_file_scale = None;
        let mut m_file_scale = None;
        let mut normal_smooth_angle = None;
        let mut m_preserve_hierarchy = None;
        let mut split_tangents_across_uv = None;
        let mut skin_weights_mode = None;
        let mut bake_axis_conversion = None;
        let mut normal_import_mode = None;
        let mut max_bones_per_vertex = None;
        let mut tangent_import_mode = None;
        let mut min_bone_weight = None;
        let mut m_imported_take_infos = None;
        let mut mesh_optimization_flags = None;
        let mut m_referenced_clips = None;
        let mut index_format = None;
        let mut m_imported_roots = None;
        let mut m_has_extra_root = None;
        let mut optimize_bones = None;
        let mut m_import_animation = None;
        let mut generate_mesh_lods = None;
        let mut m_copy_avatar = None;
        let mut mesh_lod_generation_flags = None;
        let mut m_human_description = None;
        let mut normal_calculation_mode = None;
        let mut maximum_mesh_lod = None;
        let mut m_last_human_description_avatar_source = None;
        let mut secondary_uv_margin_method = None;
        let mut import_u_vs = None;
        let mut m_animation_type = None;
        let mut m_file_scale_unit = None;
        let mut secondary_uv_min_lightmap_resolution = None;
        let mut import_vertex_colors = None;
        let mut m_additional_bone = None;
        let mut m_humanoid_oversampling = None;
        let mut m_file_scale_factor = None;
        let mut secondary_uv_min_object_scale = None;
        let mut m_user_data = None;
        let mut m_asset_bundle_name = None;
        let mut m_previous_calculated_global_scale = None;
        let mut m_asset_bundle_variant = None;
        let mut m_has_previous_calculated_global_scale = None;
        let mut legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes = None;
        let mut blend_shape_normal_import_mode = None;
        let mut m_has_embedded_textures = None;
        let mut normal_smoothing_source = None;
        let mut m_strict_vertex_data_checks = None;
        let mut m_auto_generate_avatar_mapping_if_unspecified = None;
        let mut m_avatar_setup = None;
        let mut m_contains_animation = None;
        let mut calculate_blendshape_normals_delta_from_imported_normals = None;
        let mut m_add_humanoid_extra_root_only_when_using_avatar = None;
        let mut m_remap_materials_if_material_import_mode_is_none = None;
        let mut m_import_blend_shape_deform_percent = None;
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
        } else {
            m_generate_back_face = Some(rdr.read_bool()?);
            m_merge_coplanar_faces = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_selected_nodes = Some(SketchUpImporterMSelectedNodesVariant::parse(rdr, ver)?);
            m_asset_hash = Some(Hash128::parse(rdr, ver)?);
            m_longitude = Some(rdr.read_f64()?);
            m_latitude = Some(rdr.read_f64()?);
            m_north_correction = Some(rdr.read_f64()?);
            m_file_unit = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_sketch_up_import_data = Some(SketchUpImportData::parse(rdr, ver)?);
        }
            m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_internal_id_to_name_table = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = rdr.read_i32()?; let b = rdr.read_i64()?; Ok((a, b)) } )?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_external_objects = Some(SketchUpImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_material_import_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_internal_id_to_name_table = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = rdr.read_i32()?; let b = rdr.read_i64()?; Ok((a, b)) } )?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_external_objects = Some(SketchUpImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_import_materials = Some(SketchUpImporterMImportMaterialsVariant::parse(rdr, ver)?);
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_file_id_to_recycle_name = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i64()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_external_objects = Some(SketchUpImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_import_materials = Some(SketchUpImporterMImportMaterialsVariant::parse(rdr, ver)?);
            m_supports_embedded_materials = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_file_id_to_recycle_name = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i64()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_materials = Some(SketchUpImporterMImportMaterialsVariant::parse(rdr, ver)?);
        }
            m_material_name = Some(rdr.read_i32()?);
            m_material_search = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_material_location = Some(rdr.read_i32()?);
            m_search_textures_globally = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_materials = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_material_location = Some(rdr.read_i32()?);
            m_materials = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
        }
            m_legacy_generate_animations = Some(rdr.read_i32()?);
            m_bake_simulation = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_resample_curves = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_resample_rotations = Some(rdr.read_bool()?);
        } else {
        }
            m_optimize_game_objects = Some(SketchUpImporterMOptimizeGameObjectsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_remove_constant_scale_curves = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_motion_node_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_rig_import_errors = Some(rdr.read_aligned_string()?);
            m_rig_import_warnings = Some(rdr.read_aligned_string()?);
        } else {
        }
            m_animation_import_errors = Some(rdr.read_aligned_string()?);
            m_animation_import_warnings = Some(rdr.read_aligned_string()?);
            m_animation_retargeting_warnings = Some(rdr.read_aligned_string()?);
            m_animation_do_retargeting_warnings = Some(SketchUpImporterMAnimationDoRetargetingWarningsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_import_animated_custom_properties = Some(SketchUpImporterMImportAnimatedCustomPropertiesVariant::parse(rdr, ver)?);
            m_import_constraints = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_import_animated_custom_properties = Some(SketchUpImporterMImportAnimatedCustomPropertiesVariant::parse(rdr, ver)?);
        } else {
        }
            m_animation_compression = Some(rdr.read_i32()?);
            m_animation_rotation_error = Some(rdr.read_f32()?);
            m_animation_position_error = Some(rdr.read_f32()?);
            m_animation_scale_error = Some(rdr.read_f32()?);
            m_animation_wrap_mode = Some(rdr.read_i32()?);
            m_extra_exposed_transform_paths = Some(SketchUpImporterMExtraExposedTransformPathsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_extra_user_properties = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }
            m_clip_animations = Some(SketchUpImporterMClipAnimationsVariant::parse(rdr, ver)?);
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_lod_screen_percentages = Some(SketchUpImporterMLodScreenPercentagesVariant::parse(rdr, ver)?);
            m_global_scale = Some(rdr.read_f32()?);
            m_mesh_compression = Some(rdr.read_i32()?);
            m_add_colliders = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 19) {
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_sort_hierarchy_by_name = Some(rdr.read_bool()?);
            m_import_physical_cameras = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_sort_hierarchy_by_name = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_use_srgb_material_color = Some(rdr.read_bool()?);
            m_import_visibility = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_import_visibility = Some(rdr.read_bool()?);
        } else {
        }
            m_import_blend_shapes = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_node_name_collision_strategy = Some(rdr.read_i32()?);
            m_file_ids_generation = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 17) {
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_ids_generation = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_import_cameras = Some(rdr.read_bool()?);
            m_import_lights = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            swap_uv_channels = Some(rdr.read_bool()?);
            generate_secondary_uv = Some(rdr.read_bool()?);
            m_use_file_units = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else {
            optimize_mesh_for_gpu = Some(rdr.read_bool()?);
        }
            keep_quads = Some(rdr.read_bool()?);
            weld_vertices = Some(SketchUpImporterWeldVerticesVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(SketchUpImporterOptimizeBonesVariant::parse(rdr, ver)?);
            generate_mesh_lods = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mesh_lod_generation_flags = Some(rdr.read_i32()?);
            maximum_mesh_lod = Some(rdr.read_i32()?);
            import_u_vs = Some(rdr.read_i32()?);
            import_vertex_colors = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(SketchUpImporterOptimizeBonesVariant::parse(rdr, ver)?);
            generate_mesh_lods = Some(rdr.read_bool()?);
            rdr.align(4)?;
            mesh_lod_generation_flags = Some(rdr.read_i32()?);
            maximum_mesh_lod = Some(rdr.read_i32()?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            optimize_bones = Some(SketchUpImporterOptimizeBonesVariant::parse(rdr, ver)?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            bake_axis_conversion = Some(rdr.read_bool()?);
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            skin_weights_mode = Some(rdr.read_i32()?);
            max_bones_per_vertex = Some(rdr.read_i32()?);
            min_bone_weight = Some(rdr.read_f32()?);
            mesh_optimization_flags = Some(rdr.read_i32()?);
            index_format = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_preserve_hierarchy = Some(rdr.read_bool()?);
            rdr.align(4)?;
            index_format = Some(rdr.read_i32()?);
        } else {
        }
            secondary_uv_angle_distortion = Some(rdr.read_f32()?);
            secondary_uv_area_distortion = Some(rdr.read_f32()?);
            secondary_uv_hard_angle = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            secondary_uv_margin_method = Some(rdr.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(rdr.read_f32()?);
            secondary_uv_min_object_scale = Some(rdr.read_f32()?);
        } else {
        }
            secondary_uv_pack_margin = Some(rdr.read_f32()?);
            m_use_file_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_file_scale = Some(SketchUpImporterMFileScaleVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2021, 3, 36) {
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_strict_vertex_data_checks = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_file_scale_unit = Some(rdr.read_aligned_string()?);
            m_file_scale_factor = Some(rdr.read_f32()?);
            rdr.align(4)?;
            m_previous_calculated_global_scale = Some(rdr.read_f32()?);
            m_has_previous_calculated_global_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_previous_calculated_global_scale = Some(rdr.read_f32()?);
            m_has_previous_calculated_global_scale = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            normal_smooth_angle = Some(SketchUpImporterNormalSmoothAngleVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
        } else {
            split_tangents_across_uv = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
            normal_import_mode = Some(rdr.read_i32()?);
            tangent_import_mode = Some(SketchUpImporterTangentImportModeVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            normal_calculation_mode = Some(SketchUpImporterNormalCalculationModeVariant::parse(rdr, ver)?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            blend_shape_normal_import_mode = Some(rdr.read_i32()?);
            normal_smoothing_source = Some(SketchUpImporterNormalSmoothingSourceVariant::parse(rdr, ver)?);
            calculate_blendshape_normals_delta_from_imported_normals = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            normal_calculation_mode = Some(SketchUpImporterNormalCalculationModeVariant::parse(rdr, ver)?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            blend_shape_normal_import_mode = Some(rdr.read_i32()?);
            normal_smoothing_source = Some(SketchUpImporterNormalSmoothingSourceVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            normal_calculation_mode = Some(SketchUpImporterNormalCalculationModeVariant::parse(rdr, ver)?);
        } else {
        }
            m_imported_take_infos = Some(SketchUpImporterMImportedTakeInfosVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else {
            m_referenced_clips = Some(SketchUpImporterMReferencedClipsVariant::parse(rdr, ver)?);
        }
            m_imported_roots = Some(SketchUpImporterMImportedRootsVariant::parse(rdr, ver)?);
            m_has_extra_root = Some(SketchUpImporterMHasExtraRootVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_has_embedded_textures = Some(SketchUpImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_contains_animation = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_referenced_clips = Some(SketchUpImporterMReferencedClipsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_has_embedded_textures = Some(SketchUpImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
            m_referenced_clips = Some(SketchUpImporterMReferencedClipsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_has_embedded_textures = Some(SketchUpImporterMHasEmbeddedTexturesVariant::parse(rdr, ver)?);
        } else {
        }
            m_import_animation = Some(SketchUpImporterMImportAnimationVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
        } else {
            m_copy_avatar = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
            m_human_description = Some(HumanDescription::parse(rdr, ver)?);
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_auto_generate_avatar_mapping_if_unspecified = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
            m_animation_type = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2022, 1, 5) {
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(SketchUpImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
            m_import_blend_shape_deform_percent = Some(SketchUpImporterMImportBlendShapeDeformPercentVariant::parse(rdr, ver)?);
            m_remap_materials_if_material_import_mode_is_none = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(SketchUpImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
            m_import_blend_shape_deform_percent = Some(SketchUpImporterMImportBlendShapeDeformPercentVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 5) {
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(SketchUpImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
            m_remap_materials_if_material_import_mode_is_none = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(SketchUpImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 37) {
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(SketchUpImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
            m_remap_materials_if_material_import_mode_is_none = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(SketchUpImporterMAddHumanoidExtraRootOnlyWhenUsingAvatarVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_humanoid_oversampling = Some(rdr.read_i32()?);
            m_avatar_setup = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_humanoid_oversampling = Some(rdr.read_i32()?);
        } else {
        }
            m_additional_bone = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_generate_back_face = Some(rdr.read_bool()?);
            m_merge_coplanar_faces = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_selected_nodes = Some(SketchUpImporterMSelectedNodesVariant::parse(rdr, ver)?);
            m_asset_hash = Some(Hash128::parse(rdr, ver)?);
            m_longitude = Some(rdr.read_f64()?);
            m_latitude = Some(rdr.read_f64()?);
            m_north_correction = Some(rdr.read_f64()?);
            m_file_unit = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_sketch_up_import_data = Some(SketchUpImportData::parse(rdr, ver)?);
        } else {
        }

        Ok(SketchUpImporter {
            m_generate_back_face: m_generate_back_face.unwrap(),
            m_name: m_name.unwrap(),
            m_merge_coplanar_faces: m_merge_coplanar_faces.unwrap(),
            m_internal_id_to_name_table,
            m_selected_nodes: m_selected_nodes.unwrap(),
            m_external_objects,
            m_asset_hash: m_asset_hash.unwrap(),
            m_used_file_i_ds,
            m_longitude: m_longitude.unwrap(),
            m_import_materials,
            m_supports_embedded_materials,
            m_latitude: m_latitude.unwrap(),
            m_material_import_mode,
            m_north_correction: m_north_correction.unwrap(),
            m_material_name: m_material_name.unwrap(),
            m_file_unit: m_file_unit.unwrap(),
            m_material_search: m_material_search.unwrap(),
            m_sketch_up_import_data: m_sketch_up_import_data.unwrap(),
            m_material_location,
            m_materials,
            m_search_textures_globally,
            m_file_id_to_recycle_name,
            m_legacy_generate_animations: m_legacy_generate_animations.unwrap(),
            m_bake_simulation: m_bake_simulation.unwrap(),
            m_resample_curves,
            m_optimize_game_objects: m_optimize_game_objects.unwrap(),
            m_motion_node_name: m_motion_node_name.unwrap(),
            m_remove_constant_scale_curves,
            m_rig_import_errors,
            m_animation_import_errors: m_animation_import_errors.unwrap(),
            m_resample_rotations,
            m_rig_import_warnings,
            m_animation_import_warnings: m_animation_import_warnings.unwrap(),
            m_animation_retargeting_warnings: m_animation_retargeting_warnings.unwrap(),
            m_animation_do_retargeting_warnings: m_animation_do_retargeting_warnings.unwrap(),
            m_import_animated_custom_properties,
            m_import_constraints,
            m_animation_compression: m_animation_compression.unwrap(),
            m_animation_rotation_error: m_animation_rotation_error.unwrap(),
            m_animation_position_error: m_animation_position_error.unwrap(),
            m_animation_scale_error: m_animation_scale_error.unwrap(),
            m_animation_wrap_mode: m_animation_wrap_mode.unwrap(),
            m_extra_exposed_transform_paths: m_extra_exposed_transform_paths.unwrap(),
            m_clip_animations: m_clip_animations.unwrap(),
            m_extra_user_properties,
            m_is_readable: m_is_readable.unwrap(),
            m_lod_screen_percentages: m_lod_screen_percentages.unwrap(),
            m_global_scale: m_global_scale.unwrap(),
            m_mesh_compression: m_mesh_compression.unwrap(),
            m_add_colliders: m_add_colliders.unwrap(),
            m_import_blend_shapes: m_import_blend_shapes.unwrap(),
            swap_uv_channels: swap_uv_channels.unwrap(),
            m_use_srgb_material_color,
            generate_secondary_uv: generate_secondary_uv.unwrap(),
            m_sort_hierarchy_by_name,
            m_use_file_units: m_use_file_units.unwrap(),
            m_import_physical_cameras,
            optimize_mesh_for_gpu,
            m_import_visibility,
            keep_quads: keep_quads.unwrap(),
            weld_vertices: weld_vertices.unwrap(),
            m_import_cameras,
            secondary_uv_angle_distortion: secondary_uv_angle_distortion.unwrap(),
            m_import_lights,
            secondary_uv_area_distortion: secondary_uv_area_distortion.unwrap(),
            m_file_ids_generation,
            m_node_name_collision_strategy,
            secondary_uv_hard_angle: secondary_uv_hard_angle.unwrap(),
            secondary_uv_pack_margin: secondary_uv_pack_margin.unwrap(),
            m_use_file_scale: m_use_file_scale.unwrap(),
            m_file_scale: m_file_scale.unwrap(),
            normal_smooth_angle: normal_smooth_angle.unwrap(),
            m_preserve_hierarchy,
            split_tangents_across_uv,
            skin_weights_mode,
            bake_axis_conversion,
            normal_import_mode: normal_import_mode.unwrap(),
            max_bones_per_vertex,
            tangent_import_mode: tangent_import_mode.unwrap(),
            min_bone_weight,
            m_imported_take_infos: m_imported_take_infos.unwrap(),
            mesh_optimization_flags,
            m_referenced_clips: m_referenced_clips.unwrap(),
            index_format,
            m_imported_roots: m_imported_roots.unwrap(),
            m_has_extra_root: m_has_extra_root.unwrap(),
            optimize_bones,
            m_import_animation: m_import_animation.unwrap(),
            generate_mesh_lods,
            m_copy_avatar,
            mesh_lod_generation_flags,
            m_human_description: m_human_description.unwrap(),
            normal_calculation_mode,
            maximum_mesh_lod,
            m_last_human_description_avatar_source: m_last_human_description_avatar_source.unwrap(),
            secondary_uv_margin_method,
            import_u_vs,
            m_animation_type: m_animation_type.unwrap(),
            m_file_scale_unit,
            secondary_uv_min_lightmap_resolution,
            import_vertex_colors,
            m_additional_bone: m_additional_bone.unwrap(),
            m_humanoid_oversampling,
            m_file_scale_factor,
            secondary_uv_min_object_scale,
            m_user_data: m_user_data.unwrap(),
            m_asset_bundle_name: m_asset_bundle_name.unwrap(),
            m_previous_calculated_global_scale,
            m_asset_bundle_variant: m_asset_bundle_variant.unwrap(),
            m_has_previous_calculated_global_scale,
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes,
            blend_shape_normal_import_mode,
            m_has_embedded_textures,
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
