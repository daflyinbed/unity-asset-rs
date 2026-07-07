//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::enlighten_scene_mapping::EnlightenSceneMapping;
use crate::subclasses::guid::Guid;
use crate::subclasses::light_baking_output::LightBakingOutput;
use crate::subclasses::lightmap_data::LightmapData;
use crate::subclasses::renderer_data::RendererData;
use crate::subclasses::scene_object_identifier::SceneObjectIdentifier;
use crate::subclasses::spherical_harmonics_l2::SphericalHarmonicsL2;

#[derive(Debug, Clone)]
pub struct LightmapSnapshot {
    pub m_name: String,
    pub m_lightmaps: Vec<LightmapData>,
    pub m_scene_guid: Option<Guid>,
    pub m_scene: Option<PPtr>,
    pub m_light_probes: PPtr,
    pub m_baked_ambient_probes_in_linear: Option<Vec<SphericalHarmonicsL2>>,
    pub m_baked_ambient_probe_in_linear: Option<SphericalHarmonicsL2>,
    pub m_lightmaps_mode: Option<i32>,
    pub m_lightmaps_cache_files: Option<Vec<String>>,
    pub m_ao_textures: Option<Vec<PPtr>>,
    pub m_baked_ambient_probes_in_gamma: Option<Vec<SphericalHarmonicsL2>>,
    pub m_lightmapped_renderer_data: Vec<RendererData>,
    pub m_lightmapped_renderer_data_i_ds: Vec<SceneObjectIdentifier>,
    pub m_baked_ambient_probe_in_gamma: Option<SphericalHarmonicsL2>,
    pub m_enlighten_scene_mapping: EnlightenSceneMapping,
    pub m_enlighten_scene_mapping_renderer_i_ds: Vec<SceneObjectIdentifier>,
    pub m_lights: Vec<SceneObjectIdentifier>,
    pub m_baked_reflection_probe_cubemaps: Vec<PPtr>,
    pub m_baked_reflection_probes: Vec<SceneObjectIdentifier>,
    pub m_baked_light_indices: Option<Vec<i32>>,
    pub m_light_baking_outputs: Option<Vec<LightBakingOutput>>,
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
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_lightmaps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_scene_guid = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(Guid::parse(r, ver)?)
        } else {
            None
        };
        let m_scene = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_light_probes = crate::PPtr::parse(r, ver)?;
        let m_baked_ambient_probes_in_linear = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SphericalHarmonicsL2::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_baked_ambient_probe_in_linear = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(SphericalHarmonicsL2::parse(r, ver)?)
        } else {
            None
        };
        let m_lightmaps_mode = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_lightmaps_cache_files = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_ao_textures = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_baked_ambient_probes_in_gamma = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SphericalHarmonicsL2::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_lightmapped_renderer_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { RendererData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_lightmapped_renderer_data_i_ds = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_baked_ambient_probe_in_gamma = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(SphericalHarmonicsL2::parse(r, ver)?)
        } else {
            None
        };
        let m_enlighten_scene_mapping = EnlightenSceneMapping::parse(r, ver)?;
        r.align(4)?;
        let m_enlighten_scene_mapping_renderer_i_ds = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_lights = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_baked_reflection_probe_cubemaps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_baked_reflection_probes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_baked_light_indices = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_light_baking_outputs = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightBakingOutput::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_baked_skybox_probe_cubemaps = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_enlighten_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_enlighten_data_version = if *ver >= crate::UnityVersion::new(5, 3, 6) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_baked_reflection_probe_cubemap_cache_files = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(LightmapSnapshot {
            m_name,
            m_lightmaps,
            m_scene_guid,
            m_scene,
            m_light_probes,
            m_baked_ambient_probes_in_linear,
            m_baked_ambient_probe_in_linear,
            m_lightmaps_mode,
            m_lightmaps_cache_files,
            m_ao_textures,
            m_baked_ambient_probes_in_gamma,
            m_lightmapped_renderer_data,
            m_lightmapped_renderer_data_i_ds,
            m_baked_ambient_probe_in_gamma,
            m_enlighten_scene_mapping,
            m_enlighten_scene_mapping_renderer_i_ds,
            m_lights,
            m_baked_reflection_probe_cubemaps,
            m_baked_reflection_probes,
            m_baked_light_indices,
            m_light_baking_outputs,
            m_baked_skybox_probe_cubemaps,
            m_enlighten_data,
            m_enlighten_data_version,
            m_baked_reflection_probe_cubemap_cache_files,
        })
    }
}
