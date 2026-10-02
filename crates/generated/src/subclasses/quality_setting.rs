//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::mipmap_limit_settings::MipmapLimitSettings;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：QualitySettingSoftVegetationVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum QualitySettingSoftVegetationVariant {
    Default(bool),
    V5_0(bool),
}

impl crate::Parse for QualitySettingSoftVegetationVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for QualitySettingSoftVegetationVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：QualitySettingVSyncCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum QualitySettingVSyncCountVariant {
    Default(i32),
    V3_5(i32),
}

impl crate::Parse for QualitySettingVSyncCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V3_5(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for QualitySettingVSyncCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V3_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V3_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：QualitySettingMaximumLodLevelVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum QualitySettingMaximumLodLevelVariant {
    Default(i32),
    V4_0(i32),
}

impl crate::Parse for QualitySettingMaximumLodLevelVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for QualitySettingMaximumLodLevelVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：QualitySettingParticleRaycastBudgetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum QualitySettingParticleRaycastBudgetVariant {
    Default(i32),
    V5_3(i32),
}

impl crate::Parse for QualitySettingParticleRaycastBudgetVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_3(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for QualitySettingParticleRaycastBudgetVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：QualitySettingBillboardsFaceCameraPositionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum QualitySettingBillboardsFaceCameraPositionVariant {
    Default(bool),
    V2022_2(bool),
}

impl crate::Parse for QualitySettingBillboardsFaceCameraPositionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2022_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for QualitySettingBillboardsFaceCameraPositionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：QualitySettingUseLegacyDetailDistributionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum QualitySettingUseLegacyDetailDistributionVariant {
    Default(bool),
    V2023_2(bool),
}

impl crate::Parse for QualitySettingUseLegacyDetailDistributionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2023_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for QualitySettingUseLegacyDetailDistributionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：QualitySettingAsyncUploadBufferSizeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum QualitySettingAsyncUploadBufferSizeVariant {
    Default(i32),
    V2017_1(i32),
}

impl crate::Parse for QualitySettingAsyncUploadBufferSizeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2017_1(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for QualitySettingAsyncUploadBufferSizeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：QualitySettingResolutionScalingFixedDpiFactorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum QualitySettingResolutionScalingFixedDpiFactorVariant {
    Default(f32),
    V2019_3(f32),
}

impl crate::Parse for QualitySettingResolutionScalingFixedDpiFactorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = rdr.read_f32()?;
            Ok(Self::V2019_3(v))
        } else {
            let v = rdr.read_f32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for QualitySettingResolutionScalingFixedDpiFactorVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct QualitySetting {
    pub pixel_light_count: i32,
    pub name: Option<String>,
    pub shadows: i32,
    pub shadow_resolution: i32,
    pub shadow_projection: i32,
    pub shadow_cascades: i32,
    pub shadow_distance: f32,
    pub blend_weights: Option<i32>,
    pub texture_quality: Option<i32>,
    pub shadow_cascade2_split: Option<f32>,
    pub shadow_near_plane_offset: Option<f32>,
    pub anisotropic_textures: i32,
    pub shadow_cascade4_split: Option<Vector3f>,
    pub anti_aliasing: i32,
    pub soft_particles: bool,
    pub shadowmask_mode: Option<i32>,
    pub soft_vegetation: QualitySettingSoftVegetationVariant,
    pub skin_weights: Option<i32>,
    pub v_sync_count: QualitySettingVSyncCountVariant,
    pub global_texture_mipmap_limit: Option<i32>,
    pub texture_mipmap_limit_settings: Option<Vec<MipmapLimitSettings>>,
    pub lod_bias: Option<f32>,
    pub maximum_lod_level: Option<QualitySettingMaximumLodLevelVariant>,
    pub realtime_reflection_probes: Option<bool>,
    pub particle_raycast_budget: Option<QualitySettingParticleRaycastBudgetVariant>,
    pub billboards_face_camera_position: Option<QualitySettingBillboardsFaceCameraPositionVariant>,
    pub realtime_gicpu_usage: Option<i32>,
    pub use_legacy_detail_distribution: Option<QualitySettingUseLegacyDetailDistributionVariant>,
    pub adaptive_vsync: Option<bool>,
    pub async_upload_time_slice: Option<i32>,
    pub streaming_mipmaps_active: Option<bool>,
    pub async_upload_buffer_size: Option<QualitySettingAsyncUploadBufferSizeVariant>,
    pub streaming_mipmaps_add_all_cameras: Option<bool>,
    pub streaming_mipmaps_memory_budget: Option<f32>,
    pub enable_lod_cross_fade: Option<bool>,
    pub adaptive_vsync_extra_a: Option<i32>,
    pub resolution_scaling_fixed_dpi_factor:
        Option<QualitySettingResolutionScalingFixedDpiFactorVariant>,
    pub streaming_mipmaps_renderers_per_frame: Option<i32>,
    pub adaptive_vsync_extra_b: Option<i32>,
    pub streaming_mipmaps_max_level_reduction: Option<i32>,
    pub streaming_mipmaps_max_file_io_requests: Option<i32>,
    pub mesh_lod_threshold: Option<f32>,
    pub async_upload_persistent_buffer: Option<bool>,
    pub custom_render_pipeline: Option<PPtr>,
    pub terrain_quality_overrides: Option<i32>,
    pub terrain_pixel_error: Option<f32>,
    pub terrain_detail_density_scale: Option<f32>,
    pub terrain_basemap_distance: Option<f32>,
    pub terrain_detail_distance: Option<f32>,
    pub terrain_tree_distance: Option<f32>,
    pub terrain_billboard_start: Option<f32>,
    pub terrain_fade_length: Option<f32>,
    pub terrain_max_trees: Option<i32>,
}

impl crate::Parse for QualitySetting {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut pixel_light_count = None;
        let mut name = None;
        let mut shadows = None;
        let mut shadow_resolution = None;
        let mut shadow_projection = None;
        let mut shadow_cascades = None;
        let mut shadow_distance = None;
        let mut blend_weights = None;
        let mut texture_quality = None;
        let mut shadow_cascade2_split = None;
        let mut shadow_near_plane_offset = None;
        let mut anisotropic_textures = None;
        let mut shadow_cascade4_split = None;
        let mut anti_aliasing = None;
        let mut soft_particles = None;
        let mut shadowmask_mode = None;
        let mut soft_vegetation = None;
        let mut skin_weights = None;
        let mut v_sync_count = None;
        let mut global_texture_mipmap_limit = None;
        let mut texture_mipmap_limit_settings = None;
        let mut lod_bias = None;
        let mut maximum_lod_level = None;
        let mut realtime_reflection_probes = None;
        let mut particle_raycast_budget = None;
        let mut billboards_face_camera_position = None;
        let mut realtime_gicpu_usage = None;
        let mut use_legacy_detail_distribution = None;
        let mut adaptive_vsync = None;
        let mut async_upload_time_slice = None;
        let mut streaming_mipmaps_active = None;
        let mut async_upload_buffer_size = None;
        let mut streaming_mipmaps_add_all_cameras = None;
        let mut streaming_mipmaps_memory_budget = None;
        let mut enable_lod_cross_fade = None;
        let mut adaptive_vsync_extra_a = None;
        let mut resolution_scaling_fixed_dpi_factor = None;
        let mut streaming_mipmaps_renderers_per_frame = None;
        let mut adaptive_vsync_extra_b = None;
        let mut streaming_mipmaps_max_level_reduction = None;
        let mut streaming_mipmaps_max_file_io_requests = None;
        let mut mesh_lod_threshold = None;
        let mut async_upload_persistent_buffer = None;
        let mut custom_render_pipeline = None;
        let mut terrain_quality_overrides = None;
        let mut terrain_pixel_error = None;
        let mut terrain_detail_density_scale = None;
        let mut terrain_basemap_distance = None;
        let mut terrain_detail_distance = None;
        let mut terrain_tree_distance = None;
        let mut terrain_billboard_start = None;
        let mut terrain_fade_length = None;
        let mut terrain_max_trees = None;
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            name = Some(rdr.read_aligned_string()?);
        } else {
        }
        pixel_light_count = Some(rdr.read_i32()?);
        shadows = Some(rdr.read_i32()?);
        shadow_resolution = Some(rdr.read_i32()?);
        shadow_projection = Some(rdr.read_i32()?);
        shadow_cascades = Some(rdr.read_i32()?);
        shadow_distance = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            shadow_near_plane_offset = Some(rdr.read_f32()?);
            shadow_cascade2_split = Some(rdr.read_f32()?);
            shadow_cascade4_split = Some(Vector3f::parse(rdr, ver)?);
            shadowmask_mode = Some(rdr.read_i32()?);
            skin_weights = Some(rdr.read_i32()?);
            global_texture_mipmap_limit = Some(rdr.read_i32()?);
            texture_mipmap_limit_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| MipmapLimitSettings::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            shadow_near_plane_offset = Some(rdr.read_f32()?);
            shadow_cascade2_split = Some(rdr.read_f32()?);
            shadow_cascade4_split = Some(Vector3f::parse(rdr, ver)?);
            shadowmask_mode = Some(rdr.read_i32()?);
            skin_weights = Some(rdr.read_i32()?);
            texture_quality = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            shadow_near_plane_offset = Some(rdr.read_f32()?);
            shadow_cascade2_split = Some(rdr.read_f32()?);
            shadow_cascade4_split = Some(Vector3f::parse(rdr, ver)?);
            shadowmask_mode = Some(rdr.read_i32()?);
            blend_weights = Some(rdr.read_i32()?);
            texture_quality = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            shadow_near_plane_offset = Some(rdr.read_f32()?);
            shadow_cascade2_split = Some(rdr.read_f32()?);
            shadow_cascade4_split = Some(Vector3f::parse(rdr, ver)?);
            blend_weights = Some(rdr.read_i32()?);
            texture_quality = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            shadow_cascade2_split = Some(rdr.read_f32()?);
            shadow_cascade4_split = Some(Vector3f::parse(rdr, ver)?);
            blend_weights = Some(rdr.read_i32()?);
            texture_quality = Some(rdr.read_i32()?);
        } else {
            blend_weights = Some(rdr.read_i32()?);
            texture_quality = Some(rdr.read_i32()?);
        }
        anisotropic_textures = Some(rdr.read_i32()?);
        anti_aliasing = Some(rdr.read_i32()?);
        soft_particles = Some(rdr.read_bool()?);
        soft_vegetation = Some(QualitySettingSoftVegetationVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            realtime_reflection_probes = Some(rdr.read_bool()?);
            billboards_face_camera_position = Some(
                QualitySettingBillboardsFaceCameraPositionVariant::parse(rdr, ver)?,
            );
            use_legacy_detail_distribution = Some(
                QualitySettingUseLegacyDetailDistributionVariant::parse(rdr, ver)?,
            );
            adaptive_vsync = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            realtime_reflection_probes = Some(rdr.read_bool()?);
            billboards_face_camera_position = Some(
                QualitySettingBillboardsFaceCameraPositionVariant::parse(rdr, ver)?,
            );
            use_legacy_detail_distribution = Some(
                QualitySettingUseLegacyDetailDistributionVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            realtime_reflection_probes = Some(rdr.read_bool()?);
            billboards_face_camera_position = Some(
                QualitySettingBillboardsFaceCameraPositionVariant::parse(rdr, ver)?,
            );
        } else {
        }
        v_sync_count = Some(QualitySettingVSyncCountVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            realtime_gicpu_usage = Some(rdr.read_i32()?);
            adaptive_vsync_extra_a = Some(rdr.read_i32()?);
            adaptive_vsync_extra_b = Some(rdr.read_i32()?);
            lod_bias = Some(rdr.read_f32()?);
            mesh_lod_threshold = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            enable_lod_cross_fade = Some(rdr.read_bool()?);
            streaming_mipmaps_active = Some(rdr.read_bool()?);
            streaming_mipmaps_add_all_cameras = Some(rdr.read_bool()?);
            rdr.align(4)?;
            streaming_mipmaps_memory_budget = Some(rdr.read_f32()?);
            streaming_mipmaps_renderers_per_frame = Some(rdr.read_i32()?);
            streaming_mipmaps_max_level_reduction = Some(rdr.read_i32()?);
            streaming_mipmaps_max_file_io_requests = Some(rdr.read_i32()?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
            async_upload_time_slice = Some(rdr.read_i32()?);
            async_upload_buffer_size =
                Some(QualitySettingAsyncUploadBufferSizeVariant::parse(rdr, ver)?);
            async_upload_persistent_buffer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            resolution_scaling_fixed_dpi_factor = Some(
                QualitySettingResolutionScalingFixedDpiFactorVariant::parse(rdr, ver)?,
            );
            custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            terrain_quality_overrides = Some(rdr.read_i32()?);
            terrain_pixel_error = Some(rdr.read_f32()?);
            terrain_detail_density_scale = Some(rdr.read_f32()?);
            terrain_basemap_distance = Some(rdr.read_f32()?);
            terrain_detail_distance = Some(rdr.read_f32()?);
            terrain_tree_distance = Some(rdr.read_f32()?);
            terrain_billboard_start = Some(rdr.read_f32()?);
            terrain_fade_length = Some(rdr.read_f32()?);
            terrain_max_trees = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            realtime_gicpu_usage = Some(rdr.read_i32()?);
            adaptive_vsync_extra_a = Some(rdr.read_i32()?);
            adaptive_vsync_extra_b = Some(rdr.read_i32()?);
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            enable_lod_cross_fade = Some(rdr.read_bool()?);
            streaming_mipmaps_active = Some(rdr.read_bool()?);
            streaming_mipmaps_add_all_cameras = Some(rdr.read_bool()?);
            rdr.align(4)?;
            streaming_mipmaps_memory_budget = Some(rdr.read_f32()?);
            streaming_mipmaps_renderers_per_frame = Some(rdr.read_i32()?);
            streaming_mipmaps_max_level_reduction = Some(rdr.read_i32()?);
            streaming_mipmaps_max_file_io_requests = Some(rdr.read_i32()?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
            async_upload_time_slice = Some(rdr.read_i32()?);
            async_upload_buffer_size =
                Some(QualitySettingAsyncUploadBufferSizeVariant::parse(rdr, ver)?);
            async_upload_persistent_buffer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            resolution_scaling_fixed_dpi_factor = Some(
                QualitySettingResolutionScalingFixedDpiFactorVariant::parse(rdr, ver)?,
            );
            custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            terrain_quality_overrides = Some(rdr.read_i32()?);
            terrain_pixel_error = Some(rdr.read_f32()?);
            terrain_detail_density_scale = Some(rdr.read_f32()?);
            terrain_basemap_distance = Some(rdr.read_f32()?);
            terrain_detail_distance = Some(rdr.read_f32()?);
            terrain_tree_distance = Some(rdr.read_f32()?);
            terrain_billboard_start = Some(rdr.read_f32()?);
            terrain_fade_length = Some(rdr.read_f32()?);
            terrain_max_trees = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 3, 5) {
            realtime_gicpu_usage = Some(rdr.read_i32()?);
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            enable_lod_cross_fade = Some(rdr.read_bool()?);
            streaming_mipmaps_active = Some(rdr.read_bool()?);
            streaming_mipmaps_add_all_cameras = Some(rdr.read_bool()?);
            rdr.align(4)?;
            streaming_mipmaps_memory_budget = Some(rdr.read_f32()?);
            streaming_mipmaps_renderers_per_frame = Some(rdr.read_i32()?);
            streaming_mipmaps_max_level_reduction = Some(rdr.read_i32()?);
            streaming_mipmaps_max_file_io_requests = Some(rdr.read_i32()?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
            async_upload_time_slice = Some(rdr.read_i32()?);
            async_upload_buffer_size =
                Some(QualitySettingAsyncUploadBufferSizeVariant::parse(rdr, ver)?);
            async_upload_persistent_buffer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            resolution_scaling_fixed_dpi_factor = Some(
                QualitySettingResolutionScalingFixedDpiFactorVariant::parse(rdr, ver)?,
            );
            custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            terrain_quality_overrides = Some(rdr.read_i32()?);
            terrain_pixel_error = Some(rdr.read_f32()?);
            terrain_detail_density_scale = Some(rdr.read_f32()?);
            terrain_basemap_distance = Some(rdr.read_f32()?);
            terrain_detail_distance = Some(rdr.read_f32()?);
            terrain_tree_distance = Some(rdr.read_f32()?);
            terrain_billboard_start = Some(rdr.read_f32()?);
            terrain_fade_length = Some(rdr.read_f32()?);
            terrain_max_trees = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            enable_lod_cross_fade = Some(rdr.read_bool()?);
            streaming_mipmaps_active = Some(rdr.read_bool()?);
            streaming_mipmaps_add_all_cameras = Some(rdr.read_bool()?);
            rdr.align(4)?;
            streaming_mipmaps_memory_budget = Some(rdr.read_f32()?);
            streaming_mipmaps_renderers_per_frame = Some(rdr.read_i32()?);
            streaming_mipmaps_max_level_reduction = Some(rdr.read_i32()?);
            streaming_mipmaps_max_file_io_requests = Some(rdr.read_i32()?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
            async_upload_time_slice = Some(rdr.read_i32()?);
            async_upload_buffer_size =
                Some(QualitySettingAsyncUploadBufferSizeVariant::parse(rdr, ver)?);
            async_upload_persistent_buffer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            resolution_scaling_fixed_dpi_factor = Some(
                QualitySettingResolutionScalingFixedDpiFactorVariant::parse(rdr, ver)?,
            );
            custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            terrain_quality_overrides = Some(rdr.read_i32()?);
            terrain_pixel_error = Some(rdr.read_f32()?);
            terrain_detail_density_scale = Some(rdr.read_f32()?);
            terrain_basemap_distance = Some(rdr.read_f32()?);
            terrain_detail_distance = Some(rdr.read_f32()?);
            terrain_tree_distance = Some(rdr.read_f32()?);
            terrain_billboard_start = Some(rdr.read_f32()?);
            terrain_fade_length = Some(rdr.read_f32()?);
            terrain_max_trees = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            streaming_mipmaps_active = Some(rdr.read_bool()?);
            streaming_mipmaps_add_all_cameras = Some(rdr.read_bool()?);
            rdr.align(4)?;
            streaming_mipmaps_memory_budget = Some(rdr.read_f32()?);
            streaming_mipmaps_renderers_per_frame = Some(rdr.read_i32()?);
            streaming_mipmaps_max_level_reduction = Some(rdr.read_i32()?);
            streaming_mipmaps_max_file_io_requests = Some(rdr.read_i32()?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
            async_upload_time_slice = Some(rdr.read_i32()?);
            async_upload_buffer_size =
                Some(QualitySettingAsyncUploadBufferSizeVariant::parse(rdr, ver)?);
            async_upload_persistent_buffer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            resolution_scaling_fixed_dpi_factor = Some(
                QualitySettingResolutionScalingFixedDpiFactorVariant::parse(rdr, ver)?,
            );
            custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 29) {
            realtime_gicpu_usage = Some(rdr.read_i32()?);
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            streaming_mipmaps_active = Some(rdr.read_bool()?);
            streaming_mipmaps_add_all_cameras = Some(rdr.read_bool()?);
            rdr.align(4)?;
            streaming_mipmaps_memory_budget = Some(rdr.read_f32()?);
            streaming_mipmaps_renderers_per_frame = Some(rdr.read_i32()?);
            streaming_mipmaps_max_level_reduction = Some(rdr.read_i32()?);
            streaming_mipmaps_max_file_io_requests = Some(rdr.read_i32()?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
            async_upload_time_slice = Some(rdr.read_i32()?);
            async_upload_buffer_size =
                Some(QualitySettingAsyncUploadBufferSizeVariant::parse(rdr, ver)?);
            async_upload_persistent_buffer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            resolution_scaling_fixed_dpi_factor = Some(
                QualitySettingResolutionScalingFixedDpiFactorVariant::parse(rdr, ver)?,
            );
            custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            streaming_mipmaps_active = Some(rdr.read_bool()?);
            streaming_mipmaps_add_all_cameras = Some(rdr.read_bool()?);
            rdr.align(4)?;
            streaming_mipmaps_memory_budget = Some(rdr.read_f32()?);
            streaming_mipmaps_renderers_per_frame = Some(rdr.read_i32()?);
            streaming_mipmaps_max_level_reduction = Some(rdr.read_i32()?);
            streaming_mipmaps_max_file_io_requests = Some(rdr.read_i32()?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
            async_upload_time_slice = Some(rdr.read_i32()?);
            async_upload_buffer_size =
                Some(QualitySettingAsyncUploadBufferSizeVariant::parse(rdr, ver)?);
            async_upload_persistent_buffer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            resolution_scaling_fixed_dpi_factor = Some(
                QualitySettingResolutionScalingFixedDpiFactorVariant::parse(rdr, ver)?,
            );
            custom_render_pipeline = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            streaming_mipmaps_active = Some(rdr.read_bool()?);
            streaming_mipmaps_add_all_cameras = Some(rdr.read_bool()?);
            rdr.align(4)?;
            streaming_mipmaps_memory_budget = Some(rdr.read_f32()?);
            streaming_mipmaps_renderers_per_frame = Some(rdr.read_i32()?);
            streaming_mipmaps_max_level_reduction = Some(rdr.read_i32()?);
            streaming_mipmaps_max_file_io_requests = Some(rdr.read_i32()?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
            async_upload_time_slice = Some(rdr.read_i32()?);
            async_upload_buffer_size =
                Some(QualitySettingAsyncUploadBufferSizeVariant::parse(rdr, ver)?);
            async_upload_persistent_buffer = Some(rdr.read_bool()?);
            rdr.align(4)?;
            resolution_scaling_fixed_dpi_factor = Some(
                QualitySettingResolutionScalingFixedDpiFactorVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            streaming_mipmaps_active = Some(rdr.read_bool()?);
            streaming_mipmaps_add_all_cameras = Some(rdr.read_bool()?);
            rdr.align(4)?;
            streaming_mipmaps_memory_budget = Some(rdr.read_f32()?);
            streaming_mipmaps_renderers_per_frame = Some(rdr.read_i32()?);
            streaming_mipmaps_max_level_reduction = Some(rdr.read_i32()?);
            streaming_mipmaps_max_file_io_requests = Some(rdr.read_i32()?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
            async_upload_time_slice = Some(rdr.read_i32()?);
            async_upload_buffer_size =
                Some(QualitySettingAsyncUploadBufferSizeVariant::parse(rdr, ver)?);
            resolution_scaling_fixed_dpi_factor = Some(
                QualitySettingResolutionScalingFixedDpiFactorVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
            async_upload_time_slice = Some(rdr.read_i32()?);
            async_upload_buffer_size =
                Some(QualitySettingAsyncUploadBufferSizeVariant::parse(rdr, ver)?);
            resolution_scaling_fixed_dpi_factor = Some(
                QualitySettingResolutionScalingFixedDpiFactorVariant::parse(rdr, ver)?,
            );
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
            async_upload_time_slice = Some(rdr.read_i32()?);
            async_upload_buffer_size =
                Some(QualitySettingAsyncUploadBufferSizeVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
            particle_raycast_budget =
                Some(QualitySettingParticleRaycastBudgetVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            lod_bias = Some(rdr.read_f32()?);
            maximum_lod_level = Some(QualitySettingMaximumLodLevelVariant::parse(rdr, ver)?);
        } else {
        }

        Ok(QualitySetting {
            pixel_light_count: pixel_light_count.unwrap(),
            name,
            shadows: shadows.unwrap(),
            shadow_resolution: shadow_resolution.unwrap(),
            shadow_projection: shadow_projection.unwrap(),
            shadow_cascades: shadow_cascades.unwrap(),
            shadow_distance: shadow_distance.unwrap(),
            blend_weights,
            texture_quality,
            shadow_cascade2_split,
            shadow_near_plane_offset,
            anisotropic_textures: anisotropic_textures.unwrap(),
            shadow_cascade4_split,
            anti_aliasing: anti_aliasing.unwrap(),
            soft_particles: soft_particles.unwrap(),
            shadowmask_mode,
            soft_vegetation: soft_vegetation.unwrap(),
            skin_weights,
            v_sync_count: v_sync_count.unwrap(),
            global_texture_mipmap_limit,
            texture_mipmap_limit_settings,
            lod_bias,
            maximum_lod_level,
            realtime_reflection_probes,
            particle_raycast_budget,
            billboards_face_camera_position,
            realtime_gicpu_usage,
            use_legacy_detail_distribution,
            adaptive_vsync,
            async_upload_time_slice,
            streaming_mipmaps_active,
            async_upload_buffer_size,
            streaming_mipmaps_add_all_cameras,
            streaming_mipmaps_memory_budget,
            enable_lod_cross_fade,
            adaptive_vsync_extra_a,
            resolution_scaling_fixed_dpi_factor,
            streaming_mipmaps_renderers_per_frame,
            adaptive_vsync_extra_b,
            streaming_mipmaps_max_level_reduction,
            streaming_mipmaps_max_file_io_requests,
            mesh_lod_threshold,
            async_upload_persistent_buffer,
            custom_render_pipeline,
            terrain_quality_overrides,
            terrain_pixel_error,
            terrain_detail_density_scale,
            terrain_basemap_distance,
            terrain_detail_distance,
            terrain_tree_distance,
            terrain_billboard_start,
            terrain_fade_length,
            terrain_max_trees,
        })
    }
}

impl crate::ToValue for QualitySetting {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "pixelLightCount".to_string(),
                self.pixel_light_count.to_value(),
            )),
            crate::opt_field("name", &self.name),
            Some(("shadows".to_string(), self.shadows.to_value())),
            Some((
                "shadowResolution".to_string(),
                self.shadow_resolution.to_value(),
            )),
            Some((
                "shadowProjection".to_string(),
                self.shadow_projection.to_value(),
            )),
            Some((
                "shadowCascades".to_string(),
                self.shadow_cascades.to_value(),
            )),
            Some((
                "shadowDistance".to_string(),
                self.shadow_distance.to_value(),
            )),
            crate::opt_field("blendWeights", &self.blend_weights),
            crate::opt_field("textureQuality", &self.texture_quality),
            crate::opt_field("shadowCascade2Split", &self.shadow_cascade2_split),
            crate::opt_field("shadowNearPlaneOffset", &self.shadow_near_plane_offset),
            Some((
                "anisotropicTextures".to_string(),
                self.anisotropic_textures.to_value(),
            )),
            crate::opt_field("shadowCascade4Split", &self.shadow_cascade4_split),
            Some(("antiAliasing".to_string(), self.anti_aliasing.to_value())),
            Some(("softParticles".to_string(), self.soft_particles.to_value())),
            crate::opt_field("shadowmaskMode", &self.shadowmask_mode),
            Some((
                "softVegetation".to_string(),
                self.soft_vegetation.to_value(),
            )),
            crate::opt_field("skinWeights", &self.skin_weights),
            Some(("vSyncCount".to_string(), self.v_sync_count.to_value())),
            crate::opt_field(
                "globalTextureMipmapLimit",
                &self.global_texture_mipmap_limit,
            ),
            crate::opt_field(
                "textureMipmapLimitSettings",
                &self.texture_mipmap_limit_settings,
            ),
            crate::opt_field("lodBias", &self.lod_bias),
            crate::opt_field("maximumLODLevel", &self.maximum_lod_level),
            crate::opt_field("realtimeReflectionProbes", &self.realtime_reflection_probes),
            crate::opt_field("particleRaycastBudget", &self.particle_raycast_budget),
            crate::opt_field(
                "billboardsFaceCameraPosition",
                &self.billboards_face_camera_position,
            ),
            crate::opt_field("realtimeGICPUUsage", &self.realtime_gicpu_usage),
            crate::opt_field(
                "useLegacyDetailDistribution",
                &self.use_legacy_detail_distribution,
            ),
            crate::opt_field("adaptiveVsync", &self.adaptive_vsync),
            crate::opt_field("asyncUploadTimeSlice", &self.async_upload_time_slice),
            crate::opt_field("streamingMipmapsActive", &self.streaming_mipmaps_active),
            crate::opt_field("asyncUploadBufferSize", &self.async_upload_buffer_size),
            crate::opt_field(
                "streamingMipmapsAddAllCameras",
                &self.streaming_mipmaps_add_all_cameras,
            ),
            crate::opt_field(
                "streamingMipmapsMemoryBudget",
                &self.streaming_mipmaps_memory_budget,
            ),
            crate::opt_field("enableLODCrossFade", &self.enable_lod_cross_fade),
            crate::opt_field("adaptiveVsyncExtraA", &self.adaptive_vsync_extra_a),
            crate::opt_field(
                "resolutionScalingFixedDPIFactor",
                &self.resolution_scaling_fixed_dpi_factor,
            ),
            crate::opt_field(
                "streamingMipmapsRenderersPerFrame",
                &self.streaming_mipmaps_renderers_per_frame,
            ),
            crate::opt_field("adaptiveVsyncExtraB", &self.adaptive_vsync_extra_b),
            crate::opt_field(
                "streamingMipmapsMaxLevelReduction",
                &self.streaming_mipmaps_max_level_reduction,
            ),
            crate::opt_field(
                "streamingMipmapsMaxFileIORequests",
                &self.streaming_mipmaps_max_file_io_requests,
            ),
            crate::opt_field("meshLodThreshold", &self.mesh_lod_threshold),
            crate::opt_field(
                "asyncUploadPersistentBuffer",
                &self.async_upload_persistent_buffer,
            ),
            crate::opt_field("customRenderPipeline", &self.custom_render_pipeline),
            crate::opt_field("terrainQualityOverrides", &self.terrain_quality_overrides),
            crate::opt_field("terrainPixelError", &self.terrain_pixel_error),
            crate::opt_field(
                "terrainDetailDensityScale",
                &self.terrain_detail_density_scale,
            ),
            crate::opt_field("terrainBasemapDistance", &self.terrain_basemap_distance),
            crate::opt_field("terrainDetailDistance", &self.terrain_detail_distance),
            crate::opt_field("terrainTreeDistance", &self.terrain_tree_distance),
            crate::opt_field("terrainBillboardStart", &self.terrain_billboard_start),
            crate::opt_field("terrainFadeLength", &self.terrain_fade_length),
            crate::opt_field("terrainMaxTrees", &self.terrain_max_trees),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "pixelLightCount" => (self.pixel_light_count).inspect_value(path, offset, limit),
                "name" => (self.name).inspect_value(path, offset, limit),
                "shadows" => (self.shadows).inspect_value(path, offset, limit),
                "shadowResolution" => (self.shadow_resolution).inspect_value(path, offset, limit),
                "shadowProjection" => (self.shadow_projection).inspect_value(path, offset, limit),
                "shadowCascades" => (self.shadow_cascades).inspect_value(path, offset, limit),
                "shadowDistance" => (self.shadow_distance).inspect_value(path, offset, limit),
                "blendWeights" => (self.blend_weights).inspect_value(path, offset, limit),
                "textureQuality" => (self.texture_quality).inspect_value(path, offset, limit),
                "shadowCascade2Split" => {
                    (self.shadow_cascade2_split).inspect_value(path, offset, limit)
                }
                "shadowNearPlaneOffset" => {
                    (self.shadow_near_plane_offset).inspect_value(path, offset, limit)
                }
                "anisotropicTextures" => {
                    (self.anisotropic_textures).inspect_value(path, offset, limit)
                }
                "shadowCascade4Split" => {
                    (self.shadow_cascade4_split).inspect_value(path, offset, limit)
                }
                "antiAliasing" => (self.anti_aliasing).inspect_value(path, offset, limit),
                "softParticles" => (self.soft_particles).inspect_value(path, offset, limit),
                "shadowmaskMode" => (self.shadowmask_mode).inspect_value(path, offset, limit),
                "softVegetation" => (self.soft_vegetation).inspect_value(path, offset, limit),
                "skinWeights" => (self.skin_weights).inspect_value(path, offset, limit),
                "vSyncCount" => (self.v_sync_count).inspect_value(path, offset, limit),
                "globalTextureMipmapLimit" => {
                    (self.global_texture_mipmap_limit).inspect_value(path, offset, limit)
                }
                "textureMipmapLimitSettings" => {
                    (self.texture_mipmap_limit_settings).inspect_value(path, offset, limit)
                }
                "lodBias" => (self.lod_bias).inspect_value(path, offset, limit),
                "maximumLODLevel" => (self.maximum_lod_level).inspect_value(path, offset, limit),
                "realtimeReflectionProbes" => {
                    (self.realtime_reflection_probes).inspect_value(path, offset, limit)
                }
                "particleRaycastBudget" => {
                    (self.particle_raycast_budget).inspect_value(path, offset, limit)
                }
                "billboardsFaceCameraPosition" => {
                    (self.billboards_face_camera_position).inspect_value(path, offset, limit)
                }
                "realtimeGICPUUsage" => {
                    (self.realtime_gicpu_usage).inspect_value(path, offset, limit)
                }
                "useLegacyDetailDistribution" => {
                    (self.use_legacy_detail_distribution).inspect_value(path, offset, limit)
                }
                "adaptiveVsync" => (self.adaptive_vsync).inspect_value(path, offset, limit),
                "asyncUploadTimeSlice" => {
                    (self.async_upload_time_slice).inspect_value(path, offset, limit)
                }
                "streamingMipmapsActive" => {
                    (self.streaming_mipmaps_active).inspect_value(path, offset, limit)
                }
                "asyncUploadBufferSize" => {
                    (self.async_upload_buffer_size).inspect_value(path, offset, limit)
                }
                "streamingMipmapsAddAllCameras" => {
                    (self.streaming_mipmaps_add_all_cameras).inspect_value(path, offset, limit)
                }
                "streamingMipmapsMemoryBudget" => {
                    (self.streaming_mipmaps_memory_budget).inspect_value(path, offset, limit)
                }
                "enableLODCrossFade" => {
                    (self.enable_lod_cross_fade).inspect_value(path, offset, limit)
                }
                "adaptiveVsyncExtraA" => {
                    (self.adaptive_vsync_extra_a).inspect_value(path, offset, limit)
                }
                "resolutionScalingFixedDPIFactor" => {
                    (self.resolution_scaling_fixed_dpi_factor).inspect_value(path, offset, limit)
                }
                "streamingMipmapsRenderersPerFrame" => {
                    (self.streaming_mipmaps_renderers_per_frame).inspect_value(path, offset, limit)
                }
                "adaptiveVsyncExtraB" => {
                    (self.adaptive_vsync_extra_b).inspect_value(path, offset, limit)
                }
                "streamingMipmapsMaxLevelReduction" => {
                    (self.streaming_mipmaps_max_level_reduction).inspect_value(path, offset, limit)
                }
                "streamingMipmapsMaxFileIORequests" => {
                    (self.streaming_mipmaps_max_file_io_requests).inspect_value(path, offset, limit)
                }
                "meshLodThreshold" => (self.mesh_lod_threshold).inspect_value(path, offset, limit),
                "asyncUploadPersistentBuffer" => {
                    (self.async_upload_persistent_buffer).inspect_value(path, offset, limit)
                }
                "customRenderPipeline" => {
                    (self.custom_render_pipeline).inspect_value(path, offset, limit)
                }
                "terrainQualityOverrides" => {
                    (self.terrain_quality_overrides).inspect_value(path, offset, limit)
                }
                "terrainPixelError" => {
                    (self.terrain_pixel_error).inspect_value(path, offset, limit)
                }
                "terrainDetailDensityScale" => {
                    (self.terrain_detail_density_scale).inspect_value(path, offset, limit)
                }
                "terrainBasemapDistance" => {
                    (self.terrain_basemap_distance).inspect_value(path, offset, limit)
                }
                "terrainDetailDistance" => {
                    (self.terrain_detail_distance).inspect_value(path, offset, limit)
                }
                "terrainTreeDistance" => {
                    (self.terrain_tree_distance).inspect_value(path, offset, limit)
                }
                "terrainBillboardStart" => {
                    (self.terrain_billboard_start).inspect_value(path, offset, limit)
                }
                "terrainFadeLength" => {
                    (self.terrain_fade_length).inspect_value(path, offset, limit)
                }
                "terrainMaxTrees" => (self.terrain_max_trees).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.pixel_light_count).inspect_value(path, offset, limit))
                .map(|v| ("pixelLightCount".to_string(), v)),
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.shadows).inspect_value(path, offset, limit)).map(|v| ("shadows".to_string(), v)),
            ((self.shadow_resolution).inspect_value(path, offset, limit))
                .map(|v| ("shadowResolution".to_string(), v)),
            ((self.shadow_projection).inspect_value(path, offset, limit))
                .map(|v| ("shadowProjection".to_string(), v)),
            ((self.shadow_cascades).inspect_value(path, offset, limit))
                .map(|v| ("shadowCascades".to_string(), v)),
            ((self.shadow_distance).inspect_value(path, offset, limit))
                .map(|v| ("shadowDistance".to_string(), v)),
            ((self.blend_weights).inspect_value(path, offset, limit))
                .map(|v| ("blendWeights".to_string(), v)),
            ((self.texture_quality).inspect_value(path, offset, limit))
                .map(|v| ("textureQuality".to_string(), v)),
            ((self.shadow_cascade2_split).inspect_value(path, offset, limit))
                .map(|v| ("shadowCascade2Split".to_string(), v)),
            ((self.shadow_near_plane_offset).inspect_value(path, offset, limit))
                .map(|v| ("shadowNearPlaneOffset".to_string(), v)),
            ((self.anisotropic_textures).inspect_value(path, offset, limit))
                .map(|v| ("anisotropicTextures".to_string(), v)),
            ((self.shadow_cascade4_split).inspect_value(path, offset, limit))
                .map(|v| ("shadowCascade4Split".to_string(), v)),
            ((self.anti_aliasing).inspect_value(path, offset, limit))
                .map(|v| ("antiAliasing".to_string(), v)),
            ((self.soft_particles).inspect_value(path, offset, limit))
                .map(|v| ("softParticles".to_string(), v)),
            ((self.shadowmask_mode).inspect_value(path, offset, limit))
                .map(|v| ("shadowmaskMode".to_string(), v)),
            ((self.soft_vegetation).inspect_value(path, offset, limit))
                .map(|v| ("softVegetation".to_string(), v)),
            ((self.skin_weights).inspect_value(path, offset, limit))
                .map(|v| ("skinWeights".to_string(), v)),
            ((self.v_sync_count).inspect_value(path, offset, limit))
                .map(|v| ("vSyncCount".to_string(), v)),
            ((self.global_texture_mipmap_limit).inspect_value(path, offset, limit))
                .map(|v| ("globalTextureMipmapLimit".to_string(), v)),
            ((self.texture_mipmap_limit_settings).inspect_value(path, offset, limit))
                .map(|v| ("textureMipmapLimitSettings".to_string(), v)),
            ((self.lod_bias).inspect_value(path, offset, limit))
                .map(|v| ("lodBias".to_string(), v)),
            ((self.maximum_lod_level).inspect_value(path, offset, limit))
                .map(|v| ("maximumLODLevel".to_string(), v)),
            ((self.realtime_reflection_probes).inspect_value(path, offset, limit))
                .map(|v| ("realtimeReflectionProbes".to_string(), v)),
            ((self.particle_raycast_budget).inspect_value(path, offset, limit))
                .map(|v| ("particleRaycastBudget".to_string(), v)),
            ((self.billboards_face_camera_position).inspect_value(path, offset, limit))
                .map(|v| ("billboardsFaceCameraPosition".to_string(), v)),
            ((self.realtime_gicpu_usage).inspect_value(path, offset, limit))
                .map(|v| ("realtimeGICPUUsage".to_string(), v)),
            ((self.use_legacy_detail_distribution).inspect_value(path, offset, limit))
                .map(|v| ("useLegacyDetailDistribution".to_string(), v)),
            ((self.adaptive_vsync).inspect_value(path, offset, limit))
                .map(|v| ("adaptiveVsync".to_string(), v)),
            ((self.async_upload_time_slice).inspect_value(path, offset, limit))
                .map(|v| ("asyncUploadTimeSlice".to_string(), v)),
            ((self.streaming_mipmaps_active).inspect_value(path, offset, limit))
                .map(|v| ("streamingMipmapsActive".to_string(), v)),
            ((self.async_upload_buffer_size).inspect_value(path, offset, limit))
                .map(|v| ("asyncUploadBufferSize".to_string(), v)),
            ((self.streaming_mipmaps_add_all_cameras).inspect_value(path, offset, limit))
                .map(|v| ("streamingMipmapsAddAllCameras".to_string(), v)),
            ((self.streaming_mipmaps_memory_budget).inspect_value(path, offset, limit))
                .map(|v| ("streamingMipmapsMemoryBudget".to_string(), v)),
            ((self.enable_lod_cross_fade).inspect_value(path, offset, limit))
                .map(|v| ("enableLODCrossFade".to_string(), v)),
            ((self.adaptive_vsync_extra_a).inspect_value(path, offset, limit))
                .map(|v| ("adaptiveVsyncExtraA".to_string(), v)),
            ((self.resolution_scaling_fixed_dpi_factor).inspect_value(path, offset, limit))
                .map(|v| ("resolutionScalingFixedDPIFactor".to_string(), v)),
            ((self.streaming_mipmaps_renderers_per_frame).inspect_value(path, offset, limit))
                .map(|v| ("streamingMipmapsRenderersPerFrame".to_string(), v)),
            ((self.adaptive_vsync_extra_b).inspect_value(path, offset, limit))
                .map(|v| ("adaptiveVsyncExtraB".to_string(), v)),
            ((self.streaming_mipmaps_max_level_reduction).inspect_value(path, offset, limit))
                .map(|v| ("streamingMipmapsMaxLevelReduction".to_string(), v)),
            ((self.streaming_mipmaps_max_file_io_requests).inspect_value(path, offset, limit))
                .map(|v| ("streamingMipmapsMaxFileIORequests".to_string(), v)),
            ((self.mesh_lod_threshold).inspect_value(path, offset, limit))
                .map(|v| ("meshLodThreshold".to_string(), v)),
            ((self.async_upload_persistent_buffer).inspect_value(path, offset, limit))
                .map(|v| ("asyncUploadPersistentBuffer".to_string(), v)),
            ((self.custom_render_pipeline).inspect_value(path, offset, limit))
                .map(|v| ("customRenderPipeline".to_string(), v)),
            ((self.terrain_quality_overrides).inspect_value(path, offset, limit))
                .map(|v| ("terrainQualityOverrides".to_string(), v)),
            ((self.terrain_pixel_error).inspect_value(path, offset, limit))
                .map(|v| ("terrainPixelError".to_string(), v)),
            ((self.terrain_detail_density_scale).inspect_value(path, offset, limit))
                .map(|v| ("terrainDetailDensityScale".to_string(), v)),
            ((self.terrain_basemap_distance).inspect_value(path, offset, limit))
                .map(|v| ("terrainBasemapDistance".to_string(), v)),
            ((self.terrain_detail_distance).inspect_value(path, offset, limit))
                .map(|v| ("terrainDetailDistance".to_string(), v)),
            ((self.terrain_tree_distance).inspect_value(path, offset, limit))
                .map(|v| ("terrainTreeDistance".to_string(), v)),
            ((self.terrain_billboard_start).inspect_value(path, offset, limit))
                .map(|v| ("terrainBillboardStart".to_string(), v)),
            ((self.terrain_fade_length).inspect_value(path, offset, limit))
                .map(|v| ("terrainFadeLength".to_string(), v)),
            ((self.terrain_max_trees).inspect_value(path, offset, limit))
                .map(|v| ("terrainMaxTrees".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
