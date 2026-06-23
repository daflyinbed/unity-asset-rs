//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::packing_parameters::PackingParameters;
use crate::subclasses::texture_settings::TextureSettings;

#[derive(Debug, Clone)]
pub struct SpriteAtlasEditorData {
    pub texture_settings: TextureSettings,
    pub packing_parameters: PackingParameters,
    pub packed_hash: Hash128,
    pub variant_multiplier: f32,
    pub bind_as_default: bool,
    pub hash_string: String,
    pub total_sprite_surface_area: u32,
    pub packables: Vec<PPtr>,
    pub packed_sprite_render_data_keys: Vec<(Guid, i64)>,
}

impl crate::Parse for SpriteAtlasEditorData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let texture_settings = TextureSettings::parse(r, ver)?;
        r.align(4)?;
        let packing_parameters = PackingParameters::parse(r, ver)?;
        let packed_hash = Hash128::parse(r, ver)?;
        let variant_multiplier = r.read_f32()?;
        let bind_as_default = r.read_bool()?;
        r.align(4)?;
        let hash_string = r.read_aligned_string()?;
        r.align(4)?;
        let total_sprite_surface_area = r.read_u32()?;
        let packables = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let packed_sprite_render_data_keys = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(r, ver)?; let b = r.read_i64()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(SpriteAtlasEditorData {
            texture_settings,
            packing_parameters,
            packed_hash,
            variant_multiplier,
            bind_as_default,
            hash_string,
            total_sprite_surface_area,
            packables,
            packed_sprite_render_data_keys,
        })
    }
}
