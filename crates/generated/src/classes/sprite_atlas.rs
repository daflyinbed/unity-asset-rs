//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::sprite_atlas_data::SpriteAtlasData;

#[derive(Debug, Clone)]
pub struct SpriteAtlas {
    pub m_name: String,
    pub m_packed_sprites: Vec<PPtr>,
    pub m_packed_sprite_names_to_index: Vec<String>,
    pub m_render_data_map: Vec<((Guid, i64), SpriteAtlasData)>,
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
        let m_packed_sprites = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_packed_sprite_names_to_index = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_render_data_map = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = ({
                            let a = Guid::parse(rdr, ver)?;
                            let b = rdr.read_i64()?;
                            Ok((a, b))
                        })?;
                        let b = SpriteAtlasData::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_tag = rdr.read_aligned_string()?;
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

impl crate::ToValue for SpriteAtlas {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_PackedSprites".to_string(),
                self.m_packed_sprites.to_value(),
            )),
            Some((
                "m_PackedSpriteNamesToIndex".to_string(),
                self.m_packed_sprite_names_to_index.to_value(),
            )),
            Some((
                "m_RenderDataMap".to_string(),
                self.m_render_data_map.to_value(),
            )),
            Some(("m_Tag".to_string(), self.m_tag.to_value())),
            Some(("m_IsVariant".to_string(), self.m_is_variant.to_value())),
            crate::opt_field("m_Guid", &self.m_guid),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_PackedSprites" => (self.m_packed_sprites).inspect_value(path, offset, limit),
                "m_PackedSpriteNamesToIndex" => {
                    (self.m_packed_sprite_names_to_index).inspect_value(path, offset, limit)
                }
                "m_RenderDataMap" => (self.m_render_data_map).inspect_value(path, offset, limit),
                "m_Tag" => (self.m_tag).inspect_value(path, offset, limit),
                "m_IsVariant" => (self.m_is_variant).inspect_value(path, offset, limit),
                "m_Guid" => (self.m_guid).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_packed_sprites).inspect_value(path, offset, limit))
                .map(|v| ("m_PackedSprites".to_string(), v)),
            ((self.m_packed_sprite_names_to_index).inspect_value(path, offset, limit))
                .map(|v| ("m_PackedSpriteNamesToIndex".to_string(), v)),
            ((self.m_render_data_map).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderDataMap".to_string(), v)),
            ((self.m_tag).inspect_value(path, offset, limit)).map(|v| ("m_Tag".to_string(), v)),
            ((self.m_is_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_IsVariant".to_string(), v)),
            ((self.m_guid).inspect_value(path, offset, limit)).map(|v| ("m_Guid".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
