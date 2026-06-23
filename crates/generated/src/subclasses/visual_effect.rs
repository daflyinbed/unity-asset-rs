//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vfx_property_sheet_serialized_base::VfxPropertySheetSerializedBase;

#[derive(Debug, Clone)]
pub struct VisualEffect {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_asset: PPtr,
    pub m_start_seed: u32,
    pub m_reset_seed_on_play: bool,
    pub m_property_sheet: VfxPropertySheetSerializedBase,
}

impl crate::Parse for VisualEffect {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_asset = crate::PPtr::parse(r, ver)?;
        let m_start_seed = r.read_u32()?;
        let m_reset_seed_on_play = r.read_bool()?;
        r.align(4)?;
        let m_property_sheet = VfxPropertySheetSerializedBase::parse(r, ver)?;
        r.align(4)?;

        Ok(VisualEffect {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_game_object,
            m_enabled,
            m_asset,
            m_start_seed,
            m_reset_seed_on_play,
            m_property_sheet,
        })
    }
}
