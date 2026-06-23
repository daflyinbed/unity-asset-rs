//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::int3_storage::int3_storage;
use crate::subclasses::matrix4x4f::Matrix4x4f;
use crate::subclasses::tile::Tile;
use crate::subclasses::tile_animation_data::TileAnimationData;
use crate::subclasses::tilemap_ref_counted_data::TilemapRefCountedData;
use crate::subclasses::tilemap_tile_dirty_data::TilemapTileDirtyData;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct Tilemap {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_tiles: Vec<(int3_storage, Tile)>,
    pub m_animated_tiles: Vec<(int3_storage, TileAnimationData)>,
    pub m_tile_asset_array: Vec<TilemapRefCountedData>,
    pub m_tile_sprite_array: Vec<TilemapRefCountedData>,
    pub m_tile_matrix_array: Vec<TilemapRefCountedData>,
    pub m_tile_color_array: Vec<TilemapRefCountedData>,
    pub m_animation_frame_rate: f32,
    pub m_color: ColorRgba,
    pub m_origin: int3_storage,
    pub m_size: int3_storage,
    pub m_tile_anchor: Vector3f,
    pub m_tile_orientation: i32,
    pub m_tile_orientation_matrix: Matrix4x4f,
    pub m_tile_refresh_array: Vec<TilemapTileDirtyData>,
}

impl crate::Parse for Tilemap {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_tiles = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = int3_storage::parse(r, ver)?; let b = Tile::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_animated_tiles = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = int3_storage::parse(r, ver)?; let b = TileAnimationData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_tile_asset_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TilemapRefCountedData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_tile_sprite_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TilemapRefCountedData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_tile_matrix_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TilemapRefCountedData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_tile_color_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TilemapRefCountedData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_animation_frame_rate = r.read_f32()?;
        let m_color = ColorRgba::parse(r, ver)?;
        let m_origin = int3_storage::parse(r, ver)?;
        let m_size = int3_storage::parse(r, ver)?;
        let m_tile_anchor = Vector3f::parse(r, ver)?;
        let m_tile_orientation = r.read_i32()?;
        r.align(4)?;
        let m_tile_orientation_matrix = Matrix4x4f::parse(r, ver)?;
        let m_tile_refresh_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TilemapTileDirtyData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(Tilemap {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_game_object,
            m_enabled,
            m_tiles,
            m_animated_tiles,
            m_tile_asset_array,
            m_tile_sprite_array,
            m_tile_matrix_array,
            m_tile_color_array,
            m_animation_frame_rate,
            m_color,
            m_origin,
            m_size,
            m_tile_anchor,
            m_tile_orientation,
            m_tile_orientation_matrix,
            m_tile_refresh_array,
        })
    }
}
