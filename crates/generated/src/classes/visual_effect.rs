//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vfx_property_sheet_serialized_base::VfxPropertySheetSerializedBase;

/// 版本枚举：VisualEffectMResetSeedOnPlayVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VisualEffectMResetSeedOnPlayVariant {
    Default(bool),
    V2019_3(u8),
    V2022_2(u8),
}

impl crate::Parse for VisualEffectMResetSeedOnPlayVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V2022_2(v))
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::V2019_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VisualEffectMResetSeedOnPlayVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VisualEffectMAllowInstancingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VisualEffectMAllowInstancingVariant {
    Default(u8),
    V6000_5(u8),
}

impl crate::Parse for VisualEffectMAllowInstancingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V6000_5(v))
        } else {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VisualEffectMAllowInstancingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct VisualEffect {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_asset: PPtr,
    pub m_start_seed: u32,
    pub m_initial_event_name: Option<String>,
    pub m_reset_seed_on_play: VisualEffectMResetSeedOnPlayVariant,
    pub m_initial_event_name_overriden: Option<u8>,
    pub m_property_sheet: VfxPropertySheetSerializedBase,
    pub m_allow_instancing: Option<VisualEffectMAllowInstancingVariant>,
    pub m_release_instance_on_disable: Option<u8>,
}

impl crate::Parse for VisualEffect {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_asset = None;
        let mut m_start_seed = None;
        let mut m_initial_event_name = None;
        let mut m_reset_seed_on_play = None;
        let mut m_initial_event_name_overriden = None;
        let mut m_property_sheet = None;
        let mut m_allow_instancing = None;
        let mut m_release_instance_on_disable = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_asset = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_initial_event_name = Some(rdr.read_aligned_string()?);
            m_initial_event_name_overriden = Some(rdr.read_u8()?);
            rdr.align(4)?;
        } else {
        }
        m_start_seed = Some(rdr.read_u32()?);
        m_reset_seed_on_play = Some(VisualEffectMResetSeedOnPlayVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_allow_instancing = Some(VisualEffectMAllowInstancingVariant::parse(rdr, ver)?);
            m_release_instance_on_disable = Some(rdr.read_u8()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_allow_instancing = Some(VisualEffectMAllowInstancingVariant::parse(rdr, ver)?);
        } else {
        }
        m_property_sheet = Some(VfxPropertySheetSerializedBase::parse(rdr, ver)?);

        Ok(VisualEffect {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_asset: m_asset.unwrap(),
            m_start_seed: m_start_seed.unwrap(),
            m_initial_event_name,
            m_reset_seed_on_play: m_reset_seed_on_play.unwrap(),
            m_initial_event_name_overriden,
            m_property_sheet: m_property_sheet.unwrap(),
            m_allow_instancing,
            m_release_instance_on_disable,
        })
    }
}

impl crate::ToValue for VisualEffect {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Asset".to_string(), self.m_asset.to_value())),
            Some(("m_StartSeed".to_string(), self.m_start_seed.to_value())),
            crate::opt_field("m_InitialEventName", &self.m_initial_event_name),
            Some((
                "m_ResetSeedOnPlay".to_string(),
                self.m_reset_seed_on_play.to_value(),
            )),
            crate::opt_field(
                "m_InitialEventNameOverriden",
                &self.m_initial_event_name_overriden,
            ),
            Some((
                "m_PropertySheet".to_string(),
                self.m_property_sheet.to_value(),
            )),
            crate::opt_field("m_AllowInstancing", &self.m_allow_instancing),
            crate::opt_field(
                "m_ReleaseInstanceOnDisable",
                &self.m_release_instance_on_disable,
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
                "m_Asset" => (self.m_asset).inspect_value(path, offset, limit),
                "m_StartSeed" => (self.m_start_seed).inspect_value(path, offset, limit),
                "m_InitialEventName" => {
                    (self.m_initial_event_name).inspect_value(path, offset, limit)
                }
                "m_ResetSeedOnPlay" => {
                    (self.m_reset_seed_on_play).inspect_value(path, offset, limit)
                }
                "m_InitialEventNameOverriden" => {
                    (self.m_initial_event_name_overriden).inspect_value(path, offset, limit)
                }
                "m_PropertySheet" => (self.m_property_sheet).inspect_value(path, offset, limit),
                "m_AllowInstancing" => (self.m_allow_instancing).inspect_value(path, offset, limit),
                "m_ReleaseInstanceOnDisable" => {
                    (self.m_release_instance_on_disable).inspect_value(path, offset, limit)
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
            ((self.m_asset).inspect_value(path, offset, limit)).map(|v| ("m_Asset".to_string(), v)),
            ((self.m_start_seed).inspect_value(path, offset, limit))
                .map(|v| ("m_StartSeed".to_string(), v)),
            ((self.m_initial_event_name).inspect_value(path, offset, limit))
                .map(|v| ("m_InitialEventName".to_string(), v)),
            ((self.m_reset_seed_on_play).inspect_value(path, offset, limit))
                .map(|v| ("m_ResetSeedOnPlay".to_string(), v)),
            ((self.m_initial_event_name_overriden).inspect_value(path, offset, limit))
                .map(|v| ("m_InitialEventNameOverriden".to_string(), v)),
            ((self.m_property_sheet).inspect_value(path, offset, limit))
                .map(|v| ("m_PropertySheet".to_string(), v)),
            ((self.m_allow_instancing).inspect_value(path, offset, limit))
                .map(|v| ("m_AllowInstancing".to_string(), v)),
            ((self.m_release_instance_on_disable).inspect_value(path, offset, limit))
                .map(|v| ("m_ReleaseInstanceOnDisable".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
