//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::static_batch_info::StaticBatchInfo;
use crate::subclasses::uv_animation::UvAnimation;
use crate::subclasses::vector4f::Vector4f;

/// 版本枚举：ParticleRendererMEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleRendererMEnabledVariant {
    Default(bool),
    V5_0(bool),
    V5_4(bool),
}

impl crate::Parse for ParticleRendererMEnabledVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleRendererMCastShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleRendererMCastShadowsVariant {
    Default(bool),
    V5_0(u8),
}

impl crate::Parse for ParticleRendererMCastShadowsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleRendererMReceiveShadowsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleRendererMReceiveShadowsVariant {
    Default(bool),
    V5_0(bool),
    V5_4(u8),
}

impl crate::Parse for ParticleRendererMReceiveShadowsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V5_4(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleRendererMLightmapIndexVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleRendererMLightmapIndexVariant {
    Default(u8),
    V5_0(u16),
}

impl crate::Parse for ParticleRendererMLightmapIndexVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_u16()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_u8()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleRendererMMaterialsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleRendererMMaterialsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for ParticleRendererMMaterialsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleRendererMReflectionProbeUsageVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleRendererMReflectionProbeUsageVariant {
    Default(i32),
    V5_4(u8),
}

impl crate::Parse for ParticleRendererMReflectionProbeUsageVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::V5_4(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleRendererMLightProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleRendererMLightProbeAnchorVariant {
    Default(PPtr),
    V4_3(PPtr),
}

impl crate::Parse for ParticleRendererMLightProbeAnchorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::V4_3(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleRendererMSortingLayerIdVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleRendererMSortingLayerIdVariant {
    Default(u32),
    V5_0(i32),
}

impl crate::Parse for ParticleRendererMSortingLayerIdVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleRendererMSortingOrderVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleRendererMSortingOrderVariant {
    Default(i16),
    V4_5(i16),
}

impl crate::Parse for ParticleRendererMSortingOrderVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = rdr.read_i16()?;
            rdr.align(4)?;
            Ok(Self::V4_5(v))
        } else {
            let v = rdr.read_i16()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ParticleRendererMProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ParticleRendererMProbeAnchorVariant {
    Default(PPtr),
    V5_4(PPtr),
}

impl crate::Parse for ParticleRendererMProbeAnchorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V5_4(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ParticleRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: ParticleRendererMEnabledVariant,
    pub m_cast_shadows: ParticleRendererMCastShadowsVariant,
    pub m_receive_shadows: ParticleRendererMReceiveShadowsVariant,
    pub m_lightmap_index: ParticleRendererMLightmapIndexVariant,
    pub m_motion_vectors: Option<u8>,
    pub m_dynamic_occludee: Option<u8>,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_lightmap_index_dynamic: Option<u16>,
    pub m_light_probe_usage: Option<u8>,
    pub m_materials: ParticleRendererMMaterialsVariant,
    pub m_reflection_probe_usage: Option<ParticleRendererMReflectionProbeUsageVariant>,
    pub m_subset_indices: Option<Vec<u32>>,
    pub m_lightmap_tiling_offset_dynamic: Option<Vector4f>,
    pub m_static_batch_root: PPtr,
    pub m_rendering_layer_mask: Option<u32>,
    pub m_camera_velocity_scale: f32,
    pub m_use_light_probes: Option<bool>,
    pub m_stretch_particles: i32,
    pub m_light_probe_anchor: Option<ParticleRendererMLightProbeAnchorVariant>,
    pub m_length_scale: f32,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<ParticleRendererMSortingLayerIdVariant>,
    pub m_velocity_scale: f32,
    pub m_sorting_order: Option<ParticleRendererMSortingOrderVariant>,
    pub m_static_batch_info: Option<StaticBatchInfo>,
    pub m_max_particle_size: f32,
    pub m_probe_anchor: Option<ParticleRendererMProbeAnchorVariant>,
    pub uv_animation: UvAnimation,
    pub m_light_probe_volume_override: Option<PPtr>,
}

impl crate::Parse for ParticleRenderer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_cast_shadows = None;
        let mut m_receive_shadows = None;
        let mut m_lightmap_index = None;
        let mut m_motion_vectors = None;
        let mut m_dynamic_occludee = None;
        let mut m_lightmap_tiling_offset = None;
        let mut m_lightmap_index_dynamic = None;
        let mut m_light_probe_usage = None;
        let mut m_materials = None;
        let mut m_reflection_probe_usage = None;
        let mut m_subset_indices = None;
        let mut m_lightmap_tiling_offset_dynamic = None;
        let mut m_static_batch_root = None;
        let mut m_rendering_layer_mask = None;
        let mut m_camera_velocity_scale = None;
        let mut m_use_light_probes = None;
        let mut m_stretch_particles = None;
        let mut m_light_probe_anchor = None;
        let mut m_length_scale = None;
        let mut m_sorting_layer = None;
        let mut m_sorting_layer_id = None;
        let mut m_velocity_scale = None;
        let mut m_sorting_order = None;
        let mut m_static_batch_info = None;
        let mut m_max_particle_size = None;
        let mut m_probe_anchor = None;
        let mut uv_animation = None;
        let mut m_light_probe_volume_override = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(ParticleRendererMEnabledVariant::parse(rdr, ver)?);
        m_cast_shadows = Some(ParticleRendererMCastShadowsVariant::parse(rdr, ver)?);
        m_receive_shadows = Some(ParticleRendererMReceiveShadowsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleRendererMReflectionProbeUsageVariant::parse(
                rdr, ver,
            )?);
            m_rendering_layer_mask = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_dynamic_occludee = Some(rdr.read_u8()?);
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleRendererMReflectionProbeUsageVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_motion_vectors = Some(rdr.read_u8()?);
            m_light_probe_usage = Some(rdr.read_u8()?);
            m_reflection_probe_usage = Some(ParticleRendererMReflectionProbeUsageVariant::parse(
                rdr, ver,
            )?);
        } else {
        }
        m_lightmap_index = Some(ParticleRendererMLightmapIndexVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_index_dynamic = Some(rdr.read_u16()?);
        } else {
        }
        m_lightmap_tiling_offset = Some(Vector4f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap_tiling_offset_dynamic = Some(Vector4f::parse(rdr, ver)?);
        } else {
        }
        m_materials = Some(ParticleRendererMMaterialsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_static_batch_info = Some(StaticBatchInfo::parse(rdr, ver)?);
        } else {
            m_subset_indices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }
        m_static_batch_root = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_probe_anchor = Some(ParticleRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(ParticleRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(ParticleRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_probe_anchor = Some(ParticleRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_light_probe_volume_override = Some(crate::PPtr::parse(rdr, ver)?);
            rdr.align(4)?;
            m_sorting_layer_id = Some(ParticleRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(ParticleRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_reflection_probe_usage = Some(ParticleRendererMReflectionProbeUsageVariant::parse(
                rdr, ver,
            )?);
            m_probe_anchor = Some(ParticleRendererMProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(ParticleRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(ParticleRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(ParticleRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(ParticleRendererMSortingLayerIdVariant::parse(rdr, ver)?);
            m_sorting_order = Some(ParticleRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(ParticleRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(ParticleRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(ParticleRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
        } else {
        }
        m_camera_velocity_scale = Some(rdr.read_f32()?);
        m_stretch_particles = Some(rdr.read_i32()?);
        m_length_scale = Some(rdr.read_f32()?);
        m_velocity_scale = Some(rdr.read_f32()?);
        m_max_particle_size = Some(rdr.read_f32()?);
        uv_animation = Some(UvAnimation::parse(rdr, ver)?);

        Ok(ParticleRenderer {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_cast_shadows: m_cast_shadows.unwrap(),
            m_receive_shadows: m_receive_shadows.unwrap(),
            m_lightmap_index: m_lightmap_index.unwrap(),
            m_motion_vectors,
            m_dynamic_occludee,
            m_lightmap_tiling_offset: m_lightmap_tiling_offset.unwrap(),
            m_lightmap_index_dynamic,
            m_light_probe_usage,
            m_materials: m_materials.unwrap(),
            m_reflection_probe_usage,
            m_subset_indices,
            m_lightmap_tiling_offset_dynamic,
            m_static_batch_root: m_static_batch_root.unwrap(),
            m_rendering_layer_mask,
            m_camera_velocity_scale: m_camera_velocity_scale.unwrap(),
            m_use_light_probes,
            m_stretch_particles: m_stretch_particles.unwrap(),
            m_light_probe_anchor,
            m_length_scale: m_length_scale.unwrap(),
            m_sorting_layer,
            m_sorting_layer_id,
            m_velocity_scale: m_velocity_scale.unwrap(),
            m_sorting_order,
            m_static_batch_info,
            m_max_particle_size: m_max_particle_size.unwrap(),
            m_probe_anchor,
            uv_animation: uv_animation.unwrap(),
            m_light_probe_volume_override,
        })
    }
}
