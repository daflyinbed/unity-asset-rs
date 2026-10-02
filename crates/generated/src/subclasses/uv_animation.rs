//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct UvAnimation {
    pub x_tile: i32,
    pub y_tile: i32,
    pub cycles: f32,
}

impl crate::Parse for UvAnimation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let x_tile = rdr.read_i32()?;
        let y_tile = rdr.read_i32()?;
        let cycles = rdr.read_f32()?;

        Ok(UvAnimation {
            x_tile,
            y_tile,
            cycles,
        })
    }
}

impl crate::ToValue for UvAnimation {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("x Tile".to_string(), self.x_tile.to_value())),
            Some(("y Tile".to_string(), self.y_tile.to_value())),
            Some(("cycles".to_string(), self.cycles.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "x Tile" => (self.x_tile).inspect_value(path, offset, limit),
                "y Tile" => (self.y_tile).inspect_value(path, offset, limit),
                "cycles" => (self.cycles).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.x_tile).inspect_value(path, offset, limit)).map(|v| ("x Tile".to_string(), v)),
            ((self.y_tile).inspect_value(path, offset, limit)).map(|v| ("y Tile".to_string(), v)),
            ((self.cycles).inspect_value(path, offset, limit)).map(|v| ("cycles".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
