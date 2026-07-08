//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::input_import_settings::InputImportSettings;
use crate::subclasses::material_build_target_settings::MaterialBuildTargetSettings;
use crate::subclasses::procedural_material_information::ProceduralMaterialInformation;
use crate::subclasses::procedural_texture_assignment::ProceduralTextureAssignment;
use crate::subclasses::texture_importer_platform_settings::TextureImporterPlatformSettings;
use crate::subclasses::unity_property_sheet::UnityPropertySheet;


/// 版本枚举：MaterialInstanceSettingsInputsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MaterialInstanceSettingsInputsVariant {
    Default(Vec<InputImportSettings>),
    V2017_1(Vec<InputImportSettings>),
}

impl crate::Parse for MaterialInstanceSettingsInputsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { InputImportSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { InputImportSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MaterialInstanceSettingsTextureParametersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MaterialInstanceSettingsTextureParametersVariant {
    Default(Vec<InputImportSettings>),
    V2017_1(Vec<InputImportSettings>),
}

impl crate::Parse for MaterialInstanceSettingsTextureParametersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { InputImportSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { InputImportSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MaterialInstanceSettingsBuildTargetSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MaterialInstanceSettingsBuildTargetSettingsVariant {
    Default(Vec<TextureImporterPlatformSettings>),
    V3_5(Vec<MaterialBuildTargetSettings>),
    V2017_1(Vec<MaterialBuildTargetSettings>),
}

impl crate::Parse for MaterialInstanceSettingsBuildTargetSettingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MaterialBuildTargetSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MaterialBuildTargetSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::V3_5(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TextureImporterPlatformSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MaterialInstanceSettingsTextureAssignmentsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MaterialInstanceSettingsTextureAssignmentsVariant {
    Default(Vec<ProceduralTextureAssignment>),
    V2017_1(Vec<ProceduralTextureAssignment>),
}

impl crate::Parse for MaterialInstanceSettingsTextureAssignmentsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ProceduralTextureAssignment::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ProceduralTextureAssignment::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct MaterialInstanceSettings {
    pub name: String,
    pub prototype_name: String,
    pub shader_name: Option<String>,
    pub shader: Option<PPtr>,
    pub inputs: MaterialInstanceSettingsInputsVariant,
    pub shader_keywords: Option<String>,
    pub material_information: ProceduralMaterialInformation,
    pub render_queue: Option<i32>,
    pub material_properties: UnityPropertySheet,
    pub lightmap_flags: Option<u32>,
    pub texture_parameters: MaterialInstanceSettingsTextureParametersVariant,
    pub build_target_settings: MaterialInstanceSettingsBuildTargetSettingsVariant,
    pub texture_assignments: Option<MaterialInstanceSettingsTextureAssignmentsVariant>,
}

impl crate::Parse for MaterialInstanceSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut prototype_name = None;
        let mut shader_name = None;
        let mut shader = None;
        let mut inputs = None;
        let mut shader_keywords = None;
        let mut material_information = None;
        let mut render_queue = None;
        let mut material_properties = None;
        let mut lightmap_flags = None;
        let mut texture_parameters = None;
        let mut build_target_settings = None;
        let mut texture_assignments = None;
            name = Some(rdr.read_aligned_string()?);
            prototype_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(5, 3, 4) {
            shader_name = Some(rdr.read_aligned_string()?);
            shader = Some(crate::PPtr::parse(rdr, ver)?);
            shader_keywords = Some(rdr.read_aligned_string()?);
            render_queue = Some(rdr.read_i32()?);
            lightmap_flags = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 2) {
            shader = Some(crate::PPtr::parse(rdr, ver)?);
            shader_keywords = Some(rdr.read_aligned_string()?);
            render_queue = Some(rdr.read_i32()?);
            lightmap_flags = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            shader = Some(crate::PPtr::parse(rdr, ver)?);
            shader_keywords = Some(rdr.read_aligned_string()?);
            render_queue = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            shader = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
            shader_name = Some(rdr.read_aligned_string()?);
        }
            inputs = Some(MaterialInstanceSettingsInputsVariant::parse(rdr, ver)?);
            material_information = Some(ProceduralMaterialInformation::parse(rdr, ver)?);
            material_properties = Some(UnityPropertySheet::parse(rdr, ver)?);
            texture_parameters = Some(MaterialInstanceSettingsTextureParametersVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            texture_assignments = Some(MaterialInstanceSettingsTextureAssignmentsVariant::parse(rdr, ver)?);
        } else {
        }
            build_target_settings = Some(MaterialInstanceSettingsBuildTargetSettingsVariant::parse(rdr, ver)?);

        Ok(MaterialInstanceSettings {
            name: name.unwrap(),
            prototype_name: prototype_name.unwrap(),
            shader_name,
            shader,
            inputs: inputs.unwrap(),
            shader_keywords,
            material_information: material_information.unwrap(),
            render_queue,
            material_properties: material_properties.unwrap(),
            lightmap_flags,
            texture_parameters: texture_parameters.unwrap(),
            build_target_settings: build_target_settings.unwrap(),
            texture_assignments,
        })
    }
}
