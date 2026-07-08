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
