//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::clip_animation_info::ClipAnimationInfo;
use crate::subclasses::guid::Guid;
use crate::subclasses::human_description::HumanDescription;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::take_info::TakeInfo;
use super::model_importer::ModelImporter;

#[derive(Debug, Clone)]
pub struct FbxImporter {
    pub parent: ModelImporter,
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_internal_id_to_name_table: Option<Vec<((i32, i64), String)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_import_materials: Option<bool>,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_material_name: Option<i32>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_generate_materials: Option<i32>,
    pub m_material_search: Option<i32>,
    pub m_auto_map_external_materials: Option<bool>,
    pub m_supports_embedded_materials: Option<bool>,
    pub m_generate_animations: Option<i32>,
    pub m_legacy_generate_animations: Option<i32>,
    pub m_material_import_mode: Option<i32>,
    pub m_global_scale: f32,
    pub m_bake_simulation: bool,
    pub m_split_animations: Option<bool>,
    pub m_animation_compression: i32,
    pub m_optimize_game_objects: Option<bool>,
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
    pub m_use_file_units: bool,
    pub m_animation_scale_error: f32,
    pub m_animation_import_warnings: Option<String>,
    pub m_rig_import_errors: Option<String>,
    pub m_animation_wrap_mode: i32,
    pub m_animation_retargeting_warnings: Option<String>,
    pub m_rig_import_warnings: Option<String>,
    pub m_clip_animations: Vec<ClipAnimationInfo>,
    pub m_animation_do_retargeting_warnings: Option<bool>,
    pub m_is_readable: Option<bool>,
    pub m_extra_exposed_transform_paths: Option<Vec<String>>,
    pub m_lod_screen_percentages: Option<Vec<f32>>,
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
    pub m_import_animated_custom_properties: Option<bool>,
    pub m_mesh_settings_secondary_uv_pack_margin: Option<f32>,
    pub optimize_mesh: Option<bool>,
    pub optimize_mesh_for_gpu: Option<bool>,
    pub m_import_constraints: Option<bool>,
    pub m_mesh_settings_normal_import_mode: Option<i32>,
    pub secondary_uv_angle_distortion: Option<f32>,
    pub m_extra_user_properties: Option<Vec<String>>,
    pub m_mesh_settings_tangent_import_mode: Option<i32>,
    pub secondary_uv_area_distortion: Option<f32>,
    pub weld_vertices: Option<bool>,
    pub secondary_uv_hard_angle: Option<f32>,
    pub secondary_uv_pack_margin: Option<f32>,
    pub m_first_import_version: Option<i32>,
    pub normal_smooth_angle: f32,
    pub keep_quads: Option<bool>,
    pub split_tangents_across_uv: Option<bool>,
    pub normal_import_mode: Option<i32>,
    pub m_import_visibility: Option<bool>,
    pub m_imported_roots: Vec<PPtr>,
    pub tangent_import_mode: Option<i32>,
    pub m_has_extra_root: bool,
    pub m_imported_take_infos: Option<Vec<TakeInfo>>,
    pub m_import_cameras: Option<bool>,
    pub m_referenced_clips: Option<Vec<Guid>>,
    pub m_import_lights: Option<bool>,
    pub m_use_file_scale: Option<bool>,
    pub m_file_scale: Option<f32>,
    pub m_import_animation: Option<bool>,
    pub m_copy_avatar: Option<bool>,
    pub m_use_srgb_material_color: Option<bool>,
    pub m_human_description: Option<HumanDescription>,
    pub m_sort_hierarchy_by_name: Option<bool>,
    pub m_last_human_description_avatar_source: Option<PPtr>,
    pub m_import_physical_cameras: Option<bool>,
    pub m_additional_bone: Option<bool>,
    pub m_animation_type: Option<i32>,
    pub m_user_data: Option<String>,
    pub m_node_name_collision_strategy: Option<i32>,
    pub m_file_ids_generation: Option<i32>,
    pub m_preserve_hierarchy: Option<bool>,
    pub index_format: Option<i32>,
    pub normal_calculation_mode: Option<i32>,
    pub skin_weights_mode: Option<i32>,
    pub bake_axis_conversion: Option<bool>,
    pub max_bones_per_vertex: Option<i32>,
    pub m_asset_bundle_name: Option<String>,
    pub min_bone_weight: Option<f32>,
    pub m_asset_bundle_variant: Option<String>,
    pub mesh_optimization_flags: Option<i32>,
    pub m_humanoid_oversampling: Option<i32>,
    pub m_previous_calculated_global_scale: Option<f32>,
    pub optimize_bones: Option<bool>,
    pub m_has_previous_calculated_global_scale: Option<bool>,
    pub m_file_scale_unit: Option<String>,
    pub m_file_scale_factor: Option<f32>,
    pub generate_mesh_lods: Option<bool>,
    pub secondary_uv_margin_method: Option<i32>,
    pub mesh_lod_generation_flags: Option<i32>,
    pub secondary_uv_min_lightmap_resolution: Option<f32>,
    pub maximum_mesh_lod: Option<i32>,
    pub m_has_embedded_textures: Option<bool>,
    pub secondary_uv_min_object_scale: Option<f32>,
    pub import_u_vs: Option<i32>,
    pub import_vertex_colors: Option<bool>,
    pub legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes: Option<bool>,
    pub blend_shape_normal_import_mode: Option<i32>,
    pub normal_smoothing_source: Option<i32>,
    pub m_strict_vertex_data_checks: Option<bool>,
    pub m_auto_generate_avatar_mapping_if_unspecified: Option<bool>,
    pub m_avatar_setup: Option<i32>,
    pub m_contains_animation: Option<bool>,
    pub calculate_blendshape_normals_delta_from_imported_normals: Option<bool>,
    pub m_add_humanoid_extra_root_only_when_using_avatar: Option<bool>,
    pub m_remap_materials_if_material_import_mode_is_none: Option<bool>,
    pub m_import_blend_shape_deform_percent: Option<bool>,
}

