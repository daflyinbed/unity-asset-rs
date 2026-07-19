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
