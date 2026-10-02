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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| LightmapData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| LightmapData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LightmapSettingsMLightmapsVariant {
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

/// 版本枚举：LightmapSettingsMLightmapsModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSettingsMLightmapsModeVariant {
    Default(i32),
    V5_0(i32),
    V2023_2(i32),
}

impl crate::Parse for LightmapSettingsMLightmapsModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2023_2(v))
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LightmapSettingsMLightmapsModeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
            Self::V2023_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：LightmapSettingsMGiSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LightmapSettingsMGiSettingsVariant {
    Default(Box<GiSettings>),
    V2020_1(Box<GiSettings>),
    V2023_2(Box<GiSettings>),
}

impl crate::Parse for LightmapSettingsMGiSettingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            let v = ({ Ok(Box::new(GiSettings::parse(rdr, ver)?)) })?;
            Ok(Self::V2023_2(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({ Ok(Box::new(GiSettings::parse(rdr, ver)?)) })?;
            rdr.align(4)?;
            Ok(Self::V2020_1(v))
        } else {
            let v = ({ Ok(Box::new(GiSettings::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LightmapSettingsMGiSettingsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2023_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct LightmapSettings {
    pub m_lightmaps: LightmapSettingsMLightmapsVariant,
    pub m_light_probes: Option<PPtr>,
    pub m_enlighten_scene_mapping: Option<EnlightenSceneMapping>,
    pub m_lightmaps_mode: LightmapSettingsMLightmapsModeVariant,
    pub m_bake_on_scene_load: Option<i32>,
    pub m_use_dual_lightmaps_in_forward: Option<bool>,
    pub m_baked_color_space: Option<i32>,
    pub m_gi_settings: Option<LightmapSettingsMGiSettingsVariant>,
    pub m_runtime_cpu_usage: Option<i32>,
    pub m_shadow_mask_mode: Option<i32>,
    pub m_use_shadowmask: Option<bool>,
    pub m_lighting_settings: Option<PPtr>,
}

impl crate::Parse for LightmapSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
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
            m_enlighten_scene_mapping = Some(EnlightenSceneMapping::parse(rdr, ver)?);
            m_bake_on_scene_load = Some(rdr.read_i32()?);
            m_light_probes = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_enlighten_scene_mapping = Some(EnlightenSceneMapping::parse(rdr, ver)?);
            m_light_probes = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_light_probes = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
        m_lightmaps = Some(LightmapSettingsMLightmapsVariant::parse(rdr, ver)?);
        m_lightmaps_mode = Some(LightmapSettingsMLightmapsModeVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_gi_settings = Some(LightmapSettingsMGiSettingsVariant::parse(rdr, ver)?);
            m_lighting_settings = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_gi_settings = Some(LightmapSettingsMGiSettingsVariant::parse(rdr, ver)?);
            m_use_shadowmask = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_gi_settings = Some(LightmapSettingsMGiSettingsVariant::parse(rdr, ver)?);
            m_shadow_mask_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_gi_settings = Some(LightmapSettingsMGiSettingsVariant::parse(rdr, ver)?);
            m_runtime_cpu_usage = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_baked_color_space = Some(rdr.read_i32()?);
            m_use_dual_lightmaps_in_forward = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_use_dual_lightmaps_in_forward = Some(rdr.read_bool()?);
            rdr.align(4)?;
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

impl crate::ToValue for LightmapSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Lightmaps".to_string(), self.m_lightmaps.to_value())),
            crate::opt_field("m_LightProbes", &self.m_light_probes),
            crate::opt_field("m_EnlightenSceneMapping", &self.m_enlighten_scene_mapping),
            Some((
                "m_LightmapsMode".to_string(),
                self.m_lightmaps_mode.to_value(),
            )),
            crate::opt_field("m_BakeOnSceneLoad", &self.m_bake_on_scene_load),
            crate::opt_field(
                "m_UseDualLightmapsInForward",
                &self.m_use_dual_lightmaps_in_forward,
            ),
            crate::opt_field("m_BakedColorSpace", &self.m_baked_color_space),
            crate::opt_field("m_GISettings", &self.m_gi_settings),
            crate::opt_field("m_RuntimeCPUUsage", &self.m_runtime_cpu_usage),
            crate::opt_field("m_ShadowMaskMode", &self.m_shadow_mask_mode),
            crate::opt_field("m_UseShadowmask", &self.m_use_shadowmask),
            crate::opt_field("m_LightingSettings", &self.m_lighting_settings),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Lightmaps" => (self.m_lightmaps).inspect_value(path, offset, limit),
                "m_LightProbes" => (self.m_light_probes).inspect_value(path, offset, limit),
                "m_EnlightenSceneMapping" => {
                    (self.m_enlighten_scene_mapping).inspect_value(path, offset, limit)
                }
                "m_LightmapsMode" => (self.m_lightmaps_mode).inspect_value(path, offset, limit),
                "m_BakeOnSceneLoad" => {
                    (self.m_bake_on_scene_load).inspect_value(path, offset, limit)
                }
                "m_UseDualLightmapsInForward" => {
                    (self.m_use_dual_lightmaps_in_forward).inspect_value(path, offset, limit)
                }
                "m_BakedColorSpace" => {
                    (self.m_baked_color_space).inspect_value(path, offset, limit)
                }
                "m_GISettings" => (self.m_gi_settings).inspect_value(path, offset, limit),
                "m_RuntimeCPUUsage" => {
                    (self.m_runtime_cpu_usage).inspect_value(path, offset, limit)
                }
                "m_ShadowMaskMode" => (self.m_shadow_mask_mode).inspect_value(path, offset, limit),
                "m_UseShadowmask" => (self.m_use_shadowmask).inspect_value(path, offset, limit),
                "m_LightingSettings" => {
                    (self.m_lighting_settings).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_lightmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_Lightmaps".to_string(), v)),
            ((self.m_light_probes).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbes".to_string(), v)),
            ((self.m_enlighten_scene_mapping).inspect_value(path, offset, limit))
                .map(|v| ("m_EnlightenSceneMapping".to_string(), v)),
            ((self.m_lightmaps_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapsMode".to_string(), v)),
            ((self.m_bake_on_scene_load).inspect_value(path, offset, limit))
                .map(|v| ("m_BakeOnSceneLoad".to_string(), v)),
            ((self.m_use_dual_lightmaps_in_forward).inspect_value(path, offset, limit))
                .map(|v| ("m_UseDualLightmapsInForward".to_string(), v)),
            ((self.m_baked_color_space).inspect_value(path, offset, limit))
                .map(|v| ("m_BakedColorSpace".to_string(), v)),
            ((self.m_gi_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_GISettings".to_string(), v)),
            ((self.m_runtime_cpu_usage).inspect_value(path, offset, limit))
                .map(|v| ("m_RuntimeCPUUsage".to_string(), v)),
            ((self.m_shadow_mask_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_ShadowMaskMode".to_string(), v)),
            ((self.m_use_shadowmask).inspect_value(path, offset, limit))
                .map(|v| ("m_UseShadowmask".to_string(), v)),
            ((self.m_lighting_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_LightingSettings".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
