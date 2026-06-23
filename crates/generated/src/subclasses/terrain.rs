//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct Terrain {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_terrain_data: PPtr,
    pub m_tree_distance: f32,
    pub m_tree_billboard_distance: f32,
    pub m_tree_cross_fade_length: f32,
    pub m_tree_maximum_full_lod_count: i32,
    pub m_detail_object_distance: f32,
    pub m_detail_object_density: f32,
    pub m_heightmap_pixel_error: f32,
    pub m_splat_map_distance: f32,
    pub m_heightmap_maximum_lod: i32,
    pub m_cast_shadows: bool,
    pub m_draw_heightmap: bool,
    pub m_draw_trees_and_foliage: bool,
    pub m_reflection_probe_usage: i32,
    pub m_material_type: i32,
    pub m_legacy_specular: ColorRgba,
    pub m_legacy_shininess: f32,
    pub m_use_default_smoothness: bool,
    pub m_default_smoothness: f32,
    pub m_material_template: PPtr,
    pub m_bake_light_probes_for_trees: bool,
    pub m_scale_in_lightmap: f32,
    pub m_lightmap_parameters: PPtr,
}

impl crate::Parse for Terrain {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_terrain_data = crate::PPtr::parse(r, ver)?;
        let m_tree_distance = r.read_f32()?;
        let m_tree_billboard_distance = r.read_f32()?;
        let m_tree_cross_fade_length = r.read_f32()?;
        let m_tree_maximum_full_lod_count = r.read_i32()?;
        let m_detail_object_distance = r.read_f32()?;
        let m_detail_object_density = r.read_f32()?;
        let m_heightmap_pixel_error = r.read_f32()?;
        let m_splat_map_distance = r.read_f32()?;
        let m_heightmap_maximum_lod = r.read_i32()?;
        let m_cast_shadows = r.read_bool()?;
        let m_draw_heightmap = r.read_bool()?;
        let m_draw_trees_and_foliage = r.read_bool()?;
        r.align(4)?;
        let m_reflection_probe_usage = r.read_i32()?;
        let m_material_type = r.read_i32()?;
        let m_legacy_specular = ColorRgba::parse(r, ver)?;
        let m_legacy_shininess = r.read_f32()?;
        let m_use_default_smoothness = r.read_bool()?;
        r.align(4)?;
        let m_default_smoothness = r.read_f32()?;
        let m_material_template = crate::PPtr::parse(r, ver)?;
        let m_bake_light_probes_for_trees = r.read_bool()?;
        r.align(4)?;
        let m_scale_in_lightmap = r.read_f32()?;
        let m_lightmap_parameters = crate::PPtr::parse(r, ver)?;
        r.align(4)?;

        Ok(Terrain {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_terrain_data,
            m_tree_distance,
            m_tree_billboard_distance,
            m_tree_cross_fade_length,
            m_tree_maximum_full_lod_count,
            m_detail_object_distance,
            m_detail_object_density,
            m_heightmap_pixel_error,
            m_splat_map_distance,
            m_heightmap_maximum_lod,
            m_cast_shadows,
            m_draw_heightmap,
            m_draw_trees_and_foliage,
            m_reflection_probe_usage,
            m_material_type,
            m_legacy_specular,
            m_legacy_shininess,
            m_use_default_smoothness,
            m_default_smoothness,
            m_material_template,
            m_bake_light_probes_for_trees,
            m_scale_in_lightmap,
            m_lightmap_parameters,
        })
    }
}
