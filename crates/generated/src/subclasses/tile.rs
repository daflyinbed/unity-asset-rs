//! 自动生成。请勿手动修改。

use crate::PPtr;

/// 版本枚举：TileMTileMatrixIndexVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TileMTileMatrixIndexVariant {
    Default(u32),
    V2019_3(u16),
}

impl crate::Parse for TileMTileMatrixIndexVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = rdr.read_u16()?;
            Ok(Self::V2019_3(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TileMTileMatrixIndexVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：TileMTileColorIndexVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TileMTileColorIndexVariant {
    Default(u32),
    V2019_3(u16),
}

impl crate::Parse for TileMTileColorIndexVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = rdr.read_u16()?;
            Ok(Self::V2019_3(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TileMTileColorIndexVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：TileMTileObjectToInstantiateIndexVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TileMTileObjectToInstantiateIndexVariant {
    Default(u16),
    V2019_4_24(u16),
    V2020_1(u16),
    V2020_1_8(u16),
}

impl crate::Parse for TileMTileObjectToInstantiateIndexVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 8) {
            let v = rdr.read_u16()?;
            Ok(Self::V2020_1_8(v))
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u16()?;
            rdr.align(4)?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 4, 24) {
            let v = rdr.read_u16()?;
            Ok(Self::V2019_4_24(v))
        } else {
            let v = rdr.read_u16()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TileMTileObjectToInstantiateIndexVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_4_24(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
            Self::V2020_1_8(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_4_24(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1_8(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Tile {
    pub m_tile_index: u32,
    pub m_tile_sprite_index: u32,
    pub m_tile_matrix_index: TileMTileMatrixIndexVariant,
    pub m_tile_color_index: TileMTileColorIndexVariant,
    pub m_object_to_instantiate: Option<PPtr>,
    pub m_tile_object_to_instantiate_index: Option<TileMTileObjectToInstantiateIndexVariant>,
    pub m_tile_flags: Option<i32>,
    pub m_all_tile_flags: Option<u32>,
    pub dummy_alignment: Option<u16>,
    pub m_collider_type: Option<i32>,
}

impl crate::Parse for Tile {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_tile_index = None;
        let mut m_tile_sprite_index = None;
        let mut m_tile_matrix_index = None;
        let mut m_tile_color_index = None;
        let mut m_object_to_instantiate = None;
        let mut m_tile_object_to_instantiate_index = None;
        let mut m_tile_flags = None;
        let mut m_all_tile_flags = None;
        let mut dummy_alignment = None;
        let mut m_collider_type = None;
        m_tile_index = Some(rdr.read_u32()?);
        m_tile_sprite_index = Some(rdr.read_u32()?);
        m_tile_matrix_index = Some(TileMTileMatrixIndexVariant::parse(rdr, ver)?);
        m_tile_color_index = Some(TileMTileColorIndexVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 8) {
            m_tile_object_to_instantiate_index =
                Some(TileMTileObjectToInstantiateIndexVariant::parse(rdr, ver)?);
            dummy_alignment = Some(rdr.read_u16()?);
            m_all_tile_flags = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_tile_object_to_instantiate_index =
                Some(TileMTileObjectToInstantiateIndexVariant::parse(rdr, ver)?);
            m_all_tile_flags = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 4, 24) {
            m_tile_object_to_instantiate_index =
                Some(TileMTileObjectToInstantiateIndexVariant::parse(rdr, ver)?);
            dummy_alignment = Some(rdr.read_u16()?);
            m_all_tile_flags = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_tile_object_to_instantiate_index =
                Some(TileMTileObjectToInstantiateIndexVariant::parse(rdr, ver)?);
            m_all_tile_flags = Some(rdr.read_u32()?);
        } else {
            m_object_to_instantiate = Some(crate::PPtr::parse(rdr, ver)?);
            m_tile_flags = Some(rdr.read_i32()?);
            m_collider_type = Some(rdr.read_i32()?);
        }

        Ok(Tile {
            m_tile_index: m_tile_index.unwrap(),
            m_tile_sprite_index: m_tile_sprite_index.unwrap(),
            m_tile_matrix_index: m_tile_matrix_index.unwrap(),
            m_tile_color_index: m_tile_color_index.unwrap(),
            m_object_to_instantiate,
            m_tile_object_to_instantiate_index,
            m_tile_flags,
            m_all_tile_flags,
            dummy_alignment,
            m_collider_type,
        })
    }
}

impl crate::ToValue for Tile {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_TileIndex".to_string(), self.m_tile_index.to_value())),
            Some((
                "m_TileSpriteIndex".to_string(),
                self.m_tile_sprite_index.to_value(),
            )),
            Some((
                "m_TileMatrixIndex".to_string(),
                self.m_tile_matrix_index.to_value(),
            )),
            Some((
                "m_TileColorIndex".to_string(),
                self.m_tile_color_index.to_value(),
            )),
            crate::opt_field("m_ObjectToInstantiate", &self.m_object_to_instantiate),
            crate::opt_field(
                "m_TileObjectToInstantiateIndex",
                &self.m_tile_object_to_instantiate_index,
            ),
            crate::opt_field("m_TileFlags", &self.m_tile_flags),
            crate::opt_field("m_AllTileFlags", &self.m_all_tile_flags),
            crate::opt_field("dummyAlignment", &self.dummy_alignment),
            crate::opt_field("m_ColliderType", &self.m_collider_type),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_TileIndex" => (self.m_tile_index).inspect_value(path, offset, limit),
                "m_TileSpriteIndex" => {
                    (self.m_tile_sprite_index).inspect_value(path, offset, limit)
                }
                "m_TileMatrixIndex" => {
                    (self.m_tile_matrix_index).inspect_value(path, offset, limit)
                }
                "m_TileColorIndex" => (self.m_tile_color_index).inspect_value(path, offset, limit),
                "m_ObjectToInstantiate" => {
                    (self.m_object_to_instantiate).inspect_value(path, offset, limit)
                }
                "m_TileObjectToInstantiateIndex" => {
                    (self.m_tile_object_to_instantiate_index).inspect_value(path, offset, limit)
                }
                "m_TileFlags" => (self.m_tile_flags).inspect_value(path, offset, limit),
                "m_AllTileFlags" => (self.m_all_tile_flags).inspect_value(path, offset, limit),
                "dummyAlignment" => (self.dummy_alignment).inspect_value(path, offset, limit),
                "m_ColliderType" => (self.m_collider_type).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_tile_index).inspect_value(path, offset, limit))
                .map(|v| ("m_TileIndex".to_string(), v)),
            ((self.m_tile_sprite_index).inspect_value(path, offset, limit))
                .map(|v| ("m_TileSpriteIndex".to_string(), v)),
            ((self.m_tile_matrix_index).inspect_value(path, offset, limit))
                .map(|v| ("m_TileMatrixIndex".to_string(), v)),
            ((self.m_tile_color_index).inspect_value(path, offset, limit))
                .map(|v| ("m_TileColorIndex".to_string(), v)),
            ((self.m_object_to_instantiate).inspect_value(path, offset, limit))
                .map(|v| ("m_ObjectToInstantiate".to_string(), v)),
            ((self.m_tile_object_to_instantiate_index).inspect_value(path, offset, limit))
                .map(|v| ("m_TileObjectToInstantiateIndex".to_string(), v)),
            ((self.m_tile_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_TileFlags".to_string(), v)),
            ((self.m_all_tile_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_AllTileFlags".to_string(), v)),
            ((self.dummy_alignment).inspect_value(path, offset, limit))
                .map(|v| ("dummyAlignment".to_string(), v)),
            ((self.m_collider_type).inspect_value(path, offset, limit))
                .map(|v| ("m_ColliderType".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
