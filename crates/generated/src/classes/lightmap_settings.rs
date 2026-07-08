//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::enlighten_scene_mapping::EnlightenSceneMapping;
use crate::subclasses::gi_settings::GiSettings;
use crate::subclasses::lightmap_data::LightmapData;


/// 版本枚举：LightmapSettingsMLightmapsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSettingsMLightmapsVariant {
    Default(Vec<LightmapData>),
    V2017_1(Vec<LightmapData>),
}

impl crate::Parse for LightmapSettingsMLightmapsVariant {
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

/// 版本枚举：LightmapSettingsMEnlightenSceneMappingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSettingsMEnlightenSceneMappingVariant {
    Default(EnlightenSceneMapping),
    V2017_1(EnlightenSceneMapping),
}

impl crate::Parse for LightmapSettingsMEnlightenSceneMappingVariant {
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

/// 版本枚举：LightmapSettingsMLightmapsModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSettingsMLightmapsModeVariant {
    Default(i32),
    V5_0(i32),
    V2023_2(i32),
}

impl crate::Parse for LightmapSettingsMLightmapsModeVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = r.read_i32()?;
            Ok(Self::V2023_2(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = r.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct LightmapSettings {
    pub m_lightmaps: LightmapSettingsMLightmapsVariant,
    pub m_light_probes: Option<PPtr>,
    pub m_enlighten_scene_mapping: Option<LightmapSettingsMEnlightenSceneMappingVariant>,
    pub m_lightmaps_mode: LightmapSettingsMLightmapsModeVariant,
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
        let mut m_lightmaps = None;
        let mut m_light_probes = None;
        let mut m_enlighten_scene_mapping = None;
        let mut m_lightmaps_mode = None;
        let mut m_bake_on_scene_load = None;
        let mut m_use_dual_lightmaps_in_forward = None;
        let mut m_baked_color_space = None;
        let mut m_gi_settings = None;
        let mut m_runtime_cpu_usage = None;
        let mut m_shadow_mask_mode = None;
        let mut m_use_shadowmask = None;
        let mut m_lighting_settings = None;
        if *ver >= crate::UnityVersion::new(6000, 0, 22) {
            m_enlighten_scene_mapping = Some(LightmapSettingsMEnlightenSceneMappingVariant::parse(r, ver)?);
            m_bake_on_scene_load = Some(r.read_i32()?);
            m_light_probes = Some(crate::PPtr::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enlighten_scene_mapping = Some(LightmapSettingsMEnlightenSceneMappingVariant::parse(r, ver)?);
            m_light_probes = Some(crate::PPtr::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_light_probes = Some(crate::PPtr::parse(r, ver)?);
        } else {
        }
            m_lightmaps = Some(LightmapSettingsMLightmapsVariant::parse(r, ver)?);
            m_lightmaps_mode = Some(LightmapSettingsMLightmapsModeVariant::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_gi_settings = Some(GiSettings::parse(r, ver)?);
            r.align(4)?;
            m_lighting_settings = Some(crate::PPtr::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_gi_settings = Some(GiSettings::parse(r, ver)?);
            r.align(4)?;
            m_use_shadowmask = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_gi_settings = Some(GiSettings::parse(r, ver)?);
            r.align(4)?;
            m_shadow_mask_mode = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_gi_settings = Some(GiSettings::parse(r, ver)?);
            r.align(4)?;
            m_runtime_cpu_usage = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_baked_color_space = Some(r.read_i32()?);
            m_use_dual_lightmaps_in_forward = Some(r.read_bool()?);
            r.align(4)?;
        } else {
            m_use_dual_lightmaps_in_forward = Some(r.read_bool()?);
            r.align(4)?;
        }

        Ok(LightmapSettings {
            m_lightmaps: m_lightmaps.unwrap(),
            m_light_probes,
            m_enlighten_scene_mapping,
            m_lightmaps_mode: m_lightmaps_mode.unwrap(),
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
