//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::int3_storage::int3_storage;
use crate::subclasses::matrix4x4f::Matrix4x4f;
use crate::subclasses::tile::Tile;
use crate::subclasses::tile_animation_data::TileAnimationData;
use crate::subclasses::tilemap_ref_counted_data_color_rgb_af::TilemapRefCountedData_ColorRgbAf;
use crate::subclasses::tilemap_ref_counted_data_matrix4x4f::TilemapRefCountedData_Matrix4x4f;
use crate::subclasses::tilemap_ref_counted_data_p_ptr_game_object_::TilemapRefCountedData_PPtr_GameObject_;
use crate::subclasses::tilemap_ref_counted_data_p_ptr_object_::TilemapRefCountedData_PPtr_Object_;
use crate::subclasses::tilemap_ref_counted_data_p_ptr_sprite_::TilemapRefCountedData_PPtr_Sprite_;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct Tilemap {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_tiles: Vec<(int3_storage, Tile)>,
    pub m_animated_tiles: Vec<(int3_storage, TileAnimationData)>,
    pub m_tile_asset_array: Vec<TilemapRefCountedData_PPtr_Object_>,
    pub m_tile_sprite_array: Vec<TilemapRefCountedData_PPtr_Sprite_>,
    pub m_tile_matrix_array: Vec<TilemapRefCountedData_Matrix4x4f>,
    pub m_tile_color_array: Vec<TilemapRefCountedData_ColorRgbAf>,
    pub m_animation_frame_rate: f32,
    pub m_tile_object_to_instantiate_array: Option<Vec<TilemapRefCountedData_PPtr_GameObject_>>,
    pub m_color: ColorRgbAf,
    pub m_origin: int3_storage,
    pub m_size: int3_storage,
    pub m_tile_anchor: Vector3f,
    pub m_tile_orientation: i32,
    pub m_tile_orientation_matrix: Matrix4x4f,
}

impl crate::Parse for Tilemap {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_tiles = None;
        let mut m_animated_tiles = None;
        let mut m_tile_asset_array = None;
        let mut m_tile_sprite_array = None;
        let mut m_tile_matrix_array = None;
        let mut m_tile_color_array = None;
        let mut m_animation_frame_rate = None;
        let mut m_tile_object_to_instantiate_array = None;
        let mut m_color = None;
        let mut m_origin = None;
        let mut m_size = None;
        let mut m_tile_anchor = None;
        let mut m_tile_orientation = None;
        let mut m_tile_orientation_matrix = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_tiles = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = int3_storage::parse(rdr, ver)?;
                            let b = Tile::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        m_animated_tiles = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = int3_storage::parse(rdr, ver)?;
                            let b = TileAnimationData::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        m_tile_asset_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TilemapRefCountedData_PPtr_Object_::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_tile_sprite_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TilemapRefCountedData_PPtr_Sprite_::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_tile_matrix_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TilemapRefCountedData_Matrix4x4f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_tile_color_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TilemapRefCountedData_ColorRgbAf::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_tile_object_to_instantiate_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| TilemapRefCountedData_PPtr_GameObject_::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }
        m_animation_frame_rate = Some(rdr.read_f32()?);
        m_color = Some(ColorRgbAf::parse(rdr, ver)?);
        m_origin = Some(int3_storage::parse(rdr, ver)?);
        m_size = Some(int3_storage::parse(rdr, ver)?);
        m_tile_anchor = Some(Vector3f::parse(rdr, ver)?);
        m_tile_orientation = Some(rdr.read_i32()?);
        rdr.align(4)?;
        m_tile_orientation_matrix = Some(Matrix4x4f::parse(rdr, ver)?);

        Ok(Tilemap {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_tiles: m_tiles.unwrap(),
            m_animated_tiles: m_animated_tiles.unwrap(),
            m_tile_asset_array: m_tile_asset_array.unwrap(),
            m_tile_sprite_array: m_tile_sprite_array.unwrap(),
            m_tile_matrix_array: m_tile_matrix_array.unwrap(),
            m_tile_color_array: m_tile_color_array.unwrap(),
            m_animation_frame_rate: m_animation_frame_rate.unwrap(),
            m_tile_object_to_instantiate_array,
            m_color: m_color.unwrap(),
            m_origin: m_origin.unwrap(),
            m_size: m_size.unwrap(),
            m_tile_anchor: m_tile_anchor.unwrap(),
            m_tile_orientation: m_tile_orientation.unwrap(),
            m_tile_orientation_matrix: m_tile_orientation_matrix.unwrap(),
        })
    }
}

