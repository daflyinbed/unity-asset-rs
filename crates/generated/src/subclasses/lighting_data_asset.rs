//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::enlighten_scene_mapping::EnlightenSceneMapping;
use crate::subclasses::guid::Guid;
use crate::subclasses::lightmap_data::LightmapData;
use crate::subclasses::renderer_data::RendererData;
use crate::subclasses::scene_object_identifier::SceneObjectIdentifier;
use crate::subclasses::spherical_harmonics_l2::SphericalHarmonicsL2;

#[derive(Debug, Clone)]
pub struct LightingDataAsset {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_scene_guid: Guid,
    pub m_lightmaps: Vec<LightmapData>,
    pub m_light_probes: PPtr,
    pub m_baked_ambient_probe_in_linear: SphericalHarmonicsL2,
    pub m_lightmapped_renderer_data: Vec<RendererData>,
    pub m_lightmapped_renderer_data_i_ds: Vec<SceneObjectIdentifier>,
    pub m_enlighten_scene_mapping: EnlightenSceneMapping,
    pub m_enlighten_scene_mapping_renderer_i_ds: Vec<SceneObjectIdentifier>,
    pub m_lights: Vec<SceneObjectIdentifier>,
    pub m_baked_reflection_probe_cubemaps: Vec<PPtr>,
    pub m_baked_reflection_probes: Vec<SceneObjectIdentifier>,
    pub m_enlighten_data: Vec<u8>,
}

impl crate::Parse for LightingDataAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_scene_guid = Guid::parse(r, ver)?;
        let m_lightmaps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_light_probes = crate::PPtr::parse(r, ver)?;
        let m_baked_ambient_probe_in_linear = SphericalHarmonicsL2::parse(r, ver)?;
        let m_lightmapped_renderer_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { RendererData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_lightmapped_renderer_data_i_ds = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_enlighten_scene_mapping = EnlightenSceneMapping::parse(r, ver)?;
        let m_enlighten_scene_mapping_renderer_i_ds = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_lights = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_baked_reflection_probe_cubemaps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_baked_reflection_probes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SceneObjectIdentifier::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_enlighten_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(LightingDataAsset {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_scene_guid,
            m_lightmaps,
            m_light_probes,
            m_baked_ambient_probe_in_linear,
            m_lightmapped_renderer_data,
            m_lightmapped_renderer_data_i_ds,
            m_enlighten_scene_mapping,
            m_enlighten_scene_mapping_renderer_i_ds,
            m_lights,
            m_baked_reflection_probe_cubemaps,
            m_baked_reflection_probes,
            m_enlighten_data,
        })
    }
}
