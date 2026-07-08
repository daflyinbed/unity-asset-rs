//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::build_target_settings::BuildTargetSettings;
use crate::subclasses::input_import_settings::InputImportSettings;
use crate::subclasses::procedural_material_information::ProceduralMaterialInformation;
use crate::subclasses::procedural_texture_assignment::ProceduralTextureAssignment;
use crate::subclasses::unity_property_sheet::UnityPropertySheet;


/// 版本枚举：MaterialInstanceSettingsMaterialInformationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MaterialInstanceSettingsMaterialInformationVariant {
    Default(Box<ProceduralMaterialInformation>),
    V4_5(Box<ProceduralMaterialInformation>),
}

impl crate::Parse for MaterialInstanceSettingsMaterialInformationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = ( { Ok(Box::new(ProceduralMaterialInformation::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V4_5(v))
        } else {
            let v = ( { Ok(Box::new(ProceduralMaterialInformation::parse(rdr, ver)?)) } )?;
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
    pub inputs: Vec<InputImportSettings>,
    pub shader_keywords: Option<String>,
    pub material_information: MaterialInstanceSettingsMaterialInformationVariant,
    pub render_queue: Option<i32>,
    pub material_properties: UnityPropertySheet,
    pub lightmap_flags: Option<u32>,
    pub texture_parameters: Vec<InputImportSettings>,
    pub build_target_settings: Vec<BuildTargetSettings>,
    pub texture_assignments: Option<Vec<ProceduralTextureAssignment>>,
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
            rdr.align(4)?;
            prototype_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 3, 4) {
            shader_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            shader = Some(crate::PPtr::parse(rdr, ver)?);
            shader_keywords = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            render_queue = Some(rdr.read_i32()?);
            lightmap_flags = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 2) {
            shader = Some(crate::PPtr::parse(rdr, ver)?);
            shader_keywords = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            render_queue = Some(rdr.read_i32()?);
            lightmap_flags = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            shader = Some(crate::PPtr::parse(rdr, ver)?);
            shader_keywords = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            render_queue = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            shader = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
            shader_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        }
            inputs = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { InputImportSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            material_information = Some(MaterialInstanceSettingsMaterialInformationVariant::parse(rdr, ver)?);
            material_properties = Some(UnityPropertySheet::parse(rdr, ver)?);
            rdr.align(4)?;
            texture_parameters = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { InputImportSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            texture_assignments = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ProceduralTextureAssignment::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }
            build_target_settings = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BuildTargetSettings::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;

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
