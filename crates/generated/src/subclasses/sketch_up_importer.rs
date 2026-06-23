//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::clip_animation_info::ClipAnimationInfo;
use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::human_description::HumanDescription;
use crate::subclasses::sketch_up_import_data::SketchUpImportData;
use crate::subclasses::take_info::TakeInfo;

#[derive(Debug, Clone)]
pub struct SketchUpImporter {
    pub m_generate_back_face: bool,
    pub m_merge_coplanar_faces: bool,
    pub m_selected_nodes: Vec<i32>,
    pub m_asset_hash: Hash128,
    pub m_longitude: f64,
    pub m_latitude: f64,
    pub m_north_correction: f64,
    pub m_file_unit: i32,
    pub m_sketch_up_import_data: SketchUpImportData,
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_file_id_to_recycle_name: Vec<(i64, String)>,
    pub m_import_materials: bool,
    pub m_material_name: i32,
    pub m_material_search: i32,
    pub m_legacy_generate_animations: i32,
    pub m_bake_simulation: bool,
    pub m_optimize_game_objects: bool,
    pub m_motion_node_name: String,
    pub m_animation_import_errors: String,
    pub m_animation_import_warnings: String,
    pub m_animation_retargeting_warnings: String,
    pub m_animation_do_retargeting_warnings: bool,
    pub m_animation_compression: i32,
    pub m_animation_rotation_error: f32,
    pub m_animation_position_error: f32,
    pub m_animation_scale_error: f32,
    pub m_animation_wrap_mode: i32,
    pub m_extra_exposed_transform_paths: Vec<String>,
    pub m_clip_animations: Vec<ClipAnimationInfo>,
    pub m_is_readable: bool,
    pub m_lod_screen_percentages: Vec<f32>,
    pub m_global_scale: f32,
    pub m_mesh_compression: i32,
    pub m_add_colliders: bool,
    pub m_import_blend_shapes: bool,
    pub swap_uv_channels: bool,
    pub generate_secondary_uv: bool,
    pub m_use_file_units: bool,
    pub optimize_mesh_for_gpu: bool,
    pub keep_quads: bool,
    pub weld_vertices: bool,
    pub secondary_uv_angle_distortion: f32,
    pub secondary_uv_area_distortion: f32,
    pub secondary_uv_hard_angle: f32,
    pub secondary_uv_pack_margin: f32,
    pub m_use_file_scale: bool,
    pub m_file_scale: f32,
    pub normal_smooth_angle: f32,
    pub split_tangents_across_uv: bool,
    pub normal_import_mode: i32,
    pub tangent_import_mode: i32,
    pub m_imported_take_infos: Vec<TakeInfo>,
    pub m_referenced_clips: Vec<Guid>,
    pub m_imported_roots: Vec<PPtr>,
    pub m_has_extra_root: bool,
    pub m_import_animation: bool,
    pub m_copy_avatar: bool,
    pub m_human_description: HumanDescription,
    pub m_last_human_description_avatar_source: PPtr,
    pub m_animation_type: i32,
    pub m_additional_bone: bool,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for SketchUpImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_generate_back_face = r.read_bool()?;
        let m_merge_coplanar_faces = r.read_bool()?;
        r.align(4)?;
        let m_selected_nodes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_asset_hash = Hash128::parse(r, ver)?;
        let m_longitude = r.read_f64()?;
        let m_latitude = r.read_f64()?;
        let m_north_correction = r.read_f64()?;
        let m_file_unit = r.read_i32()?;
        r.align(4)?;
        let m_sketch_up_import_data = SketchUpImportData::parse(r, ver)?;
        r.align(4)?;
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_file_id_to_recycle_name = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i64()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_import_materials = r.read_bool()?;
        r.align(4)?;
        let m_material_name = r.read_i32()?;
        let m_material_search = r.read_i32()?;
        let m_legacy_generate_animations = r.read_i32()?;
        let m_bake_simulation = r.read_bool()?;
        let m_optimize_game_objects = r.read_bool()?;
        r.align(4)?;
        let m_motion_node_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_animation_import_errors = r.read_aligned_string()?;
        r.align(4)?;
        let m_animation_import_warnings = r.read_aligned_string()?;
        r.align(4)?;
        let m_animation_retargeting_warnings = r.read_aligned_string()?;
        r.align(4)?;
        let m_animation_do_retargeting_warnings = r.read_bool()?;
        r.align(4)?;
        let m_animation_compression = r.read_i32()?;
        let m_animation_rotation_error = r.read_f32()?;
        let m_animation_position_error = r.read_f32()?;
        let m_animation_scale_error = r.read_f32()?;
        let m_animation_wrap_mode = r.read_i32()?;
        let m_extra_exposed_transform_paths = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_clip_animations = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClipAnimationInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_is_readable = r.read_bool()?;
        r.align(4)?;
        let m_lod_screen_percentages = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_global_scale = r.read_f32()?;
        let m_mesh_compression = r.read_i32()?;
        let m_add_colliders = r.read_bool()?;
        let m_import_blend_shapes = r.read_bool()?;
        let swap_uv_channels = r.read_bool()?;
        let generate_secondary_uv = r.read_bool()?;
        let m_use_file_units = r.read_bool()?;
        let optimize_mesh_for_gpu = r.read_bool()?;
        let keep_quads = r.read_bool()?;
        let weld_vertices = r.read_bool()?;
        r.align(4)?;
        let secondary_uv_angle_distortion = r.read_f32()?;
        let secondary_uv_area_distortion = r.read_f32()?;
        let secondary_uv_hard_angle = r.read_f32()?;
        let secondary_uv_pack_margin = r.read_f32()?;
        let m_use_file_scale = r.read_bool()?;
        r.align(4)?;
        let m_file_scale = r.read_f32()?;
        r.align(4)?;
        let normal_smooth_angle = r.read_f32()?;
        let split_tangents_across_uv = r.read_bool()?;
        r.align(4)?;
        let normal_import_mode = r.read_i32()?;
        let tangent_import_mode = r.read_i32()?;
        r.align(4)?;
        let m_imported_take_infos = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TakeInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_referenced_clips = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_imported_roots = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_has_extra_root = r.read_bool()?;
        r.align(4)?;
        let m_import_animation = r.read_bool()?;
        let m_copy_avatar = r.read_bool()?;
        r.align(4)?;
        let m_human_description = HumanDescription::parse(r, ver)?;
        r.align(4)?;
        let m_last_human_description_avatar_source = crate::PPtr::parse(r, ver)?;
        let m_animation_type = r.read_i32()?;
        let m_additional_bone = r.read_bool()?;
        r.align(4)?;
        let m_user_data = r.read_aligned_string()?;
        r.align(4)?;
        let m_asset_bundle_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_asset_bundle_variant = r.read_aligned_string()?;
        r.align(4)?;

