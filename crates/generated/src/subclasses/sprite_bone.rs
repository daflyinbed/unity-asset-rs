//! 自动生成。请勿手动修改。

use crate::classes::vector3f::Vector3f;
use crate::subclasses::quaternionf::Quaternionf;

#[derive(Debug, Clone)]
pub struct SpriteBone {
    pub name: String,
    pub position: Vector3f,
    pub rotation: Quaternionf,
    pub length: f32,
    pub parent_id: i32,
}

impl crate::Parse for SpriteBone {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let position = Vector3f::parse(r, ver)?;
        let rotation = Quaternionf::parse(r, ver)?;
        let length = r.read_f32()?;
        let parent_id = r.read_i32()?;

        Ok(SpriteBone {
            name,
            position,
            rotation,
            length,
            parent_id,
        })
    }
}
