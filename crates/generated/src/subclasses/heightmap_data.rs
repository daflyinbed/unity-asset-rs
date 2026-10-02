//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::matrix4x4f::Matrix4x4f;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct HeightmapData {
    pub position: Option<Vector3f>,
    pub surface_to_terrain: Option<Matrix4x4f>,
    pub terrain_data: PPtr,
    pub is_rotated: Option<bool>,
}

impl crate::Parse for HeightmapData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut position = None;
        let mut surface_to_terrain = None;
        let mut terrain_data = None;
        let mut is_rotated = None;
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            surface_to_terrain = Some(Matrix4x4f::parse(rdr, ver)?);
        } else {
            position = Some(Vector3f::parse(rdr, ver)?);
        }
        terrain_data = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            is_rotated = Some(rdr.read_bool()?);
        } else {
        }

        Ok(HeightmapData {
            position,
            surface_to_terrain,
            terrain_data: terrain_data.unwrap(),
            is_rotated,
        })
    }
}

impl crate::ToValue for HeightmapData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("position", &self.position),
            crate::opt_field("surfaceToTerrain", &self.surface_to_terrain),
            Some(("terrainData".to_string(), self.terrain_data.to_value())),
            crate::opt_field("isRotated", &self.is_rotated),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "position" => (self.position).inspect_value(path, offset, limit),
                "surfaceToTerrain" => (self.surface_to_terrain).inspect_value(path, offset, limit),
                "terrainData" => (self.terrain_data).inspect_value(path, offset, limit),
                "isRotated" => (self.is_rotated).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.position).inspect_value(path, offset, limit))
                .map(|v| ("position".to_string(), v)),
            ((self.surface_to_terrain).inspect_value(path, offset, limit))
                .map(|v| ("surfaceToTerrain".to_string(), v)),
            ((self.terrain_data).inspect_value(path, offset, limit))
                .map(|v| ("terrainData".to_string(), v)),
            ((self.is_rotated).inspect_value(path, offset, limit))
                .map(|v| ("isRotated".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
