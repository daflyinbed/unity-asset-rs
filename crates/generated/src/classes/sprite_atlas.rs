//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::sprite_atlas_data::SpriteAtlasData;


/// 版本枚举：SpriteAtlasMRenderDataMapVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteAtlasMRenderDataMapVariant {
    Default(Vec<((Guid, i64), SpriteAtlasData)>),
    V2020_2(Vec<((Guid, i64), SpriteAtlasData)>),
}

impl crate::Parse for SpriteAtlasMRenderDataMapVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = Guid::parse(rdr, ver)?; let b = rdr.read_i64()?; Ok((a, b)) } )?; let b = SpriteAtlasData::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2020_2(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = Guid::parse(rdr, ver)?; let b = rdr.read_i64()?; Ok((a, b)) } )?; let b = SpriteAtlasData::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SpriteAtlas {
    pub m_name: String,
    pub m_packed_sprites: Vec<PPtr>,
    pub m_packed_sprite_names_to_index: Vec<String>,
    pub m_render_data_map: SpriteAtlasMRenderDataMapVariant,
    pub m_tag: String,
    pub m_is_variant: bool,
    pub m_guid: Option<Guid>,
}

impl crate::Parse for SpriteAtlas {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_packed_sprites = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_packed_sprite_names_to_index = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_render_data_map = SpriteAtlasMRenderDataMapVariant::parse(rdr, ver)?;
        let m_tag = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_is_variant = rdr.read_bool()?;
        rdr.align(4)?;
        let m_guid = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = Guid::parse(rdr, ver)?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SpriteAtlas {
            m_name,
            m_packed_sprites,
            m_packed_sprite_names_to_index,
            m_render_data_map,
            m_tag,
            m_is_variant,
            m_guid,
        })
    }
}
