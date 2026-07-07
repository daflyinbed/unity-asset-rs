//! 自动生成。请勿手动修改。

use crate::classes::vector3f::Vector3f;
use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct TreeInstance {
    pub position: Vector3f,
    pub width_scale: f32,
    pub height_scale: f32,
    pub color: ColorRgba,
    pub lightmap_color: ColorRgba,
    pub index: i32,
}

impl crate::Parse for TreeInstance {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let position = Vector3f::parse(r, ver)?;
        let width_scale = r.read_f32()?;
        let height_scale = r.read_f32()?;
        let color = ColorRgba::parse(r, ver)?;
        let lightmap_color = ColorRgba::parse(r, ver)?;
        let index = r.read_i32()?;

        Ok(TreeInstance {
            position,
            width_scale,
            height_scale,
            color,
            lightmap_color,
            index,
        })
    }
}
