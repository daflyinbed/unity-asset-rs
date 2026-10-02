//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct NavMeshParams {
    pub tile_size: f32,
    pub walkable_height: f32,
    pub walkable_radius: f32,
    pub walkable_climb: f32,
    pub cell_size: f32,
}

impl crate::Parse for NavMeshParams {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let tile_size = rdr.read_f32()?;
        let walkable_height = rdr.read_f32()?;
        let walkable_radius = rdr.read_f32()?;
        let walkable_climb = rdr.read_f32()?;
        let cell_size = rdr.read_f32()?;

        Ok(NavMeshParams {
            tile_size,
            walkable_height,
            walkable_radius,
            walkable_climb,
            cell_size,
        })
    }
}

impl crate::ToValue for NavMeshParams {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("tileSize".to_string(), self.tile_size.to_value())),
            Some((
                "walkableHeight".to_string(),
                self.walkable_height.to_value(),
            )),
            Some((
                "walkableRadius".to_string(),
                self.walkable_radius.to_value(),
            )),
            Some(("walkableClimb".to_string(), self.walkable_climb.to_value())),
            Some(("cellSize".to_string(), self.cell_size.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "tileSize" => (self.tile_size).inspect_value(path, offset, limit),
                "walkableHeight" => (self.walkable_height).inspect_value(path, offset, limit),
                "walkableRadius" => (self.walkable_radius).inspect_value(path, offset, limit),
                "walkableClimb" => (self.walkable_climb).inspect_value(path, offset, limit),
                "cellSize" => (self.cell_size).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.tile_size).inspect_value(path, offset, limit))
                .map(|v| ("tileSize".to_string(), v)),
            ((self.walkable_height).inspect_value(path, offset, limit))
                .map(|v| ("walkableHeight".to_string(), v)),
            ((self.walkable_radius).inspect_value(path, offset, limit))
                .map(|v| ("walkableRadius".to_string(), v)),
            ((self.walkable_climb).inspect_value(path, offset, limit))
                .map(|v| ("walkableClimb".to_string(), v)),
            ((self.cell_size).inspect_value(path, offset, limit))
                .map(|v| ("cellSize".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