impl crate::ToValue for Tilemap {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Tiles".to_string(), self.m_tiles.to_value())),
            Some((
                "m_AnimatedTiles".to_string(),
                self.m_animated_tiles.to_value(),
            )),
            Some((
                "m_TileAssetArray".to_string(),
                self.m_tile_asset_array.to_value(),
            )),
            Some((
                "m_TileSpriteArray".to_string(),
                self.m_tile_sprite_array.to_value(),
            )),
            Some((
                "m_TileMatrixArray".to_string(),
                self.m_tile_matrix_array.to_value(),
            )),
            Some((
                "m_TileColorArray".to_string(),
                self.m_tile_color_array.to_value(),
            )),
            Some((
                "m_AnimationFrameRate".to_string(),
                self.m_animation_frame_rate.to_value(),
            )),
            crate::opt_field(
                "m_TileObjectToInstantiateArray",
                &self.m_tile_object_to_instantiate_array,
            ),
            Some(("m_Color".to_string(), self.m_color.to_value())),
            Some(("m_Origin".to_string(), self.m_origin.to_value())),
            Some(("m_Size".to_string(), self.m_size.to_value())),
            Some(("m_TileAnchor".to_string(), self.m_tile_anchor.to_value())),
            Some((
                "m_TileOrientation".to_string(),
                self.m_tile_orientation.to_value(),
            )),
            Some((
                "m_TileOrientationMatrix".to_string(),
                self.m_tile_orientation_matrix.to_value(),
            )),
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
                "m_Tiles" => (self.m_tiles).inspect_value(path, offset, limit),
                "m_AnimatedTiles" => (self.m_animated_tiles).inspect_value(path, offset, limit),
                "m_TileAssetArray" => (self.m_tile_asset_array).inspect_value(path, offset, limit),
                "m_TileSpriteArray" => {
                    (self.m_tile_sprite_array).inspect_value(path, offset, limit)
                }
                "m_TileMatrixArray" => {
                    (self.m_tile_matrix_array).inspect_value(path, offset, limit)
                }
                "m_TileColorArray" => (self.m_tile_color_array).inspect_value(path, offset, limit),
                "m_AnimationFrameRate" => {
                    (self.m_animation_frame_rate).inspect_value(path, offset, limit)
                }
                "m_TileObjectToInstantiateArray" => {
                    (self.m_tile_object_to_instantiate_array).inspect_value(path, offset, limit)
                }
                "m_Color" => (self.m_color).inspect_value(path, offset, limit),
                "m_Origin" => (self.m_origin).inspect_value(path, offset, limit),
                "m_Size" => (self.m_size).inspect_value(path, offset, limit),
                "m_TileAnchor" => (self.m_tile_anchor).inspect_value(path, offset, limit),
                "m_TileOrientation" => (self.m_tile_orientation).inspect_value(path, offset, limit),
                "m_TileOrientationMatrix" => {
                    (self.m_tile_orientation_matrix).inspect_value(path, offset, limit)
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
            ((self.m_tiles).inspect_value(path, offset, limit)).map(|v| ("m_Tiles".to_string(), v)),
            ((self.m_animated_tiles).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimatedTiles".to_string(), v)),
            ((self.m_tile_asset_array).inspect_value(path, offset, limit))
                .map(|v| ("m_TileAssetArray".to_string(), v)),
            ((self.m_tile_sprite_array).inspect_value(path, offset, limit))
                .map(|v| ("m_TileSpriteArray".to_string(), v)),
            ((self.m_tile_matrix_array).inspect_value(path, offset, limit))
                .map(|v| ("m_TileMatrixArray".to_string(), v)),
            ((self.m_tile_color_array).inspect_value(path, offset, limit))
                .map(|v| ("m_TileColorArray".to_string(), v)),
            ((self.m_animation_frame_rate).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationFrameRate".to_string(), v)),
            ((self.m_tile_object_to_instantiate_array).inspect_value(path, offset, limit))
                .map(|v| ("m_TileObjectToInstantiateArray".to_string(), v)),
            ((self.m_color).inspect_value(path, offset, limit)).map(|v| ("m_Color".to_string(), v)),
            ((self.m_origin).inspect_value(path, offset, limit))
                .map(|v| ("m_Origin".to_string(), v)),
            ((self.m_size).inspect_value(path, offset, limit)).map(|v| ("m_Size".to_string(), v)),
            ((self.m_tile_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_TileAnchor".to_string(), v)),
            ((self.m_tile_orientation).inspect_value(path, offset, limit))
                .map(|v| ("m_TileOrientation".to_string(), v)),
            ((self.m_tile_orientation_matrix).inspect_value(path, offset, limit))
                .map(|v| ("m_TileOrientationMatrix".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
