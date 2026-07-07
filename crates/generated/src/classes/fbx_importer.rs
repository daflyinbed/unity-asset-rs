//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::clip_animation_info::ClipAnimationInfo;
use crate::subclasses::guid::Guid;
use crate::subclasses::human_description::HumanDescription;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::take_info::TakeInfo;

#[derive(Debug, Clone)]
pub struct FbxImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i64, String)>>,
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
        let mut m_auto_map_external_materials = None;
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
        let mut m_import_visibility = None;
        let mut m_imported_roots = None;
        let mut tangent_import_mode = None;
        let mut m_has_extra_root = None;
        let mut m_imported_take_infos = None;
        let mut m_import_cameras = None;
        let mut m_referenced_clips = None;
        let mut m_import_lights = None;
        let mut m_use_file_scale = None;
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
        let mut m_node_name_collision_strategy = None;
        let mut m_file_ids_generation = None;
        let mut m_preserve_hierarchy = None;
        let mut index_format = None;
        let mut normal_calculation_mode = None;
        let mut skin_weights_mode = None;
        let mut bake_axis_conversion = None;
        let mut max_bones_per_vertex = None;
        let mut m_asset_bundle_name = None;
        let mut min_bone_weight = None;
        let mut m_asset_bundle_variant = None;
        let mut mesh_optimization_flags = None;
        let mut m_humanoid_oversampling = None;
        let mut m_previous_calculated_global_scale = None;
        let mut optimize_bones = None;
        let mut m_has_previous_calculated_global_scale = None;
        let mut m_file_scale_unit = None;
        let mut m_file_scale_factor = None;
        let mut generate_mesh_lods = None;
        let mut secondary_uv_margin_method = None;
        let mut mesh_lod_generation_flags = None;
        let mut secondary_uv_min_lightmap_resolution = None;
        let mut maximum_mesh_lod = None;
        let mut m_has_embedded_textures = None;
        let mut secondary_uv_min_object_scale = None;
        let mut import_u_vs = None;
        let mut import_vertex_colors = None;
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
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_internal_id_to_name_table = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = r.read_i32()?; let b = r.read_i64()?; Ok((a, b)) } )?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_supports_embedded_materials = Some(r.read_bool()?);
            r.align(4)?;
            m_material_import_mode = Some(r.read_i32()?);
            m_material_name = Some(r.read_i32()?);
            m_material_search = Some(r.read_i32()?);
            m_material_location = Some(r.read_i32()?);
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_legacy_generate_animations = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_internal_id_to_name_table = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = r.read_i32()?; let b = r.read_i64()?; Ok((a, b)) } )?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_supports_embedded_materials = Some(r.read_bool()?);
            r.align(4)?;
            m_material_import_mode = Some(r.read_i32()?);
            m_material_name = Some(r.read_i32()?);
            m_material_search = Some(r.read_i32()?);
            m_material_location = Some(r.read_i32()?);
            m_search_textures_globally = Some(r.read_bool()?);
            r.align(4)?;
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_legacy_generate_animations = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_internal_id_to_name_table = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = r.read_i32()?; let b = r.read_i64()?; Ok((a, b)) } )?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_supports_embedded_materials = Some(r.read_bool()?);
            r.align(4)?;
            m_material_import_mode = Some(r.read_i32()?);
            m_material_name = Some(r.read_i32()?);
            m_material_search = Some(r.read_i32()?);
            m_material_location = Some(r.read_i32()?);
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_legacy_generate_animations = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_internal_id_to_name_table = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = r.read_i32()?; let b = r.read_i64()?; Ok((a, b)) } )?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_import_materials = Some(r.read_bool()?);
            m_supports_embedded_materials = Some(r.read_bool()?);
            r.align(4)?;
            m_material_name = Some(r.read_i32()?);
            m_material_search = Some(r.read_i32()?);
            m_material_location = Some(r.read_i32()?);
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_legacy_generate_animations = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_file_id_to_recycle_name = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i64()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_import_materials = Some(r.read_bool()?);
            m_supports_embedded_materials = Some(r.read_bool()?);
            r.align(4)?;
            m_material_name = Some(r.read_i32()?);
            m_material_search = Some(r.read_i32()?);
            m_material_location = Some(r.read_i32()?);
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_legacy_generate_animations = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_file_id_to_recycle_name = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i64()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_import_materials = Some(r.read_bool()?);
            m_auto_map_external_materials = Some(r.read_bool()?);
            r.align(4)?;
            m_material_name = Some(r.read_i32()?);
            m_material_search = Some(r.read_i32()?);
            m_material_location = Some(r.read_i32()?);
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_legacy_generate_animations = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_file_id_to_recycle_name = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i64()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_import_materials = Some(r.read_bool()?);
            m_material_name = Some(r.read_i32()?);
            m_material_search = Some(r.read_i32()?);
            m_legacy_generate_animations = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_file_id_to_recycle_name = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i64()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_import_materials = Some(r.read_bool()?);
            m_material_name = Some(r.read_i32()?);
            m_material_search = Some(r.read_i32()?);
            m_generate_animations = Some(r.read_i32()?);
        } else {
            m_file_id_to_recycle_name = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i64()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_old_hash_identity = Some(MdFour::parse(r, ver)?);
            r.align(4)?;
            m_new_hash_identity = Some(MdFour::parse(r, ver)?);
            r.align(4)?;
            m_generate_materials = Some(r.read_i32()?);
            m_generate_animations = Some(r.read_i32()?);
            m_global_scale = Some(r.read_f32()?);
        }
            m_bake_simulation = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_resample_curves = Some(r.read_bool()?);
            m_optimize_game_objects = Some(r.read_bool()?);
            m_remove_constant_scale_curves = Some(r.read_bool()?);
            r.align(4)?;
            m_motion_node_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_retargeting_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_do_retargeting_warnings = Some(r.read_bool()?);
            m_import_animated_custom_properties = Some(r.read_bool()?);
            m_import_constraints = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_resample_curves = Some(r.read_bool()?);
            m_optimize_game_objects = Some(r.read_bool()?);
            m_remove_constant_scale_curves = Some(r.read_bool()?);
            r.align(4)?;
            m_motion_node_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_rig_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_rig_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_retargeting_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_do_retargeting_warnings = Some(r.read_bool()?);
            m_import_animated_custom_properties = Some(r.read_bool()?);
            m_import_constraints = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_resample_curves = Some(r.read_bool()?);
            m_optimize_game_objects = Some(r.read_bool()?);
            m_motion_node_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_rig_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_rig_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_retargeting_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_do_retargeting_warnings = Some(r.read_bool()?);
            m_import_animated_custom_properties = Some(r.read_bool()?);
            m_import_constraints = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_resample_curves = Some(r.read_bool()?);
            m_optimize_game_objects = Some(r.read_bool()?);
            m_motion_node_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_rig_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_rig_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_retargeting_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_do_retargeting_warnings = Some(r.read_bool()?);
            m_import_animated_custom_properties = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_resample_curves = Some(r.read_bool()?);
            m_optimize_game_objects = Some(r.read_bool()?);
            m_motion_node_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_rig_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_rig_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_retargeting_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_do_retargeting_warnings = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_resample_curves = Some(r.read_bool()?);
            m_optimize_game_objects = Some(r.read_bool()?);
            m_motion_node_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_retargeting_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_do_retargeting_warnings = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_resample_rotations = Some(r.read_bool()?);
            m_optimize_game_objects = Some(r.read_bool()?);
            m_motion_node_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_retargeting_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_do_retargeting_warnings = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_optimize_game_objects = Some(r.read_bool()?);
            m_motion_node_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_retargeting_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_do_retargeting_warnings = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_optimize_game_objects = Some(r.read_bool()?);
            m_motion_node_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_optimize_game_objects = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_split_animations = Some(r.read_bool()?);
            r.align(4)?;
        } else {
            m_split_animations = Some(r.read_bool()?);
            r.align(4)?;
            m_add_colliders = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
        }
            m_animation_compression = Some(r.read_i32()?);
            m_animation_rotation_error = Some(r.read_f32()?);
            m_animation_position_error = Some(r.read_f32()?);
            m_animation_scale_error = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_mesh_compression = Some(r.read_i32()?);
            m_mesh_settings_swap_uv_channels = Some(r.read_bool()?);
            m_mesh_settings_generate_secondary_uv = Some(r.read_bool()?);
            r.align(4)?;
            m_mesh_settings_secondary_uv_angle_distortion = Some(r.read_f32()?);
            m_mesh_settings_secondary_uv_area_distortion = Some(r.read_f32()?);
            m_mesh_settings_secondary_uv_hard_angle = Some(r.read_f32()?);
            m_mesh_settings_secondary_uv_pack_margin = Some(r.read_f32()?);
            r.align(4)?;
            m_mesh_settings_normal_import_mode = Some(r.read_i32()?);
            m_mesh_settings_tangent_import_mode = Some(r.read_i32()?);
        }
            m_animation_wrap_mode = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_extra_exposed_transform_paths = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_extra_user_properties = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_extra_exposed_transform_paths = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else {
        }
            m_clip_animations = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClipAnimationInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_physical_cameras = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            generate_mesh_lods = Some(r.read_bool()?);
            r.align(4)?;
            mesh_lod_generation_flags = Some(r.read_i32()?);
            maximum_mesh_lod = Some(r.read_i32()?);
            import_u_vs = Some(r.read_i32()?);
            import_vertex_colors = Some(r.read_bool()?);
            r.align(4)?;
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_strict_vertex_data_checks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_physical_cameras = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            generate_mesh_lods = Some(r.read_bool()?);
            r.align(4)?;
            mesh_lod_generation_flags = Some(r.read_i32()?);
            maximum_mesh_lod = Some(r.read_i32()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_strict_vertex_data_checks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_physical_cameras = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_strict_vertex_data_checks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_strict_vertex_data_checks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_physical_cameras = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_strict_vertex_data_checks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_strict_vertex_data_checks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 19) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_physical_cameras = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_strict_vertex_data_checks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_strict_vertex_data_checks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 36) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_strict_vertex_data_checks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 17) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_file_ids_generation = Some(r.read_i32()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_previous_calculated_global_scale = Some(r.read_f32()?);
            m_has_previous_calculated_global_scale = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            optimize_mesh_for_gpu = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_previous_calculated_global_scale = Some(r.read_f32()?);
            m_has_previous_calculated_global_scale = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            optimize_mesh_for_gpu = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
            m_previous_calculated_global_scale = Some(r.read_f32()?);
            m_has_previous_calculated_global_scale = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            optimize_mesh_for_gpu = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            index_format = Some(r.read_i32()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            optimize_mesh_for_gpu = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            optimize_mesh_for_gpu = Some(r.read_bool()?);
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            m_import_blend_shapes = Some(r.read_bool()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            optimize_mesh_for_gpu = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            optimize_mesh_for_gpu = Some(r.read_bool()?);
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
            optimize_mesh = Some(r.read_bool()?);
            r.align(4)?;
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
            secondary_uv_pack_margin = Some(r.read_f32()?);
        } else {
            m_first_import_version = Some(r.read_i32()?);
        }
            normal_smooth_angle = Some(r.read_f32()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            normal_import_mode = Some(r.read_i32()?);
            tangent_import_mode = Some(r.read_i32()?);
            normal_calculation_mode = Some(r.read_i32()?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes = Some(r.read_bool()?);
            r.align(4)?;
            blend_shape_normal_import_mode = Some(r.read_i32()?);
            normal_smoothing_source = Some(r.read_i32()?);
            calculate_blendshape_normals_delta_from_imported_normals = Some(r.read_bool()?);
            r.align(4)?;
            m_imported_take_infos = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { TakeInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            normal_import_mode = Some(r.read_i32()?);
            tangent_import_mode = Some(r.read_i32()?);
            normal_calculation_mode = Some(r.read_i32()?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes = Some(r.read_bool()?);
            r.align(4)?;
            blend_shape_normal_import_mode = Some(r.read_i32()?);
            normal_smoothing_source = Some(r.read_i32()?);
            m_imported_take_infos = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { TakeInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            normal_import_mode = Some(r.read_i32()?);
            tangent_import_mode = Some(r.read_i32()?);
            normal_calculation_mode = Some(r.read_i32()?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes = Some(r.read_bool()?);
            r.align(4)?;
            blend_shape_normal_import_mode = Some(r.read_i32()?);
            normal_smoothing_source = Some(r.read_i32()?);
            m_imported_take_infos = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { TakeInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            normal_import_mode = Some(r.read_i32()?);
            tangent_import_mode = Some(r.read_i32()?);
            normal_calculation_mode = Some(r.read_i32()?);
            m_imported_take_infos = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { TakeInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            normal_import_mode = Some(r.read_i32()?);
            tangent_import_mode = Some(r.read_i32()?);
            m_imported_take_infos = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { TakeInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            split_tangents_across_uv = Some(r.read_bool()?);
            r.align(4)?;
            normal_import_mode = Some(r.read_i32()?);
            tangent_import_mode = Some(r.read_i32()?);
            m_imported_take_infos = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { TakeInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            split_tangents_across_uv = Some(r.read_bool()?);
            r.align(4)?;
            normal_import_mode = Some(r.read_i32()?);
            tangent_import_mode = Some(r.read_i32()?);
        } else {
            split_tangents_across_uv = Some(r.read_bool()?);
            r.align(4)?;
        }
            m_imported_roots = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_has_extra_root = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_contains_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_import_blend_shape_deform_percent = Some(r.read_bool()?);
            m_remap_materials_if_material_import_mode_is_none = Some(r.read_bool()?);
            r.align(4)?;
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 5) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_import_blend_shape_deform_percent = Some(r.read_bool()?);
            m_remap_materials_if_material_import_mode_is_none = Some(r.read_bool()?);
            r.align(4)?;
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_import_blend_shape_deform_percent = Some(r.read_bool()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 5) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_remap_materials_if_material_import_mode_is_none = Some(r.read_bool()?);
            r.align(4)?;
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 3, 37) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_remap_materials_if_material_import_mode_is_none = Some(r.read_bool()?);
            r.align(4)?;
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_copy_avatar = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_copy_avatar = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_copy_avatar = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_animation_type = Some(r.read_i32()?);
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_copy_avatar = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_animation_type = Some(r.read_i32()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 3) {
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_copy_avatar = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_animation_type = Some(r.read_i32()?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_copy_avatar = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_animation_type = Some(r.read_i32()?);
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_copy_avatar = Some(r.read_bool()?);
            r.align(4)?;
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_animation_type = Some(r.read_i32()?);
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
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
            m_auto_map_external_materials,
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
            m_import_visibility,
            m_imported_roots: m_imported_roots.unwrap(),
            tangent_import_mode,
            m_has_extra_root: m_has_extra_root.unwrap(),
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
