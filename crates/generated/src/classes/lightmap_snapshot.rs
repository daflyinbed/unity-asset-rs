//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::enlighten_scene_mapping::EnlightenSceneMapping;
use crate::subclasses::guid::Guid;
use crate::subclasses::light_baking_output::LightBakingOutput;
use crate::subclasses::lightmap_data::LightmapData;
use crate::subclasses::renderer_data::RendererData;
use crate::subclasses::scene_object_identifier::SceneObjectIdentifier;
use crate::subclasses::spherical_harmonics_l2::SphericalHarmonicsL2;


/// 版本枚举：LightmapSnapshotMLightmapsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSnapshotMLightmapsVariant {
    Default(Vec<LightmapData>),
    V2017_1(Vec<LightmapData>),
}

impl crate::Parse for LightmapSnapshotMLightmapsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightmapSnapshotMLightmappedRendererDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSnapshotMLightmappedRendererDataVariant {
    Default(Vec<RendererData>),
    V2017_1(Vec<RendererData>),
}

impl crate::Parse for LightmapSnapshotMLightmappedRendererDataVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { RendererData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { RendererData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightmapSnapshotMLightmappedRendererDataIDsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSnapshotMLightmappedRendererDataIDsVariant {
    Default(Vec<SceneObjectIdentifier>),
    V2017_1(Vec<SceneObjectIdentifier>),
}

impl crate::Parse for LightmapSnapshotMLightmappedRendererDataIDsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightmapSnapshotMEnlightenSceneMappingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSnapshotMEnlightenSceneMappingVariant {
    Default(EnlightenSceneMapping),
    V2017_1(EnlightenSceneMapping),
}

impl crate::Parse for LightmapSnapshotMEnlightenSceneMappingVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = EnlightenSceneMapping::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = EnlightenSceneMapping::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightmapSnapshotMEnlightenSceneMappingRendererIDsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSnapshotMEnlightenSceneMappingRendererIDsVariant {
    Default(Vec<SceneObjectIdentifier>),
    V2017_1(Vec<SceneObjectIdentifier>),
}

impl crate::Parse for LightmapSnapshotMEnlightenSceneMappingRendererIDsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightmapSnapshotMLightsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSnapshotMLightsVariant {
    Default(Vec<SceneObjectIdentifier>),
    V2017_1(Vec<SceneObjectIdentifier>),
}

impl crate::Parse for LightmapSnapshotMLightsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightmapSnapshotMBakedReflectionProbeCubemapsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSnapshotMBakedReflectionProbeCubemapsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for LightmapSnapshotMBakedReflectionProbeCubemapsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightmapSnapshotMBakedReflectionProbesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSnapshotMBakedReflectionProbesVariant {
    Default(Vec<SceneObjectIdentifier>),
    V2017_1(Vec<SceneObjectIdentifier>),
}

impl crate::Parse for LightmapSnapshotMBakedReflectionProbesVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightmapSnapshotMLightBakingOutputsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSnapshotMLightBakingOutputsVariant {
    Default(Vec<LightBakingOutput>),
    V2017_1(Vec<LightBakingOutput>),
}

impl crate::Parse for LightmapSnapshotMLightBakingOutputsVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightBakingOutput::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightBakingOutput::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct LightmapSnapshot {
    pub m_name: String,
    pub m_lightmaps: LightmapSnapshotMLightmapsVariant,
    pub m_scene_guid: Option<Guid>,
    pub m_scene: Option<PPtr>,
    pub m_light_probes: PPtr,
    pub m_baked_ambient_probes_in_linear: Option<Vec<SphericalHarmonicsL2>>,
    pub m_baked_ambient_probe_in_linear: Option<SphericalHarmonicsL2>,
    pub m_lightmaps_mode: Option<i32>,
    pub m_lightmaps_cache_files: Option<Vec<String>>,
    pub m_ao_textures: Option<Vec<PPtr>>,
    pub m_baked_ambient_probes_in_gamma: Option<Vec<SphericalHarmonicsL2>>,
    pub m_lightmapped_renderer_data: LightmapSnapshotMLightmappedRendererDataVariant,
    pub m_lightmapped_renderer_data_i_ds: LightmapSnapshotMLightmappedRendererDataIDsVariant,
    pub m_baked_ambient_probe_in_gamma: Option<SphericalHarmonicsL2>,
    pub m_enlighten_scene_mapping: LightmapSnapshotMEnlightenSceneMappingVariant,
    pub m_enlighten_scene_mapping_renderer_i_ds: LightmapSnapshotMEnlightenSceneMappingRendererIDsVariant,
    pub m_lights: LightmapSnapshotMLightsVariant,
    pub m_baked_reflection_probe_cubemaps: LightmapSnapshotMBakedReflectionProbeCubemapsVariant,
    pub m_baked_reflection_probes: LightmapSnapshotMBakedReflectionProbesVariant,
    pub m_baked_light_indices: Option<Vec<i32>>,
    pub m_light_baking_outputs: Option<LightmapSnapshotMLightBakingOutputsVariant>,
    pub m_baked_skybox_probe_cubemaps: Option<Vec<PPtr>>,
    pub m_enlighten_data: Vec<u8>,
    pub m_enlighten_data_version: Option<i32>,
    pub m_baked_reflection_probe_cubemap_cache_files: Option<Vec<String>>,
}

impl crate::Parse for LightmapSnapshot {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_lightmaps = None;
        let mut m_scene_guid = None;
        let mut m_scene = None;
        let mut m_light_probes = None;
        let mut m_baked_ambient_probes_in_linear = None;
        let mut m_baked_ambient_probe_in_linear = None;
        let mut m_lightmaps_mode = None;
        let mut m_lightmaps_cache_files = None;
        let mut m_ao_textures = None;
        let mut m_baked_ambient_probes_in_gamma = None;
        let mut m_lightmapped_renderer_data = None;
        let mut m_lightmapped_renderer_data_i_ds = None;
        let mut m_baked_ambient_probe_in_gamma = None;
        let mut m_enlighten_scene_mapping = None;
        let mut m_enlighten_scene_mapping_renderer_i_ds = None;
        let mut m_lights = None;
        let mut m_baked_reflection_probe_cubemaps = None;
        let mut m_baked_reflection_probes = None;
        let mut m_baked_light_indices = None;
        let mut m_light_baking_outputs = None;
        let mut m_baked_skybox_probe_cubemaps = None;
        let mut m_enlighten_data = None;
        let mut m_enlighten_data_version = None;
        let mut m_baked_reflection_probe_cubemap_cache_files = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_scene = Some(crate::PPtr::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_scene_guid = Some(Guid::parse(r, ver)?);
        } else {
        }
            m_lightmaps = Some(LightmapSnapshotMLightmapsVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_ao_textures = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_ao_textures = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_lightmaps_cache_files = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_lightmaps_cache_files = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else {
        }
            m_light_probes = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_lightmaps_mode = Some(r.read_i32()?);
            m_baked_ambient_probe_in_linear = Some(SphericalHarmonicsL2::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_baked_ambient_probe_in_linear = Some(SphericalHarmonicsL2::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_baked_ambient_probe_in_linear = Some(SphericalHarmonicsL2::parse(r, ver)?);
            m_baked_ambient_probe_in_gamma = Some(SphericalHarmonicsL2::parse(r, ver)?);
        } else {
            m_baked_ambient_probes_in_linear = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SphericalHarmonicsL2::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_baked_ambient_probes_in_gamma = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SphericalHarmonicsL2::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_lightmapped_renderer_data = Some(LightmapSnapshotMLightmappedRendererDataVariant::parse(r, ver)?);
            m_lightmapped_renderer_data_i_ds = Some(LightmapSnapshotMLightmappedRendererDataIDsVariant::parse(r, ver)?);
            m_enlighten_scene_mapping = Some(LightmapSnapshotMEnlightenSceneMappingVariant::parse(r, ver)?);
            m_enlighten_scene_mapping_renderer_i_ds = Some(LightmapSnapshotMEnlightenSceneMappingRendererIDsVariant::parse(r, ver)?);
            m_lights = Some(LightmapSnapshotMLightsVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_light_baking_outputs = Some(LightmapSnapshotMLightBakingOutputsVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_light_baking_outputs = Some(LightmapSnapshotMLightBakingOutputsVariant::parse(r, ver)?);
            m_baked_reflection_probe_cubemap_cache_files = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_light_baking_outputs = Some(LightmapSnapshotMLightBakingOutputsVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_baked_light_indices = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
        }
            m_baked_reflection_probe_cubemaps = Some(LightmapSnapshotMBakedReflectionProbeCubemapsVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
        } else {
            m_baked_skybox_probe_cubemaps = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_baked_reflection_probes = Some(LightmapSnapshotMBakedReflectionProbesVariant::parse(r, ver)?);
            m_enlighten_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 3, 6) {
            m_enlighten_data_version = Some(r.read_i32()?);
        } else {
        }

        Ok(LightmapSnapshot {
            m_name: m_name.unwrap(),
            m_lightmaps: m_lightmaps.unwrap(),
            m_scene_guid,
            m_scene,
            m_light_probes: m_light_probes.unwrap(),
            m_baked_ambient_probes_in_linear,
            m_baked_ambient_probe_in_linear,
            m_lightmaps_mode,
            m_lightmaps_cache_files,
            m_ao_textures,
            m_baked_ambient_probes_in_gamma,
            m_lightmapped_renderer_data: m_lightmapped_renderer_data.unwrap(),
            m_lightmapped_renderer_data_i_ds: m_lightmapped_renderer_data_i_ds.unwrap(),
            m_baked_ambient_probe_in_gamma,
            m_enlighten_scene_mapping: m_enlighten_scene_mapping.unwrap(),
            m_enlighten_scene_mapping_renderer_i_ds: m_enlighten_scene_mapping_renderer_i_ds.unwrap(),
            m_lights: m_lights.unwrap(),
            m_baked_reflection_probe_cubemaps: m_baked_reflection_probe_cubemaps.unwrap(),
            m_baked_reflection_probes: m_baked_reflection_probes.unwrap(),
            m_baked_light_indices,
            m_light_baking_outputs,
            m_baked_skybox_probe_cubemaps,
            m_enlighten_data: m_enlighten_data.unwrap(),
            m_enlighten_data_version,
            m_baked_reflection_probe_cubemap_cache_files,
        })
    }
}
