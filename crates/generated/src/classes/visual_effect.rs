//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vfx_property_sheet_serialized_base::VfxPropertySheetSerializedBase;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct VisualEffect {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_asset: PPtr,
    pub m_start_seed: u32,
    pub m_initial_event_name: Option<String>,
    pub m_reset_seed_on_play: bool,
    pub m_initial_event_name_overriden: Option<u8>,
    pub m_property_sheet: VfxPropertySheetSerializedBase,
    pub m_allow_instancing: Option<u8>,
    pub m_release_instance_on_disable: Option<u8>,
}

impl crate::Parse for VisualEffect {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_asset = crate::PPtr::parse(r, ver)?;
        let m_start_seed = r.read_u32()?;
        let m_initial_event_name = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_reset_seed_on_play = r.read_bool()?;
        r.align(4)?;
        let m_initial_event_name_overriden = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_property_sheet = VfxPropertySheetSerializedBase::parse(r, ver)?;
        r.align(4)?;
        let m_allow_instancing = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_release_instance_on_disable = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = r.read_u8()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(VisualEffect {
            parent,
            m_game_object,
            m_enabled,
            m_asset,
            m_start_seed,
            m_initial_event_name,
            m_reset_seed_on_play,
            m_initial_event_name_overriden,
            m_property_sheet,
            m_allow_instancing,
            m_release_instance_on_disable,
        })
    }
}
