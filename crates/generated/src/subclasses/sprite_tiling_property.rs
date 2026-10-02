//! 自动生成。请勿手动修改。

use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct SpriteTilingProperty {
    pub border: Vector4f,
    pub pivot: Vector2f,
    pub old_size: Vector2f,
    pub new_size: Vector2f,
    pub adaptive_tiling_threshold: f32,
    pub draw_mode: i32,
    pub adaptive_tiling: bool,
}

impl crate::Parse for SpriteTilingProperty {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let border = Vector4f::parse(rdr, ver)?;
        let pivot = Vector2f::parse(rdr, ver)?;
        let old_size = Vector2f::parse(rdr, ver)?;
        let new_size = Vector2f::parse(rdr, ver)?;
        let adaptive_tiling_threshold = rdr.read_f32()?;
        let draw_mode = rdr.read_i32()?;
        let adaptive_tiling = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(SpriteTilingProperty {
            border,
            pivot,
            old_size,
            new_size,
            adaptive_tiling_threshold,
            draw_mode,
            adaptive_tiling,
        })
    }
}

impl crate::ToValue for SpriteTilingProperty {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("border".to_string(), self.border.to_value())),
            Some(("pivot".to_string(), self.pivot.to_value())),
            Some(("oldSize".to_string(), self.old_size.to_value())),
            Some(("newSize".to_string(), self.new_size.to_value())),
            Some((
                "adaptiveTilingThreshold".to_string(),
                self.adaptive_tiling_threshold.to_value(),
            )),
            Some(("drawMode".to_string(), self.draw_mode.to_value())),
            Some((
                "adaptiveTiling".to_string(),
                self.adaptive_tiling.to_value(),
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
                "border" => (self.border).inspect_value(path, offset, limit),
                "pivot" => (self.pivot).inspect_value(path, offset, limit),
                "oldSize" => (self.old_size).inspect_value(path, offset, limit),
                "newSize" => (self.new_size).inspect_value(path, offset, limit),
                "adaptiveTilingThreshold" => {
                    (self.adaptive_tiling_threshold).inspect_value(path, offset, limit)
                }
                "drawMode" => (self.draw_mode).inspect_value(path, offset, limit),
                "adaptiveTiling" => (self.adaptive_tiling).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.border).inspect_value(path, offset, limit)).map(|v| ("border".to_string(), v)),
            ((self.pivot).inspect_value(path, offset, limit)).map(|v| ("pivot".to_string(), v)),
            ((self.old_size).inspect_value(path, offset, limit))
                .map(|v| ("oldSize".to_string(), v)),
            ((self.new_size).inspect_value(path, offset, limit))
                .map(|v| ("newSize".to_string(), v)),
            ((self.adaptive_tiling_threshold).inspect_value(path, offset, limit))
                .map(|v| ("adaptiveTilingThreshold".to_string(), v)),
            ((self.draw_mode).inspect_value(path, offset, limit))
                .map(|v| ("drawMode".to_string(), v)),
            ((self.adaptive_tiling).inspect_value(path, offset, limit))
                .map(|v| ("adaptiveTiling".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
