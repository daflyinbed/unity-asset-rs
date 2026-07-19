//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba32::ColorRgba32;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::vector4f::Vector4f;

/// 版本枚举：TerrainMDrawTreesAndFoliageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TerrainMDrawTreesAndFoliageVariant {
    Default(bool),
    V2021_1(bool),
}

impl crate::Parse for TerrainMDrawTreesAndFoliageVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TerrainMBakeLightProbesForTreesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TerrainMBakeLightProbesForTreesVariant {
    Default(bool),
    V2018_2(bool),
}

impl crate::Parse for TerrainMBakeLightProbesForTreesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TerrainMChunkDynamicUvstVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TerrainMChunkDynamicUvstVariant {
    Default(Box<Vector4f>),
    V2018_3(Box<Vector4f>),
}

impl crate::Parse for TerrainMChunkDynamicUvstVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = ({ Ok(Box::new(Vector4f::parse(rdr, ver)?)) })?;
            rdr.align(4)?;
            Ok(Self::V2018_3(v))
        } else {
            let v = ({ Ok(Box::new(Vector4f::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TerrainMEnableTreesAndDetailsRayTracingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TerrainMEnableTreesAndDetailsRayTracingVariant {
    Default(bool),
    V2022_3_3(bool),
    V2023_1(bool),
    V2023_1_2(bool),
    V6000_0_76(bool),
    V6000_1(bool),
    V6000_3_17(bool),
    V6000_4(bool),
    V6000_4_9(bool),
    V6000_6(bool),
}

impl crate::Parse for TerrainMEnableTreesAndDetailsRayTracingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V6000_6(v))
        } else if *ver >= crate::UnityVersion::new(6000, 4, 9) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_4_9(v))
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V6000_4(v))
        } else if *ver >= crate::UnityVersion::new(6000, 3, 17) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_3_17(v))
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V6000_1(v))
        } else if *ver >= crate::UnityVersion::new(6000, 0, 76) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_0_76(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 2) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2023_1_2(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2023_1(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 3) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2022_3_3(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Terrain {
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
    pub m_draw_trees_and_foliage: TerrainMDrawTreesAndFoliageVariant,
    pub m_draw_instanced: Option<bool>,
    pub m_reflection_probe_usage: i32,
    pub m_material_type: Option<i32>,
    pub m_static_shadow_caster: Option<bool>,
    pub m_legacy_specular: Option<ColorRgba32>,
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
    pub m_bake_light_probes_for_trees: TerrainMBakeLightProbesForTreesVariant,
    pub m_preserve_tree_prototype_layers: Option<bool>,
    pub m_dynamic_uvst: Vector4f,
    pub m_chunk_dynamic_uvst: TerrainMChunkDynamicUvstVariant,
    pub m_grouping_id: Option<i32>,
    pub m_allow_auto_connect: Option<bool>,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_enable_heightmap_ray_tracing: Option<bool>,
    pub m_enable_trees_and_details_ray_tracing:
        Option<TerrainMEnableTreesAndDetailsRayTracingVariant>,
    pub m_tree_motion_vector_mode_override: Option<i32>,
    pub m_enable_heightmap_lod_frustum_culling: Option<bool>,
}

impl crate::Parse for Terrain {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_terrain_data = None;
        let mut m_tree_distance = None;
        let mut m_tree_billboard_distance = None;
        let mut m_tree_cross_fade_length = None;
        let mut m_tree_maximum_full_lod_count = None;
        let mut m_detail_object_distance = None;
        let mut m_detail_object_density = None;
        let mut m_heightmap_pixel_error = None;
        let mut m_splat_map_distance = None;
        let mut m_heightmap_maximum_lod = None;
        let mut m_heightmap_minimum_lod_simplification = None;
        let mut m_cast_shadows = None;
        let mut m_shadow_casting_mode = None;
        let mut m_draw_heightmap = None;
        let mut m_draw_trees_and_foliage = None;
        let mut m_draw_instanced = None;
        let mut m_reflection_probe_usage = None;
        let mut m_material_type = None;
        let mut m_static_shadow_caster = None;
        let mut m_legacy_specular = None;
        let mut m_material_template = None;
        let mut m_ignore_quality_settings = None;
        let mut m_legacy_shininess = None;
        let mut m_lightmap_index = None;
        let mut m_use_default_smoothness = None;
        let mut m_lightmap_index_dynamic = None;
        let mut m_default_smoothness = None;
        let mut m_lightmap_tiling_offset = None;
        let mut m_lightmap_tiling_offset_dynamic = None;
        let mut m_explicit_probe_set_hash = None;
        let mut m_bake_light_probes_for_trees = None;
        let mut m_preserve_tree_prototype_layers = None;
        let mut m_dynamic_uvst = None;
        let mut m_chunk_dynamic_uvst = None;
        let mut m_grouping_id = None;
        let mut m_allow_auto_connect = None;
        let mut m_rendering_layer_mask = None;
        let mut m_enable_heightmap_ray_tracing = None;
        let mut m_enable_trees_and_details_ray_tracing = None;
        let mut m_tree_motion_vector_mode_override = None;
        let mut m_enable_heightmap_lod_frustum_culling = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_terrain_data = Some(crate::PPtr::parse(rdr, ver)?);
        m_tree_distance = Some(rdr.read_f32()?);
        m_tree_billboard_distance = Some(rdr.read_f32()?);
        m_tree_cross_fade_length = Some(rdr.read_f32()?);
        m_tree_maximum_full_lod_count = Some(rdr.read_i32()?);
        m_detail_object_distance = Some(rdr.read_f32()?);
        m_detail_object_density = Some(rdr.read_f32()?);
        m_heightmap_pixel_error = Some(rdr.read_f32()?);
        m_splat_map_distance = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 3, 31) {
            m_heightmap_minimum_lod_simplification = Some(rdr.read_i32()?);
        } else {
        }
        m_heightmap_maximum_lod = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_shadow_casting_mode = Some(rdr.read_i32()?);
        } else {
            m_cast_shadows = Some(rdr.read_bool()?);
        }
        m_draw_heightmap = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_draw_instanced = Some(rdr.read_bool()?);
        } else {
        }
        m_draw_trees_and_foliage = Some(TerrainMDrawTreesAndFoliageVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_static_shadow_caster = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_ignore_quality_settings = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_static_shadow_caster = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_reflection_probe_usage = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 0, 1) {
            m_material_type = Some(rdr.read_i32()?);
            m_legacy_specular = Some(ColorRgba32::parse(rdr, ver)?);
            m_legacy_shininess = Some(rdr.read_f32()?);
        } else {
            m_material_type = Some(rdr.read_i32()?);
            m_legacy_specular = Some(ColorRgba32::parse(rdr, ver)?);
            m_legacy_shininess = Some(rdr.read_f32()?);
            m_use_default_smoothness = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_default_smoothness = Some(rdr.read_f32()?);
        }
        m_material_template = Some(crate::PPtr::parse(rdr, ver)?);
        m_lightmap_index = Some(rdr.read_u16()?);
        m_lightmap_index_dynamic = Some(rdr.read_u16()?);
        m_lightmap_tiling_offset = Some(Vector4f::parse(rdr, ver)?);
        m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_explicit_probe_set_hash = Some(Hash128::parse(rdr, ver)?);
        } else {
        }
        m_bake_light_probes_for_trees =
            Some(TerrainMBakeLightProbesForTreesVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_preserve_tree_prototype_layers = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }
        m_dynamic_uvst = Some(Vector4f::parse(rdr, ver)?);
        m_chunk_dynamic_uvst = Some(TerrainMChunkDynamicUvstVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
            m_enable_heightmap_ray_tracing = Some(rdr.read_bool()?);
            m_enable_trees_and_details_ray_tracing = Some(
                TerrainMEnableTreesAndDetailsRayTracingVariant::parse(rdr, ver)?,
            );
            m_tree_motion_vector_mode_override = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 9) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
            m_enable_heightmap_ray_tracing = Some(rdr.read_bool()?);
            m_enable_trees_and_details_ray_tracing = Some(
                TerrainMEnableTreesAndDetailsRayTracingVariant::parse(rdr, ver)?,
            );
            m_enable_heightmap_lod_frustum_culling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_tree_motion_vector_mode_override = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
            m_enable_heightmap_ray_tracing = Some(rdr.read_bool()?);
            m_enable_trees_and_details_ray_tracing = Some(
                TerrainMEnableTreesAndDetailsRayTracingVariant::parse(rdr, ver)?,
            );
            m_tree_motion_vector_mode_override = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 3, 17) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
            m_enable_heightmap_ray_tracing = Some(rdr.read_bool()?);
            m_enable_trees_and_details_ray_tracing = Some(
                TerrainMEnableTreesAndDetailsRayTracingVariant::parse(rdr, ver)?,
            );
            m_enable_heightmap_lod_frustum_culling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_tree_motion_vector_mode_override = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
            m_enable_heightmap_ray_tracing = Some(rdr.read_bool()?);
            m_enable_trees_and_details_ray_tracing = Some(
                TerrainMEnableTreesAndDetailsRayTracingVariant::parse(rdr, ver)?,
            );
            m_tree_motion_vector_mode_override = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 76) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
            m_enable_heightmap_ray_tracing = Some(rdr.read_bool()?);
            m_enable_trees_and_details_ray_tracing = Some(
                TerrainMEnableTreesAndDetailsRayTracingVariant::parse(rdr, ver)?,
            );
            m_enable_heightmap_lod_frustum_culling = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_tree_motion_vector_mode_override = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 2) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
            m_enable_heightmap_ray_tracing = Some(rdr.read_bool()?);
            m_enable_trees_and_details_ray_tracing = Some(
                TerrainMEnableTreesAndDetailsRayTracingVariant::parse(rdr, ver)?,
            );
            m_tree_motion_vector_mode_override = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
            m_enable_heightmap_ray_tracing = Some(rdr.read_bool()?);
            m_enable_trees_and_details_ray_tracing = Some(
                TerrainMEnableTreesAndDetailsRayTracingVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2022, 3, 3) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
            m_enable_heightmap_ray_tracing = Some(rdr.read_bool()?);
            m_enable_trees_and_details_ray_tracing = Some(
                TerrainMEnableTreesAndDetailsRayTracingVariant::parse(rdr, ver)?,
            );
            m_tree_motion_vector_mode_override = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
            m_enable_heightmap_ray_tracing = Some(rdr.read_bool()?);
            m_enable_trees_and_details_ray_tracing = Some(
                TerrainMEnableTreesAndDetailsRayTracingVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_grouping_id = Some(rdr.read_i32()?);
            m_allow_auto_connect = Some(rdr.read_bool()?);
        } else {
        }

        Ok(Terrain {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_terrain_data: m_terrain_data.unwrap(),
            m_tree_distance: m_tree_distance.unwrap(),
            m_tree_billboard_distance: m_tree_billboard_distance.unwrap(),
            m_tree_cross_fade_length: m_tree_cross_fade_length.unwrap(),
            m_tree_maximum_full_lod_count: m_tree_maximum_full_lod_count.unwrap(),
            m_detail_object_distance: m_detail_object_distance.unwrap(),
            m_detail_object_density: m_detail_object_density.unwrap(),
            m_heightmap_pixel_error: m_heightmap_pixel_error.unwrap(),
            m_splat_map_distance: m_splat_map_distance.unwrap(),
            m_heightmap_maximum_lod: m_heightmap_maximum_lod.unwrap(),
            m_heightmap_minimum_lod_simplification,
            m_cast_shadows,
            m_shadow_casting_mode,
            m_draw_heightmap: m_draw_heightmap.unwrap(),
            m_draw_trees_and_foliage: m_draw_trees_and_foliage.unwrap(),
            m_draw_instanced,
            m_reflection_probe_usage: m_reflection_probe_usage.unwrap(),
            m_material_type,
            m_static_shadow_caster,
            m_legacy_specular,
            m_material_template: m_material_template.unwrap(),
            m_ignore_quality_settings,
            m_legacy_shininess,
            m_lightmap_index: m_lightmap_index.unwrap(),
            m_use_default_smoothness,
            m_lightmap_index_dynamic: m_lightmap_index_dynamic.unwrap(),
            m_default_smoothness,
            m_lightmap_tiling_offset: m_lightmap_tiling_offset.unwrap(),
            m_lightmap_tiling_offset_dynamic: m_lightmap_tiling_offset_dynamic.unwrap(),
            m_explicit_probe_set_hash,
            m_bake_light_probes_for_trees: m_bake_light_probes_for_trees.unwrap(),
            m_preserve_tree_prototype_layers,
            m_dynamic_uvst: m_dynamic_uvst.unwrap(),
            m_chunk_dynamic_uvst: m_chunk_dynamic_uvst.unwrap(),
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
