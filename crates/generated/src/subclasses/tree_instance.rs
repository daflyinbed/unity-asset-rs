//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgba32::ColorRgba32;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct TreeInstance {
    pub position: Vector3f,
    pub width_scale: f32,
    pub height_scale: f32,
    pub color: ColorRgba32,
    pub rotation: Option<f32>,
    pub lightmap_color: ColorRgba32,
    pub index: i32,
}

impl crate::Parse for TreeInstance {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut position = None;
        let mut width_scale = None;
        let mut height_scale = None;
        let mut color = None;
        let mut rotation = None;
        let mut lightmap_color = None;
        let mut index = None;
        position = Some(Vector3f::parse(rdr, ver)?);
        width_scale = Some(rdr.read_f32()?);
        height_scale = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            rotation = Some(rdr.read_f32()?);
        } else {
        }
        color = Some(ColorRgba32::parse(rdr, ver)?);
        lightmap_color = Some(ColorRgba32::parse(rdr, ver)?);
        index = Some(rdr.read_i32()?);

        Ok(TreeInstance {
            position: position.unwrap(),
            width_scale: width_scale.unwrap(),
            height_scale: height_scale.unwrap(),
            color: color.unwrap(),
            rotation,
            lightmap_color: lightmap_color.unwrap(),
            index: index.unwrap(),
        })
    }
}

impl crate::ToValue for TreeInstance {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("position".to_string(), self.position.to_value())),
            Some(("widthScale".to_string(), self.width_scale.to_value())),
            Some(("heightScale".to_string(), self.height_scale.to_value())),
            Some(("color".to_string(), self.color.to_value())),
            crate::opt_field("rotation", &self.rotation),
            Some(("lightmapColor".to_string(), self.lightmap_color.to_value())),
            Some(("index".to_string(), self.index.to_value())),
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
                "widthScale" => (self.width_scale).inspect_value(path, offset, limit),
                "heightScale" => (self.height_scale).inspect_value(path, offset, limit),
                "color" => (self.color).inspect_value(path, offset, limit),
                "rotation" => (self.rotation).inspect_value(path, offset, limit),
                "lightmapColor" => (self.lightmap_color).inspect_value(path, offset, limit),
                "index" => (self.index).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.position).inspect_value(path, offset, limit))
                .map(|v| ("position".to_string(), v)),
            ((self.width_scale).inspect_value(path, offset, limit))
                .map(|v| ("widthScale".to_string(), v)),
            ((self.height_scale).inspect_value(path, offset, limit))
                .map(|v| ("heightScale".to_string(), v)),
            ((self.color).inspect_value(path, offset, limit)).map(|v| ("color".to_string(), v)),
            ((self.rotation).inspect_value(path, offset, limit))
                .map(|v| ("rotation".to_string(), v)),
            ((self.lightmap_color).inspect_value(path, offset, limit))
                .map(|v| ("lightmapColor".to_string(), v)),
            ((self.index).inspect_value(path, offset, limit)).map(|v| ("index".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
