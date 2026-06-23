//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::clip_animation_info::ClipAnimationInfo;
use crate::subclasses::md_four::MdFour;

#[derive(Debug, Clone)]
pub struct ModelImporter {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_file_id_to_recycle_name: Vec<(i32, String)>,
    pub m_old_hash_identity: MdFour,
    pub m_new_hash_identity: MdFour,
    pub m_generate_materials: i32,
    pub m_generate_animations: i32,
    pub m_global_scale: f32,
    pub m_bake_simulation: bool,
    pub m_split_animations: bool,
    pub m_add_colliders: bool,
    pub m_use_file_units: bool,
    pub m_animation_compression: i32,
    pub m_animation_rotation_error: f32,
    pub m_animation_position_error: f32,
    pub m_animation_scale_error: f32,
    pub m_mesh_compression: i32,
    pub m_mesh_settings_swap_uv_channels: bool,
    pub m_mesh_settings_generate_secondary_uv: bool,
    pub m_mesh_settings_secondary_uv_angle_distortion: f32,
    pub m_mesh_settings_secondary_uv_area_distortion: f32,
    pub m_mesh_settings_secondary_uv_hard_angle: f32,
    pub m_mesh_settings_secondary_uv_pack_margin: f32,
    pub m_mesh_settings_normal_import_mode: i32,
    pub m_mesh_settings_tangent_import_mode: i32,
    pub m_animation_wrap_mode: i32,
    pub m_clip_animations: Vec<ClipAnimationInfo>,
    pub m_first_import_version: i32,
    pub normal_smooth_angle: f32,
    pub split_tangents_across_uv: bool,
    pub m_imported_roots: Vec<PPtr>,
    pub m_has_extra_root: bool,
}

impl crate::Parse for ModelImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_file_id_to_recycle_name = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_old_hash_identity = MdFour::parse(r, ver)?;
        r.align(4)?;
        let m_new_hash_identity = MdFour::parse(r, ver)?;
        r.align(4)?;
        let m_generate_materials = r.read_i32()?;
        let m_generate_animations = r.read_i32()?;
        let m_global_scale = r.read_f32()?;
        let m_bake_simulation = r.read_bool()?;
        let m_split_animations = r.read_bool()?;
        let m_add_colliders = r.read_bool()?;
        let m_use_file_units = r.read_bool()?;
        r.align(4)?;
        let m_animation_compression = r.read_i32()?;
        let m_animation_rotation_error = r.read_f32()?;
        let m_animation_position_error = r.read_f32()?;
        let m_animation_scale_error = r.read_f32()?;
        let m_mesh_compression = r.read_i32()?;
        let m_mesh_settings_swap_uv_channels = r.read_bool()?;
        let m_mesh_settings_generate_secondary_uv = r.read_bool()?;
        r.align(4)?;
        let m_mesh_settings_secondary_uv_angle_distortion = r.read_f32()?;
        let m_mesh_settings_secondary_uv_area_distortion = r.read_f32()?;
        let m_mesh_settings_secondary_uv_hard_angle = r.read_f32()?;
        let m_mesh_settings_secondary_uv_pack_margin = r.read_f32()?;
        r.align(4)?;
        let m_mesh_settings_normal_import_mode = r.read_i32()?;
        let m_mesh_settings_tangent_import_mode = r.read_i32()?;
        let m_animation_wrap_mode = r.read_i32()?;
        let m_clip_animations = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClipAnimationInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_first_import_version = r.read_i32()?;
        let normal_smooth_angle = r.read_f32()?;
        let split_tangents_across_uv = r.read_bool()?;
        r.align(4)?;
        let m_imported_roots = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_has_extra_root = r.read_bool()?;

        Ok(ModelImporter {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_file_id_to_recycle_name,
            m_old_hash_identity,
            m_new_hash_identity,
            m_generate_materials,
            m_generate_animations,
            m_global_scale,
            m_bake_simulation,
            m_split_animations,
            m_add_colliders,
            m_use_file_units,
            m_animation_compression,
            m_animation_rotation_error,
            m_animation_position_error,
            m_animation_scale_error,
            m_mesh_compression,
            m_mesh_settings_swap_uv_channels,
            m_mesh_settings_generate_secondary_uv,
            m_mesh_settings_secondary_uv_angle_distortion,
            m_mesh_settings_secondary_uv_area_distortion,
            m_mesh_settings_secondary_uv_hard_angle,
            m_mesh_settings_secondary_uv_pack_margin,
            m_mesh_settings_normal_import_mode,
            m_mesh_settings_tangent_import_mode,
            m_animation_wrap_mode,
            m_clip_animations,
            m_first_import_version,
            normal_smooth_angle,
            split_tangents_across_uv,
            m_imported_roots,
            m_has_extra_root,
        })
    }
}
