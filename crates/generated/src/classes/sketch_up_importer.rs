//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::clip_animation_info::ClipAnimationInfo;
use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::human_description::HumanDescription;
use crate::subclasses::sketch_up_import_data::SketchUpImportData;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::take_info::TakeInfo;

#[derive(Debug, Clone)]
pub struct SketchUpImporter {
    pub m_generate_back_face: bool,
    pub m_name: String,
    pub m_merge_coplanar_faces: bool,
    pub m_internal_id_to_name_table: Option<Vec<((i32, i64), String)>>,
    pub m_selected_nodes: Vec<i32>,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_asset_hash: Hash128,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_longitude: f64,
    pub m_import_materials: Option<bool>,
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
    pub m_optimize_game_objects: bool,
    pub m_auto_map_external_materials: Option<bool>,
    pub m_motion_node_name: String,
    pub m_remove_constant_scale_curves: Option<bool>,
    pub m_rig_import_errors: Option<String>,
    pub m_animation_import_errors: String,
    pub m_resample_rotations: Option<bool>,
    pub m_rig_import_warnings: Option<String>,
    pub m_animation_import_warnings: String,
    pub m_animation_retargeting_warnings: String,
    pub m_animation_do_retargeting_warnings: bool,
    pub m_import_animated_custom_properties: Option<bool>,
    pub m_import_constraints: Option<bool>,
    pub m_animation_compression: i32,
    pub m_animation_rotation_error: f32,
    pub m_animation_position_error: f32,
    pub m_animation_scale_error: f32,
    pub m_animation_wrap_mode: i32,
    pub m_extra_exposed_transform_paths: Vec<String>,
    pub m_clip_animations: Vec<ClipAnimationInfo>,
    pub m_extra_user_properties: Option<Vec<String>>,
    pub m_is_readable: bool,
    pub m_lod_screen_percentages: Vec<f32>,
    pub m_global_scale: f32,
    pub m_mesh_compression: i32,
    pub m_add_colliders: bool,
    pub m_import_blend_shapes: bool,
    pub swap_uv_channels: bool,
    pub m_use_srgb_material_color: Option<bool>,
    pub generate_secondary_uv: bool,
    pub m_sort_hierarchy_by_name: Option<bool>,
    pub m_use_file_units: bool,
    pub m_import_visibility: Option<bool>,
    pub m_import_physical_cameras: Option<bool>,
    pub optimize_mesh_for_gpu: Option<bool>,
    pub keep_quads: bool,
    pub m_import_cameras: Option<bool>,
    pub weld_vertices: bool,
    pub m_import_lights: Option<bool>,
    pub secondary_uv_angle_distortion: f32,
    pub m_node_name_collision_strategy: Option<i32>,
    pub secondary_uv_area_distortion: f32,
    pub m_file_ids_generation: Option<i32>,
    pub secondary_uv_hard_angle: f32,
    pub secondary_uv_pack_margin: f32,
    pub m_use_file_scale: bool,
    pub m_file_scale: f32,
    pub normal_smooth_angle: f32,
    pub m_preserve_hierarchy: Option<bool>,
    pub bake_axis_conversion: Option<bool>,
    pub split_tangents_across_uv: Option<bool>,
    pub skin_weights_mode: Option<i32>,
    pub normal_import_mode: i32,
    pub max_bones_per_vertex: Option<i32>,
    pub tangent_import_mode: i32,
    pub min_bone_weight: Option<f32>,
    pub m_imported_take_infos: Vec<TakeInfo>,
    pub mesh_optimization_flags: Option<i32>,
    pub m_referenced_clips: Vec<Guid>,
    pub index_format: Option<i32>,
    pub m_imported_roots: Vec<PPtr>,
    pub optimize_bones: Option<bool>,
    pub m_has_extra_root: bool,
    pub m_import_animation: bool,
    pub generate_mesh_lods: Option<bool>,
    pub m_copy_avatar: Option<bool>,
    pub normal_calculation_mode: Option<i32>,
    pub secondary_uv_margin_method: Option<i32>,
    pub mesh_lod_generation_flags: Option<i32>,
    pub m_human_description: HumanDescription,
    pub secondary_uv_min_lightmap_resolution: Option<f32>,
    pub maximum_mesh_lod: Option<i32>,
    pub m_last_human_description_avatar_source: PPtr,
    pub secondary_uv_min_object_scale: Option<f32>,
    pub import_u_vs: Option<i32>,
    pub m_animation_type: i32,
    pub m_file_scale_unit: Option<String>,
    pub import_vertex_colors: Option<bool>,
    pub m_additional_bone: bool,
    pub m_humanoid_oversampling: Option<i32>,
    pub m_file_scale_factor: Option<f32>,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_previous_calculated_global_scale: Option<f32>,
    pub m_asset_bundle_variant: String,
    pub m_has_previous_calculated_global_scale: Option<bool>,
    pub legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes: Option<bool>,
    pub blend_shape_normal_import_mode: Option<i32>,
    pub m_has_embedded_textures: Option<bool>,
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

impl crate::Parse for SketchUpImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
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
        let mut m_auto_map_external_materials = None;
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
        let mut m_import_visibility = None;
        let mut m_import_physical_cameras = None;
        let mut optimize_mesh_for_gpu = None;
        let mut keep_quads = None;
        let mut m_import_cameras = None;
        let mut weld_vertices = None;
        let mut m_import_lights = None;
        let mut secondary_uv_angle_distortion = None;
        let mut m_node_name_collision_strategy = None;
        let mut secondary_uv_area_distortion = None;
        let mut m_file_ids_generation = None;
        let mut secondary_uv_hard_angle = None;
        let mut secondary_uv_pack_margin = None;
        let mut m_use_file_scale = None;
        let mut m_file_scale = None;
        let mut normal_smooth_angle = None;
        let mut m_preserve_hierarchy = None;
        let mut bake_axis_conversion = None;
        let mut split_tangents_across_uv = None;
        let mut skin_weights_mode = None;
        let mut normal_import_mode = None;
        let mut max_bones_per_vertex = None;
        let mut tangent_import_mode = None;
        let mut min_bone_weight = None;
        let mut m_imported_take_infos = None;
        let mut mesh_optimization_flags = None;
        let mut m_referenced_clips = None;
        let mut index_format = None;
        let mut m_imported_roots = None;
        let mut optimize_bones = None;
        let mut m_has_extra_root = None;
        let mut m_import_animation = None;
        let mut generate_mesh_lods = None;
        let mut m_copy_avatar = None;
        let mut normal_calculation_mode = None;
        let mut secondary_uv_margin_method = None;
        let mut mesh_lod_generation_flags = None;
        let mut m_human_description = None;
        let mut secondary_uv_min_lightmap_resolution = None;
        let mut maximum_mesh_lod = None;
        let mut m_last_human_description_avatar_source = None;
        let mut secondary_uv_min_object_scale = None;
        let mut import_u_vs = None;
        let mut m_animation_type = None;
        let mut m_file_scale_unit = None;
        let mut import_vertex_colors = None;
        let mut m_additional_bone = None;
        let mut m_humanoid_oversampling = None;
        let mut m_file_scale_factor = None;
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
            m_generate_back_face = Some(r.read_bool()?);
            m_merge_coplanar_faces = Some(r.read_bool()?);
            r.align(4)?;
            m_selected_nodes = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_asset_hash = Some(Hash128::parse(r, ver)?);
            m_longitude = Some(r.read_f64()?);
            m_latitude = Some(r.read_f64()?);
            m_north_correction = Some(r.read_f64()?);
            m_file_unit = Some(r.read_i32()?);
            r.align(4)?;
            m_sketch_up_import_data = Some(SketchUpImportData::parse(r, ver)?);
            r.align(4)?;
        }
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_internal_id_to_name_table = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = r.read_i32()?; let b = r.read_i64()?; Ok((a, b)) } )?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_supports_embedded_materials = Some(r.read_bool()?);
            r.align(4)?;
            m_material_import_mode = Some(r.read_i32()?);
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
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_file_id_to_recycle_name = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i64()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_import_materials = Some(r.read_bool()?);
            m_supports_embedded_materials = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_file_id_to_recycle_name = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i64()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_import_materials = Some(r.read_bool()?);
            m_auto_map_external_materials = Some(r.read_bool()?);
            r.align(4)?;
        } else {
            m_file_id_to_recycle_name = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i64()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_import_materials = Some(r.read_bool()?);
        }
            m_material_name = Some(r.read_i32()?);
            m_material_search = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_material_location = Some(r.read_i32()?);
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_material_location = Some(r.read_i32()?);
            m_search_textures_globally = Some(r.read_bool()?);
            r.align(4)?;
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_material_location = Some(r.read_i32()?);
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SourceAssetIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else {
        }
            m_legacy_generate_animations = Some(r.read_i32()?);
            m_bake_simulation = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_resample_curves = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_resample_rotations = Some(r.read_bool()?);
        } else {
        }
            m_optimize_game_objects = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_remove_constant_scale_curves = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }
            m_motion_node_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_rig_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_rig_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else {
        }
            m_animation_import_errors = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_import_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_retargeting_warnings = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_animation_do_retargeting_warnings = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_import_animated_custom_properties = Some(r.read_bool()?);
            m_import_constraints = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_import_animated_custom_properties = Some(r.read_bool()?);
        } else {
        }
            m_animation_compression = Some(r.read_i32()?);
            m_animation_rotation_error = Some(r.read_f32()?);
            m_animation_position_error = Some(r.read_f32()?);
            m_animation_scale_error = Some(r.read_f32()?);
            m_animation_wrap_mode = Some(r.read_i32()?);
            m_extra_exposed_transform_paths = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_extra_user_properties = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else {
        }
            m_clip_animations = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClipAnimationInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_is_readable = Some(r.read_bool()?);
            r.align(4)?;
            m_lod_screen_percentages = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_global_scale = Some(r.read_f32()?);
            m_mesh_compression = Some(r.read_i32()?);
            m_add_colliders = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_physical_cameras = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_physical_cameras = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 19) {
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_physical_cameras = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_sort_hierarchy_by_name = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_use_srgb_material_color = Some(r.read_bool()?);
            m_import_visibility = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_import_visibility = Some(r.read_bool()?);
        } else {
        }
            m_import_blend_shapes = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_node_name_collision_strategy = Some(r.read_i32()?);
            m_file_ids_generation = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_file_ids_generation = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 17) {
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
            m_file_ids_generation = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_import_cameras = Some(r.read_bool()?);
            m_import_lights = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }
            swap_uv_channels = Some(r.read_bool()?);
            generate_secondary_uv = Some(r.read_bool()?);
            m_use_file_units = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else {
            optimize_mesh_for_gpu = Some(r.read_bool()?);
        }
            keep_quads = Some(r.read_bool()?);
            weld_vertices = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
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
        } else if *ver >= crate::UnityVersion::new(6000, 2, 0) {
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
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            optimize_bones = Some(r.read_bool()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            bake_axis_conversion = Some(r.read_bool()?);
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            skin_weights_mode = Some(r.read_i32()?);
            max_bones_per_vertex = Some(r.read_i32()?);
            min_bone_weight = Some(r.read_f32()?);
            mesh_optimization_flags = Some(r.read_i32()?);
            index_format = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_preserve_hierarchy = Some(r.read_bool()?);
            r.align(4)?;
            index_format = Some(r.read_i32()?);
        } else {
        }
            secondary_uv_angle_distortion = Some(r.read_f32()?);
            secondary_uv_area_distortion = Some(r.read_f32()?);
            secondary_uv_hard_angle = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            secondary_uv_margin_method = Some(r.read_i32()?);
            secondary_uv_min_lightmap_resolution = Some(r.read_f32()?);
            secondary_uv_min_object_scale = Some(r.read_f32()?);
        } else {
        }
            secondary_uv_pack_margin = Some(r.read_f32()?);
            m_use_file_scale = Some(r.read_bool()?);
            r.align(4)?;
            m_file_scale = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_strict_vertex_data_checks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 36) {
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_strict_vertex_data_checks = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_file_scale_unit = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_file_scale_factor = Some(r.read_f32()?);
            r.align(4)?;
            m_previous_calculated_global_scale = Some(r.read_f32()?);
            m_has_previous_calculated_global_scale = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_previous_calculated_global_scale = Some(r.read_f32()?);
            m_has_previous_calculated_global_scale = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }
            normal_smooth_angle = Some(r.read_f32()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
        } else {
            split_tangents_across_uv = Some(r.read_bool()?);
            r.align(4)?;
        }
            normal_import_mode = Some(r.read_i32()?);
            tangent_import_mode = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            normal_calculation_mode = Some(r.read_i32()?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes = Some(r.read_bool()?);
            r.align(4)?;
            blend_shape_normal_import_mode = Some(r.read_i32()?);
            normal_smoothing_source = Some(r.read_i32()?);
            calculate_blendshape_normals_delta_from_imported_normals = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            normal_calculation_mode = Some(r.read_i32()?);
            legacy_compute_all_normals_from_smoothing_groups_when_mesh_has_blend_shapes = Some(r.read_bool()?);
            r.align(4)?;
            blend_shape_normal_import_mode = Some(r.read_i32()?);
            normal_smoothing_source = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            normal_calculation_mode = Some(r.read_i32()?);
        } else {
        }
            m_imported_take_infos = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { TakeInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else {
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_imported_roots = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_has_extra_root = Some(r.read_bool()?);
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_contains_animation = Some(r.read_bool()?);
            r.align(4)?;
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
            m_referenced_clips = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_has_embedded_textures = Some(r.read_bool()?);
        } else {
        }
            m_import_animation = Some(r.read_bool()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
        } else {
            m_copy_avatar = Some(r.read_bool()?);
            r.align(4)?;
        }
            m_human_description = Some(HumanDescription::parse(r, ver)?);
            r.align(4)?;
            m_last_human_description_avatar_source = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_auto_generate_avatar_mapping_if_unspecified = Some(r.read_bool()?);
            r.align(4)?;
        } else {
        }
            m_animation_type = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(2022, 1, 5) {
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_import_blend_shape_deform_percent = Some(r.read_bool()?);
            m_remap_materials_if_material_import_mode_is_none = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_import_blend_shape_deform_percent = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 5) {
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_remap_materials_if_material_import_mode_is_none = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 37) {
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
            m_remap_materials_if_material_import_mode_is_none = Some(r.read_bool()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
            m_add_humanoid_extra_root_only_when_using_avatar = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_humanoid_oversampling = Some(r.read_i32()?);
            m_avatar_setup = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_humanoid_oversampling = Some(r.read_i32()?);
        } else {
        }
            m_additional_bone = Some(r.read_bool()?);
            r.align(4)?;
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_generate_back_face = Some(r.read_bool()?);
            m_merge_coplanar_faces = Some(r.read_bool()?);
            r.align(4)?;
            m_selected_nodes = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_asset_hash = Some(Hash128::parse(r, ver)?);
            m_longitude = Some(r.read_f64()?);
            m_latitude = Some(r.read_f64()?);
            m_north_correction = Some(r.read_f64()?);
            m_file_unit = Some(r.read_i32()?);
            r.align(4)?;
            m_sketch_up_import_data = Some(SketchUpImportData::parse(r, ver)?);
            r.align(4)?;
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
            m_auto_map_external_materials,
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
            m_import_visibility,
            m_import_physical_cameras,
            optimize_mesh_for_gpu,
            keep_quads: keep_quads.unwrap(),
            m_import_cameras,
            weld_vertices: weld_vertices.unwrap(),
            m_import_lights,
            secondary_uv_angle_distortion: secondary_uv_angle_distortion.unwrap(),
            m_node_name_collision_strategy,
            secondary_uv_area_distortion: secondary_uv_area_distortion.unwrap(),
            m_file_ids_generation,
            secondary_uv_hard_angle: secondary_uv_hard_angle.unwrap(),
            secondary_uv_pack_margin: secondary_uv_pack_margin.unwrap(),
            m_use_file_scale: m_use_file_scale.unwrap(),
            m_file_scale: m_file_scale.unwrap(),
            normal_smooth_angle: normal_smooth_angle.unwrap(),
            m_preserve_hierarchy,
            bake_axis_conversion,
            split_tangents_across_uv,
            skin_weights_mode,
            normal_import_mode: normal_import_mode.unwrap(),
            max_bones_per_vertex,
            tangent_import_mode: tangent_import_mode.unwrap(),
            min_bone_weight,
            m_imported_take_infos: m_imported_take_infos.unwrap(),
            mesh_optimization_flags,
            m_referenced_clips: m_referenced_clips.unwrap(),
            index_format,
            m_imported_roots: m_imported_roots.unwrap(),
            optimize_bones,
            m_has_extra_root: m_has_extra_root.unwrap(),
            m_import_animation: m_import_animation.unwrap(),
            generate_mesh_lods,
            m_copy_avatar,
            normal_calculation_mode,
            secondary_uv_margin_method,
            mesh_lod_generation_flags,
            m_human_description: m_human_description.unwrap(),
            secondary_uv_min_lightmap_resolution,
            maximum_mesh_lod,
            m_last_human_description_avatar_source: m_last_human_description_avatar_source.unwrap(),
            secondary_uv_min_object_scale,
            import_u_vs,
            m_animation_type: m_animation_type.unwrap(),
            m_file_scale_unit,
            import_vertex_colors,
            m_additional_bone: m_additional_bone.unwrap(),
            m_humanoid_oversampling,
            m_file_scale_factor,
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
