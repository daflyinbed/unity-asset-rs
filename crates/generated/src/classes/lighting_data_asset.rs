//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::enlighten_scene_mapping::EnlightenSceneMapping;
use crate::subclasses::guid::Guid;
use crate::subclasses::light_baking_output::LightBakingOutput;
use crate::subclasses::lightmap_data::LightmapData;
use crate::subclasses::renderer_data::RendererData;
use crate::subclasses::scene_object_identifier::SceneObjectIdentifier;
use crate::subclasses::spherical_harmonics_l2::SphericalHarmonicsL2;


/// 版本枚举：LightingDataAssetMLightmapsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightingDataAssetMLightmapsVariant {
    Default(Vec<LightmapData>),
    V2017_1(Vec<LightmapData>),
}

impl crate::Parse for LightingDataAssetMLightmapsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { LightmapData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { LightmapData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightingDataAssetMLightmappedRendererDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightingDataAssetMLightmappedRendererDataVariant {
    Default(Vec<RendererData>),
    V2017_1(Vec<RendererData>),
}

impl crate::Parse for LightingDataAssetMLightmappedRendererDataVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { RendererData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { RendererData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightingDataAssetMLightmappedRendererDataIDsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightingDataAssetMLightmappedRendererDataIDsVariant {
    Default(Vec<SceneObjectIdentifier>),
    V2017_1(Vec<SceneObjectIdentifier>),
}

impl crate::Parse for LightingDataAssetMLightmappedRendererDataIDsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightingDataAssetMEnlightenSceneMappingRendererIDsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightingDataAssetMEnlightenSceneMappingRendererIDsVariant {
    Default(Vec<SceneObjectIdentifier>),
    V2017_1(Vec<SceneObjectIdentifier>),
}

impl crate::Parse for LightingDataAssetMEnlightenSceneMappingRendererIDsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightingDataAssetMLightsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightingDataAssetMLightsVariant {
    Default(Vec<SceneObjectIdentifier>),
    V2017_1(Vec<SceneObjectIdentifier>),
}

impl crate::Parse for LightingDataAssetMLightsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightingDataAssetMBakedReflectionProbeCubemapsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightingDataAssetMBakedReflectionProbeCubemapsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for LightingDataAssetMBakedReflectionProbeCubemapsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightingDataAssetMBakedReflectionProbesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightingDataAssetMBakedReflectionProbesVariant {
    Default(Vec<SceneObjectIdentifier>),
    V2017_1(Vec<SceneObjectIdentifier>),
}

impl crate::Parse for LightingDataAssetMBakedReflectionProbesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：LightingDataAssetMLightBakingOutputsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightingDataAssetMLightBakingOutputsVariant {
    Default(Vec<LightBakingOutput>),
    V2017_1(Vec<LightBakingOutput>),
}

impl crate::Parse for LightingDataAssetMLightBakingOutputsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { LightBakingOutput::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { LightBakingOutput::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct LightingDataAsset {
    pub m_name: String,
    pub m_lightmaps: LightingDataAssetMLightmapsVariant,
    pub m_scene_guid: Option<Guid>,
    pub m_scene: Option<PPtr>,
    pub m_light_probes: PPtr,
    pub m_baked_ambient_probes_in_linear: Option<Vec<SphericalHarmonicsL2>>,
    pub m_baked_ambient_probe_in_linear: Option<SphericalHarmonicsL2>,
    pub m_lightmaps_mode: Option<i32>,
    pub m_lightmaps_cache_files: Option<Vec<String>>,
    pub m_ao_textures: Option<Vec<PPtr>>,
    pub m_baked_ambient_probes_in_gamma: Option<Vec<SphericalHarmonicsL2>>,
    pub m_lightmapped_renderer_data: LightingDataAssetMLightmappedRendererDataVariant,
    pub m_lightmapped_renderer_data_i_ds: LightingDataAssetMLightmappedRendererDataIDsVariant,
    pub m_baked_ambient_probe_in_gamma: Option<SphericalHarmonicsL2>,
    pub m_enlighten_scene_mapping: EnlightenSceneMapping,
    pub m_enlighten_scene_mapping_renderer_i_ds: LightingDataAssetMEnlightenSceneMappingRendererIDsVariant,
    pub m_lights: LightingDataAssetMLightsVariant,
    pub m_baked_reflection_probe_cubemaps: LightingDataAssetMBakedReflectionProbeCubemapsVariant,
    pub m_baked_reflection_probes: LightingDataAssetMBakedReflectionProbesVariant,
    pub m_baked_light_indices: Option<Vec<i32>>,
    pub m_light_baking_outputs: Option<LightingDataAssetMLightBakingOutputsVariant>,
    pub m_baked_skybox_probe_cubemaps: Option<Vec<PPtr>>,
    pub m_enlighten_data: Vec<u8>,
    pub m_enlighten_data_version: Option<i32>,
    pub m_baked_reflection_probe_cubemap_cache_files: Option<Vec<String>>,
}

impl crate::Parse for LightingDataAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
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
            m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_scene = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_scene_guid = Some(Guid::parse(rdr, ver)?);
        } else {
        }
            m_lightmaps = Some(LightingDataAssetMLightmapsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_ao_textures = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_ao_textures = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_lightmaps_cache_files = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_lightmaps_cache_files = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }
            m_light_probes = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_lightmaps_mode = Some(rdr.read_i32()?);
            m_baked_ambient_probe_in_linear = Some(SphericalHarmonicsL2::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_baked_ambient_probe_in_linear = Some(SphericalHarmonicsL2::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_baked_ambient_probe_in_linear = Some(SphericalHarmonicsL2::parse(rdr, ver)?);
            m_baked_ambient_probe_in_gamma = Some(SphericalHarmonicsL2::parse(rdr, ver)?);
        } else {
            m_baked_ambient_probes_in_linear = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SphericalHarmonicsL2::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_baked_ambient_probes_in_gamma = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SphericalHarmonicsL2::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_lightmapped_renderer_data = Some(LightingDataAssetMLightmappedRendererDataVariant::parse(rdr, ver)?);
            m_lightmapped_renderer_data_i_ds = Some(LightingDataAssetMLightmappedRendererDataIDsVariant::parse(rdr, ver)?);
            m_enlighten_scene_mapping = Some(EnlightenSceneMapping::parse(rdr, ver)?);
            m_enlighten_scene_mapping_renderer_i_ds = Some(LightingDataAssetMEnlightenSceneMappingRendererIDsVariant::parse(rdr, ver)?);
            m_lights = Some(LightingDataAssetMLightsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_light_baking_outputs = Some(LightingDataAssetMLightBakingOutputsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_light_baking_outputs = Some(LightingDataAssetMLightBakingOutputsVariant::parse(rdr, ver)?);
            m_baked_reflection_probe_cubemap_cache_files = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_light_baking_outputs = Some(LightingDataAssetMLightBakingOutputsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_baked_light_indices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
        }
            m_baked_reflection_probe_cubemaps = Some(LightingDataAssetMBakedReflectionProbeCubemapsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
        } else {
            m_baked_skybox_probe_cubemaps = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        }
            m_baked_reflection_probes = Some(LightingDataAssetMBakedReflectionProbesVariant::parse(rdr, ver)?);
            m_enlighten_data = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 3, 6) {
            m_enlighten_data_version = Some(rdr.read_i32()?);
        } else {
        }

        Ok(LightingDataAsset {
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
