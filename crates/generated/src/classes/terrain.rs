//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::vector4f::Vector4f;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct Terrain {
    pub parent: Behaviour,
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
    pub m_heightmap_minimum_lod_simplification: Option<i32>,
    pub m_cast_shadows: Option<bool>,
    pub m_shadow_casting_mode: Option<i32>,
    pub m_draw_heightmap: bool,
    pub m_draw_trees_and_foliage: bool,
    pub m_draw_instanced: Option<bool>,
    pub m_reflection_probe_usage: i32,
    pub m_material_type: Option<i32>,
    pub m_static_shadow_caster: Option<bool>,
    pub m_legacy_specular: Option<ColorRgba>,
    pub m_material_template: PPtr,
    pub m_ignore_quality_settings: Option<bool>,
    pub m_legacy_shininess: Option<f32>,
    pub m_lightmap_index: u16,
    pub m_use_default_smoothness: Option<bool>,
    pub m_lightmap_index_dynamic: u16,
    pub m_default_smoothness: Option<f32>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_tiling_offset_dynamic: Vector4f,
    pub m_explicit_probe_set_hash: Option<Hash128>,
    pub m_bake_light_probes_for_trees: bool,
    pub m_preserve_tree_prototype_layers: Option<bool>,
    pub m_dynamic_uvst: Vector4f,
    pub m_chunk_dynamic_uvst: Vector4f,
    pub m_grouping_id: Option<i32>,
    pub m_allow_auto_connect: Option<bool>,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_enable_heightmap_ray_tracing: Option<bool>,
    pub m_enable_trees_and_details_ray_tracing: Option<bool>,
    pub m_tree_motion_vector_mode_override: Option<i32>,
    pub m_enable_heightmap_lod_frustum_culling: Option<bool>,
}

impl crate::Parse for Terrain {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
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
        let m_heightmap_minimum_lod_simplification = if *ver >= crate::UnityVersion::new(2022, 3, 31) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_cast_shadows = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_shadow_casting_mode = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_draw_heightmap = r.read_bool()?;
        let m_draw_trees_and_foliage = r.read_bool()?;
        r.align(4)?;
        let m_draw_instanced = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_reflection_probe_usage = r.read_i32()?;
        let m_material_type = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_static_shadow_caster = if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_legacy_specular = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(ColorRgba::parse(r, ver)?)
        } else {
            None
        };
        let m_material_template = crate::PPtr::parse(r, ver)?;
        let m_ignore_quality_settings = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_legacy_shininess = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_lightmap_index = r.read_u16()?;
        let m_use_default_smoothness = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_lightmap_index_dynamic = r.read_u16()?;
        let m_default_smoothness = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_lightmap_tiling_offset = Vector4f::parse(r, ver)?;
        let m_lightmap_tiling_offset_dynamic = Vector4f::parse(r, ver)?;
        let m_explicit_probe_set_hash = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            Some(Hash128::parse(r, ver)?)
        } else {
            None
        };
        let m_bake_light_probes_for_trees = r.read_bool()?;
        r.align(4)?;
        let m_preserve_tree_prototype_layers = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_dynamic_uvst = Vector4f::parse(r, ver)?;
        let m_chunk_dynamic_uvst = Vector4f::parse(r, ver)?;
        let m_grouping_id = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_allow_auto_connect = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_rendering_layer_mask = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_enable_heightmap_ray_tracing = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_enable_trees_and_details_ray_tracing = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_tree_motion_vector_mode_override = if *ver >= crate::UnityVersion::new(2022, 3, 3) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_enable_heightmap_lod_frustum_culling = if *ver >= crate::UnityVersion::new(6000, 0, 76) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(Terrain {
            parent,
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
            m_heightmap_minimum_lod_simplification,
            m_cast_shadows,
            m_shadow_casting_mode,
            m_draw_heightmap,
            m_draw_trees_and_foliage,
            m_draw_instanced,
            m_reflection_probe_usage,
            m_material_type,
            m_static_shadow_caster,
            m_legacy_specular,
            m_material_template,
            m_ignore_quality_settings,
            m_legacy_shininess,
            m_lightmap_index,
            m_use_default_smoothness,
            m_lightmap_index_dynamic,
            m_default_smoothness,
            m_lightmap_tiling_offset,
            m_lightmap_tiling_offset_dynamic,
            m_explicit_probe_set_hash,
            m_bake_light_probes_for_trees,
            m_preserve_tree_prototype_layers,
            m_dynamic_uvst,
            m_chunk_dynamic_uvst,
            m_grouping_id,
            m_allow_auto_connect,
            m_rendering_layer_mask,
            m_enable_heightmap_ray_tracing,
            m_enable_trees_and_details_ray_tracing,
            m_tree_motion_vector_mode_override,
            m_enable_heightmap_lod_frustum_culling,
        })
    }
}
