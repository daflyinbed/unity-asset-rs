//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgba32::ColorRgba32;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SpriteBone {
    pub name: String,
    pub position: Vector3f,
    pub guid: Option<String>,
    pub rotation: Quaternionf,
    pub length: f32,
    pub parent_id: i32,
    pub color: Option<ColorRgba32>,
}

impl crate::Parse for SpriteBone {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut position = None;
        let mut guid = None;
        let mut rotation = None;
        let mut length = None;
        let mut parent_id = None;
        let mut color = None;
        name = Some(rdr.read_aligned_string()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            guid = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else {
        }
        position = Some(Vector3f::parse(rdr, ver)?);
        rotation = Some(Quaternionf::parse(rdr, ver)?);
        length = Some(rdr.read_f32()?);
        parent_id = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            color = Some(ColorRgba32::parse(rdr, ver)?);
        } else {
        }

        Ok(SpriteBone {
            name: name.unwrap(),
            position: position.unwrap(),
            guid,
            rotation: rotation.unwrap(),
            length: length.unwrap(),
            parent_id: parent_id.unwrap(),
            color,
        })
    }
}

impl crate::ToValue for SpriteBone {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("position".to_string(), self.position.to_value())),
            crate::opt_field("guid", &self.guid),
            Some(("rotation".to_string(), self.rotation.to_value())),
            Some(("length".to_string(), self.length.to_value())),
            Some(("parentId".to_string(), self.parent_id.to_value())),
            crate::opt_field("color", &self.color),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "position" => (self.position).inspect_value(path, offset, limit),
                "guid" => (self.guid).inspect_value(path, offset, limit),
                "rotation" => (self.rotation).inspect_value(path, offset, limit),
                "length" => (self.length).inspect_value(path, offset, limit),
                "parentId" => (self.parent_id).inspect_value(path, offset, limit),
                "color" => (self.color).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.position).inspect_value(path, offset, limit))
                .map(|v| ("position".to_string(), v)),
            ((self.guid).inspect_value(path, offset, limit)).map(|v| ("guid".to_string(), v)),
            ((self.rotation).inspect_value(path, offset, limit))
                .map(|v| ("rotation".to_string(), v)),
            ((self.length).inspect_value(path, offset, limit)).map(|v| ("length".to_string(), v)),
            ((self.parent_id).inspect_value(path, offset, limit))
                .map(|v| ("parentId".to_string(), v)),
            ((self.color).inspect_value(path, offset, limit)).map(|v| ("color".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
