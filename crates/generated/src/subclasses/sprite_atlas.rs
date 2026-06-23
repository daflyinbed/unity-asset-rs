//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::sprite_atlas_editor_data::SpriteAtlasEditorData;

#[derive(Debug, Clone)]
pub struct SpriteAtlas {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_editor_data: SpriteAtlasEditorData,
    pub m_master_atlas: PPtr,
    pub m_packed_sprites: Vec<PPtr>,
    pub m_packed_sprite_names_to_index: Vec<String>,
    pub m_tag: String,
    pub m_is_variant: bool,
}

impl crate::Parse for SpriteAtlas {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_editor_data = SpriteAtlasEditorData::parse(r, ver)?;
        r.align(4)?;
        let m_master_atlas = crate::PPtr::parse(r, ver)?;
        let m_packed_sprites = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_packed_sprite_names_to_index = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_tag = r.read_aligned_string()?;
        r.align(4)?;
        let m_is_variant = r.read_bool()?;
        r.align(4)?;

        Ok(SpriteAtlas {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_editor_data,
            m_master_atlas,
            m_packed_sprites,
            m_packed_sprite_names_to_index,
            m_tag,
            m_is_variant,
        })
    }
}
