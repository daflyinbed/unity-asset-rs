//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::sprite_atlas_editor_data::SpriteAtlasEditorData;

#[derive(Debug, Clone)]
pub struct SpriteAtlasAsset {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_name: String,
    pub m_master_atlas: PPtr,
    pub m_importer_data: SpriteAtlasEditorData,
    pub m_is_variant: bool,
}

impl crate::Parse for SpriteAtlasAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_master_atlas = crate::PPtr::parse(r, ver)?;
        let m_importer_data = SpriteAtlasEditorData::parse(r, ver)?;
        r.align(4)?;
        let m_is_variant = r.read_bool()?;
        r.align(4)?;

        Ok(SpriteAtlasAsset {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_name,
            m_master_atlas,
            m_importer_data,
            m_is_variant,
        })
    }
}