impl crate::Parse for FbxImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = ModelImporter::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_file_id_to_recycle_name = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_internal_id_to_name_table = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = r.read_i32()?; let b = r.read_i64()?; Ok((a, b)) } )?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_old_hash_identity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = MdFour::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_import_materials = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_external_objects = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_new_hash_identity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = MdFour::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_material_name = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_used_file_i_ds = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_generate_materials = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_material_search = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_auto_map_external_materials = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_supports_embedded_materials = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_generate_animations = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_legacy_generate_animations = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_material_import_mode = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_global_scale = r.read_f32()?;
        let m_bake_simulation = r.read_bool()?;
        let m_split_animations = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_animation_compression = r.read_i32()?;
        let m_optimize_game_objects = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_resample_rotations = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_resample_curves = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_material_location = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_animation_rotation_error = r.read_f32()?;
        let m_motion_node_name = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_materials = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_add_colliders = r.read_bool()?;
        let m_animation_position_error = r.read_f32()?;
        let m_animation_import_errors = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_search_textures_globally = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_use_file_units = r.read_bool()?;
        r.align(4)?;
        let m_animation_scale_error = r.read_f32()?;
        let m_animation_import_warnings = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_rig_import_errors = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_animation_wrap_mode = r.read_i32()?;
        let m_animation_retargeting_warnings = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_rig_import_warnings = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_clip_animations = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClipAnimationInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_animation_do_retargeting_warnings = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_readable = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_extra_exposed_transform_paths = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_lod_screen_percentages = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_remove_constant_scale_curves = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mesh_compression = r.read_i32()?;
        let m_mesh_settings_swap_uv_channels = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_mesh_settings_generate_secondary_uv = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mesh_settings_secondary_uv_angle_distortion = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let swap_uv_channels = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_import_blend_shapes = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_mesh_settings_secondary_uv_area_distortion = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let generate_secondary_uv = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_mesh_settings_secondary_uv_hard_angle = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_import_animated_custom_properties = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mesh_settings_secondary_uv_pack_margin = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_f32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let optimize_mesh = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let optimize_mesh_for_gpu = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_import_constraints = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mesh_settings_normal_import_mode = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let secondary_uv_angle_distortion = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_extra_user_properties = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mesh_settings_tangent_import_mode = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let secondary_uv_area_distortion = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let weld_vertices = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let secondary_uv_hard_angle = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let secondary_uv_pack_margin = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_first_import_version = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let normal_smooth_angle = r.read_f32()?;
        let keep_quads = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let split_tangents_across_uv = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let normal_import_mode = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_import_visibility = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_imported_roots = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let tangent_import_mode = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_has_extra_root = r.read_bool()?;
        let m_imported_take_infos = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TakeInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_import_cameras = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_referenced_clips = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_import_lights = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_use_file_scale = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_file_scale = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_f32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_import_animation = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_copy_avatar = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_use_srgb_material_color = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_human_description = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = HumanDescription::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sort_hierarchy_by_name = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_last_human_description_avatar_source = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_import_physical_cameras = if *ver >= crate::UnityVersion::new(2022, 2, 19) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_additional_bone = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_animation_type = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_user_data = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_node_name_collision_strategy = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_file_ids_generation = if *ver >= crate::UnityVersion::new(2019, 3, 17) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_preserve_hierarchy = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let index_format = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let normal_calculation_mode = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let skin_weights_mode = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let bake_axis_conversion = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let max_bones_per_vertex = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_asset_bundle_name = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let min_bone_weight = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_asset_bundle_variant = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let mesh_optimization_flags = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_humanoid_oversampling = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_previous_calculated_global_scale = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let optimize_bones = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_has_previous_calculated_global_scale = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_file_scale_unit = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_file_scale_factor = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_f32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let generate_mesh_lods = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let secondary_uv_margin_method = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let mesh_lod_generation_flags = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let secondary_uv_min_lightmap_resolution = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let maximum_mesh_lod = if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_has_embedded_textures = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let secondary_uv_min_object_scale = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let import_u_vs = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let import_vertex_colors = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let blend_shape_normal_import_mode = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let normal_smoothing_source = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_strict_vertex_data_checks = if *ver >= crate::UnityVersion::new(2021, 3, 36) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_auto_generate_avatar_mapping_if_unspecified = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_avatar_setup = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_contains_animation = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let calculate_blendshape_normals_delta_from_imported_normals = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_add_humanoid_extra_root_only_when_using_avatar = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_remap_materials_if_material_import_mode_is_none = if *ver >= crate::UnityVersion::new(2020, 3, 37) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_import_blend_shape_deform_percent = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(FbxImporter {
            parent,
            m_name,
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
            m_auto_map_external_materials,
            m_supports_embedded_materials,
            m_generate_animations,
            m_legacy_generate_animations,
            m_material_import_mode,
            m_global_scale,
            m_bake_simulation,
            m_split_animations,
            m_animation_compression,
            m_optimize_game_objects,
            m_resample_rotations,
            m_resample_curves,
            m_material_location,
            m_animation_rotation_error,
            m_motion_node_name,
            m_materials,
            m_add_colliders,
            m_animation_position_error,
            m_animation_import_errors,
            m_search_textures_globally,
            m_use_file_units,
            m_animation_scale_error,
            m_animation_import_warnings,
            m_rig_import_errors,
            m_animation_wrap_mode,
            m_animation_retargeting_warnings,
            m_rig_import_warnings,
            m_clip_animations,
            m_animation_do_retargeting_warnings,
            m_is_readable,
            m_extra_exposed_transform_paths,
            m_lod_screen_percentages,
            m_remove_constant_scale_curves,
            m_mesh_compression,
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
            normal_smooth_angle,
            keep_quads,
            split_tangents_across_uv,
            normal_import_mode,
            m_import_visibility,
            m_imported_roots,
            tangent_import_mode,
            m_has_extra_root,
            m_imported_take_infos,
            m_import_cameras,
            m_referenced_clips,
            m_import_lights,
            m_use_file_scale,
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
            m_node_name_collision_strategy,
            m_file_ids_generation,
            m_preserve_hierarchy,
            index_format,
            normal_calculation_mode,
            skin_weights_mode,
            bake_axis_conversion,
            max_bones_per_vertex,
            m_asset_bundle_name,
            min_bone_weight,
            m_asset_bundle_variant,
            mesh_optimization_flags,
            m_humanoid_oversampling,
            m_previous_calculated_global_scale,
            optimize_bones,
            m_has_previous_calculated_global_scale,
            m_file_scale_unit,
            m_file_scale_factor,
            generate_mesh_lods,
            secondary_uv_margin_method,
            mesh_lod_generation_flags,
            secondary_uv_min_lightmap_resolution,
            maximum_mesh_lod,
            m_has_embedded_textures,
            secondary_uv_min_object_scale,
            import_u_vs,
            import_vertex_colors,
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
