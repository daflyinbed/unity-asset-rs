//! 自动生成。请勿手动修改。

use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SpriteVertex {
    pub pos: Vector3f,
    pub uv: Option<Vector2f>,
}

impl crate::Parse for SpriteVertex {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut pos = None;
        let mut uv = None;
        pos = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
        } else {
            uv = Some(Vector2f::parse(rdr, ver)?);
        }

        Ok(SpriteVertex {
            pos: pos.unwrap(),
            uv,
        })
    }
}

impl crate::ToValue for SpriteVertex {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("pos".to_string(), self.pos.to_value())),
            crate::opt_field("uv", &self.uv),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "pos" => (self.pos).inspect_value(path, offset, limit),
                "uv" => (self.uv).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.pos).inspect_value(path, offset, limit)).map(|v| ("pos".to_string(), v)),
            ((self.uv).inspect_value(path, offset, limit)).map(|v| ("uv".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