        Ok(SketchUpImporter {
            m_generate_back_face,
            m_merge_coplanar_faces,
            m_selected_nodes,
            m_asset_hash,
            m_longitude,
            m_latitude,
            m_north_correction,
            m_file_unit,
            m_sketch_up_import_data,
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_file_id_to_recycle_name,
            m_import_materials,
            m_material_name,
            m_material_search,
            m_legacy_generate_animations,
            m_bake_simulation,
            m_optimize_game_objects,
            m_motion_node_name,
            m_animation_import_errors,
            m_animation_import_warnings,
            m_animation_retargeting_warnings,
            m_animation_do_retargeting_warnings,
            m_animation_compression,
            m_animation_rotation_error,
            m_animation_position_error,
            m_animation_scale_error,
            m_animation_wrap_mode,
            m_extra_exposed_transform_paths,
            m_clip_animations,
            m_is_readable,
            m_lod_screen_percentages,
            m_global_scale,
            m_mesh_compression,
            m_add_colliders,
            m_import_blend_shapes,
            swap_uv_channels,
            generate_secondary_uv,
            m_use_file_units,
            optimize_mesh_for_gpu,
            keep_quads,
            weld_vertices,
            secondary_uv_angle_distortion,
            secondary_uv_area_distortion,
            secondary_uv_hard_angle,
            secondary_uv_pack_margin,
            m_use_file_scale,
            m_file_scale,
            normal_smooth_angle,
            split_tangents_across_uv,
            normal_import_mode,
            tangent_import_mode,
            m_imported_take_infos,
            m_referenced_clips,
            m_imported_roots,
            m_has_extra_root,
            m_import_animation,
            m_copy_avatar,
            m_human_description,
            m_last_human_description_avatar_source,
            m_animation_type,
            m_additional_bone,
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}
