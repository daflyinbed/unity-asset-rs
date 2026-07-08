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
