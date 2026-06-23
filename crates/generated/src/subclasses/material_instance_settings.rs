//! 自动生成。请勿手动修改。

use crate::subclasses::build_target_settings::BuildTargetSettings;
use crate::subclasses::input_import_settings::InputImportSettings;
use crate::subclasses::procedural_material_information::ProceduralMaterialInformation;
use crate::subclasses::unity_property_sheet::UnityPropertySheet;

#[derive(Debug, Clone)]
pub struct MaterialInstanceSettings {
    pub name: String,
    pub prototype_name: String,
    pub shader_name: String,
    pub inputs: Vec<InputImportSettings>,
    pub material_information: ProceduralMaterialInformation,
    pub material_properties: UnityPropertySheet,
    pub texture_parameters: Vec<InputImportSettings>,
    pub build_target_settings: Vec<BuildTargetSettings>,
}

impl crate::Parse for MaterialInstanceSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let prototype_name = r.read_aligned_string()?;
        r.align(4)?;
        let shader_name = r.read_aligned_string()?;
        r.align(4)?;
        let inputs = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { InputImportSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let material_information = ProceduralMaterialInformation::parse(r, ver)?;
        let material_properties = UnityPropertySheet::parse(r, ver)?;
        r.align(4)?;
        let texture_parameters = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { InputImportSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let build_target_settings = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildTargetSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(MaterialInstanceSettings {
            name,
            prototype_name,
            shader_name,
            inputs,
            material_information,
            material_properties,
            texture_parameters,
            build_target_settings,
        })
    }
}
