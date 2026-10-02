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

impl crate::ToValue for ParticleRendererMEnabledVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ParticleRendererMCastShadowsVariant {
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

impl crate::ToValue for ParticleRendererMReceiveShadowsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ParticleRendererMLightmapIndexVariant {
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

impl crate::ToValue for ParticleRendererMMaterialsVariant {
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

impl crate::ToValue for ParticleRendererMReflectionProbeUsageVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ParticleRendererMLightProbeAnchorVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_3(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ParticleRendererMSortingLayerIdVariant {
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

impl crate::ToValue for ParticleRendererMSortingOrderVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_5(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ParticleRendererMProbeAnchorVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ParticleRenderer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_CastShadows".to_string(), self.m_cast_shadows.to_value())),
            Some((
                "m_ReceiveShadows".to_string(),
                self.m_receive_shadows.to_value(),
            )),
            Some((
                "m_LightmapIndex".to_string(),
                self.m_lightmap_index.to_value(),
            )),
            crate::opt_field("m_MotionVectors", &self.m_motion_vectors),
            crate::opt_field("m_DynamicOccludee", &self.m_dynamic_occludee),
            Some((
                "m_LightmapTilingOffset".to_string(),
                self.m_lightmap_tiling_offset.to_value(),
            )),
            crate::opt_field("m_LightmapIndexDynamic", &self.m_lightmap_index_dynamic),
            crate::opt_field("m_LightProbeUsage", &self.m_light_probe_usage),
            Some(("m_Materials".to_string(), self.m_materials.to_value())),
            crate::opt_field("m_ReflectionProbeUsage", &self.m_reflection_probe_usage),
            crate::opt_field("m_SubsetIndices", &self.m_subset_indices),
            crate::opt_field(
                "m_LightmapTilingOffsetDynamic",
                &self.m_lightmap_tiling_offset_dynamic,
            ),
            Some((
                "m_StaticBatchRoot".to_string(),
                self.m_static_batch_root.to_value(),
            )),
            crate::opt_field("m_RenderingLayerMask", &self.m_rendering_layer_mask),
            Some((
                "m_CameraVelocityScale".to_string(),
                self.m_camera_velocity_scale.to_value(),
            )),
            crate::opt_field("m_UseLightProbes", &self.m_use_light_probes),
            Some((
                "m_StretchParticles".to_string(),
                self.m_stretch_particles.to_value(),
            )),
            crate::opt_field("m_LightProbeAnchor", &self.m_light_probe_anchor),
            Some(("m_LengthScale".to_string(), self.m_length_scale.to_value())),
            crate::opt_field("m_SortingLayer", &self.m_sorting_layer),
            crate::opt_field("m_SortingLayerID", &self.m_sorting_layer_id),
            Some((
                "m_VelocityScale".to_string(),
                self.m_velocity_scale.to_value(),
            )),
            crate::opt_field("m_SortingOrder", &self.m_sorting_order),
            crate::opt_field("m_StaticBatchInfo", &self.m_static_batch_info),
            Some((
                "m_MaxParticleSize".to_string(),
                self.m_max_particle_size.to_value(),
            )),
            crate::opt_field("m_ProbeAnchor", &self.m_probe_anchor),
            Some(("UV Animation".to_string(), self.uv_animation.to_value())),
            crate::opt_field(
                "m_LightProbeVolumeOverride",
                &self.m_light_probe_volume_override,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_CastShadows" => (self.m_cast_shadows).inspect_value(path, offset, limit),
                "m_ReceiveShadows" => (self.m_receive_shadows).inspect_value(path, offset, limit),
                "m_LightmapIndex" => (self.m_lightmap_index).inspect_value(path, offset, limit),
                "m_MotionVectors" => (self.m_motion_vectors).inspect_value(path, offset, limit),
                "m_DynamicOccludee" => (self.m_dynamic_occludee).inspect_value(path, offset, limit),
                "m_LightmapTilingOffset" => {
                    (self.m_lightmap_tiling_offset).inspect_value(path, offset, limit)
                }
                "m_LightmapIndexDynamic" => {
                    (self.m_lightmap_index_dynamic).inspect_value(path, offset, limit)
                }
                "m_LightProbeUsage" => {
                    (self.m_light_probe_usage).inspect_value(path, offset, limit)
                }
                "m_Materials" => (self.m_materials).inspect_value(path, offset, limit),
                "m_ReflectionProbeUsage" => {
                    (self.m_reflection_probe_usage).inspect_value(path, offset, limit)
                }
                "m_SubsetIndices" => (self.m_subset_indices).inspect_value(path, offset, limit),
                "m_LightmapTilingOffsetDynamic" => {
                    (self.m_lightmap_tiling_offset_dynamic).inspect_value(path, offset, limit)
                }
                "m_StaticBatchRoot" => {
                    (self.m_static_batch_root).inspect_value(path, offset, limit)
                }
                "m_RenderingLayerMask" => {
                    (self.m_rendering_layer_mask).inspect_value(path, offset, limit)
                }
                "m_CameraVelocityScale" => {
                    (self.m_camera_velocity_scale).inspect_value(path, offset, limit)
                }
                "m_UseLightProbes" => (self.m_use_light_probes).inspect_value(path, offset, limit),
                "m_StretchParticles" => {
                    (self.m_stretch_particles).inspect_value(path, offset, limit)
                }
                "m_LightProbeAnchor" => {
                    (self.m_light_probe_anchor).inspect_value(path, offset, limit)
                }
                "m_LengthScale" => (self.m_length_scale).inspect_value(path, offset, limit),
                "m_SortingLayer" => (self.m_sorting_layer).inspect_value(path, offset, limit),
                "m_SortingLayerID" => (self.m_sorting_layer_id).inspect_value(path, offset, limit),
                "m_VelocityScale" => (self.m_velocity_scale).inspect_value(path, offset, limit),
                "m_SortingOrder" => (self.m_sorting_order).inspect_value(path, offset, limit),
                "m_StaticBatchInfo" => {
                    (self.m_static_batch_info).inspect_value(path, offset, limit)
                }
                "m_MaxParticleSize" => {
                    (self.m_max_particle_size).inspect_value(path, offset, limit)
                }
                "m_ProbeAnchor" => (self.m_probe_anchor).inspect_value(path, offset, limit),
                "UV Animation" => (self.uv_animation).inspect_value(path, offset, limit),
                "m_LightProbeVolumeOverride" => {
                    (self.m_light_probe_volume_override).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_cast_shadows).inspect_value(path, offset, limit))
                .map(|v| ("m_CastShadows".to_string(), v)),
            ((self.m_receive_shadows).inspect_value(path, offset, limit))
                .map(|v| ("m_ReceiveShadows".to_string(), v)),
            ((self.m_lightmap_index).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapIndex".to_string(), v)),
            ((self.m_motion_vectors).inspect_value(path, offset, limit))
                .map(|v| ("m_MotionVectors".to_string(), v)),
            ((self.m_dynamic_occludee).inspect_value(path, offset, limit))
                .map(|v| ("m_DynamicOccludee".to_string(), v)),
            ((self.m_lightmap_tiling_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapTilingOffset".to_string(), v)),
            ((self.m_lightmap_index_dynamic).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapIndexDynamic".to_string(), v)),
            ((self.m_light_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeUsage".to_string(), v)),
            ((self.m_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_Materials".to_string(), v)),
            ((self.m_reflection_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("m_ReflectionProbeUsage".to_string(), v)),
            ((self.m_subset_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_SubsetIndices".to_string(), v)),
            ((self.m_lightmap_tiling_offset_dynamic).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapTilingOffsetDynamic".to_string(), v)),
            ((self.m_static_batch_root).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticBatchRoot".to_string(), v)),
            ((self.m_rendering_layer_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderingLayerMask".to_string(), v)),
            ((self.m_camera_velocity_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_CameraVelocityScale".to_string(), v)),
            ((self.m_use_light_probes).inspect_value(path, offset, limit))
                .map(|v| ("m_UseLightProbes".to_string(), v)),
            ((self.m_stretch_particles).inspect_value(path, offset, limit))
                .map(|v| ("m_StretchParticles".to_string(), v)),
            ((self.m_light_probe_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeAnchor".to_string(), v)),
            ((self.m_length_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_LengthScale".to_string(), v)),
            ((self.m_sorting_layer).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayer".to_string(), v)),
            ((self.m_sorting_layer_id).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayerID".to_string(), v)),
            ((self.m_velocity_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_VelocityScale".to_string(), v)),
            ((self.m_sorting_order).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingOrder".to_string(), v)),
            ((self.m_static_batch_info).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticBatchInfo".to_string(), v)),
            ((self.m_max_particle_size).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxParticleSize".to_string(), v)),
            ((self.m_probe_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_ProbeAnchor".to_string(), v)),
            ((self.uv_animation).inspect_value(path, offset, limit))
                .map(|v| ("UV Animation".to_string(), v)),
            ((self.m_light_probe_volume_override).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeVolumeOverride".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
