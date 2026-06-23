//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::per_lod_settings::PerLodSettings;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use super::asset_importer::AssetImporter;

#[derive(Debug, Clone)]
pub struct SpeedTreeImporter {
    pub parent: AssetImporter,
    pub m_name: String,
    pub m_main_color: ColorRgba,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_spec_color: Option<ColorRgba>,
    pub m_hue_variation: ColorRgba,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_alpha_test_ref: f32,
    pub m_shininess: Option<f32>,
    pub m_best_wind_quality: i32,
    pub m_has_billboard: bool,
    pub m_lod_settings: Vec<PerLodSettings>,
    pub m_enable_hue_variation: Option<bool>,
    pub m_enable_smooth_lod_transition: bool,
    pub m_enable_bump_mapping: Option<bool>,
    pub m_animate_cross_fading: Option<bool>,
    pub m_enable_subsurface_scattering: Option<bool>,
    pub m_billboard_transition_cross_fade_width: f32,
    pub m_fade_out_width: f32,
    pub m_selected_wind_quality: Option<i32>,
    pub m_scale_factor: f32,
    pub m_enable_shadow_casting: Option<bool>,
    pub m_material_version: Option<i32>,
    pub m_enable_shadow_receiving: Option<bool>,
    pub m_user_data: String,
    pub m_enable_light_probes: Option<bool>,
    pub m_asset_bundle_name: String,
    pub m_material_location: Option<i32>,
    pub m_motion_vector_mode_enum_value: Option<i32>,
    pub m_reflection_probe_enum_value: Option<i32>,
    pub m_asset_bundle_variant: String,
    pub m_materials: Option<Vec<SourceAssetIdentifier>>,
    pub m_supports_embedded_materials: Option<bool>,
    pub m_file_id_type: Option<i32>,
    pub m_unit_conversion_enum_value: Option<i32>,
    pub m_generate_colliders: Option<bool>,
    pub m_generate_rigidbody: Option<bool>,
}

impl crate::Parse for SpeedTreeImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AssetImporter::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_main_color = ColorRgba::parse(r, ver)?;
        let m_external_objects = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_spec_color = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let m_hue_variation = ColorRgba::parse(r, ver)?;
        let m_used_file_i_ds = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_alpha_test_ref = r.read_f32()?;
        let m_shininess = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_best_wind_quality = r.read_i32()?;
        let m_has_billboard = r.read_bool()?;
        r.align(4)?;
        let m_lod_settings = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { PerLodSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_enable_hue_variation = if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_smooth_lod_transition = r.read_bool()?;
        r.align(4)?;
        let m_enable_bump_mapping = if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_animate_cross_fading = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enable_subsurface_scattering = if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_billboard_transition_cross_fade_width = r.read_f32()?;
        let m_fade_out_width = r.read_f32()?;
        let m_selected_wind_quality = if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_scale_factor = r.read_f32()?;
        r.align(4)?;
        let m_enable_shadow_casting = if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_material_version = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enable_shadow_receiving = if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_user_data = r.read_aligned_string()?;
        r.align(4)?;
        let m_enable_light_probes = if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_material_location = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_motion_vector_mode_enum_value = if *ver >= crate::UnityVersion::new(2022, 3, 3) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_reflection_probe_enum_value = if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_asset_bundle_variant = r.read_aligned_string()?;
        r.align(4)?;
        let m_materials = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_supports_embedded_materials = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_file_id_type = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_unit_conversion_enum_value = if *ver >= crate::UnityVersion::new(2022, 3, 12) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_generate_colliders = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_generate_rigidbody = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SpeedTreeImporter {
            parent,
            m_name,
            m_main_color,
            m_external_objects,
            m_spec_color,
            m_hue_variation,
            m_used_file_i_ds,
            m_alpha_test_ref,
            m_shininess,
            m_best_wind_quality,
            m_has_billboard,
            m_lod_settings,
            m_enable_hue_variation,
            m_enable_smooth_lod_transition,
            m_enable_bump_mapping,
            m_animate_cross_fading,
            m_enable_subsurface_scattering,
            m_billboard_transition_cross_fade_width,
            m_fade_out_width,
            m_selected_wind_quality,
            m_scale_factor,
            m_enable_shadow_casting,
            m_material_version,
            m_enable_shadow_receiving,
            m_user_data,
            m_enable_light_probes,
            m_asset_bundle_name,
            m_material_location,
            m_motion_vector_mode_enum_value,
            m_reflection_probe_enum_value,
            m_asset_bundle_variant,
            m_materials,
            m_supports_embedded_materials,
            m_file_id_type,
            m_unit_conversion_enum_value,
            m_generate_colliders,
            m_generate_rigidbody,
        })
    }
}
