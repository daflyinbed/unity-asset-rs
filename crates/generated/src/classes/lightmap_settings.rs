//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::enlighten_scene_mapping::EnlightenSceneMapping;
use crate::subclasses::gi_settings::GiSettings;
use crate::subclasses::lightmap_data::LightmapData;
use super::level_game_manager::LevelGameManager;

#[derive(Debug, Clone)]
pub struct LightmapSettings {
    pub parent: LevelGameManager,
    pub m_lightmaps: Vec<LightmapData>,
    pub m_light_probes: Option<PPtr>,
    pub m_enlighten_scene_mapping: Option<EnlightenSceneMapping>,
    pub m_lightmaps_mode: i32,
    pub m_bake_on_scene_load: Option<i32>,
    pub m_use_dual_lightmaps_in_forward: Option<bool>,
    pub m_baked_color_space: Option<i32>,
    pub m_gi_settings: Option<GiSettings>,
    pub m_runtime_cpu_usage: Option<i32>,
    pub m_shadow_mask_mode: Option<i32>,
    pub m_use_shadowmask: Option<bool>,
    pub m_lighting_settings: Option<PPtr>,
}

impl crate::Parse for LightmapSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = LevelGameManager::parse(r, ver)?;
        let m_lightmaps = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LightmapData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_light_probes = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };
        let m_enlighten_scene_mapping = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(EnlightenSceneMapping::parse(r, ver)?)
        } else {
            None
        };
        let m_lightmaps_mode = r.read_i32()?;
        let m_bake_on_scene_load = if *ver >= crate::UnityVersion::new(6000, 0, 22) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_use_dual_lightmaps_in_forward = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_baked_color_space = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_gi_settings = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = GiSettings::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_runtime_cpu_usage = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_shadow_mask_mode = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_use_shadowmask = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_lighting_settings = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(crate::PPtr::parse(r, ver)?)
        } else {
            None
        };

        Ok(LightmapSettings {
            parent,
            m_lightmaps,
            m_light_probes,
            m_enlighten_scene_mapping,
            m_lightmaps_mode,
            m_bake_on_scene_load,
            m_use_dual_lightmaps_in_forward,
            m_baked_color_space,
            m_gi_settings,
            m_runtime_cpu_usage,
            m_shadow_mask_mode,
            m_use_shadowmask,
            m_lighting_settings,
        })
    }
}
