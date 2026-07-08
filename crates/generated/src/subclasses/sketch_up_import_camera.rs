//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SketchUpImportCamera {
    pub position: Vector3f,
    pub look_at: Vector3f,
    pub up: Vector3f,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub ortho_size: f32,
    pub is_perspective: i32,
}

impl crate::Parse for SketchUpImportCamera {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let position = Vector3f::parse(r, ver)?;
        let look_at = Vector3f::parse(r, ver)?;
        let up = Vector3f::parse(r, ver)?;
        let fov = r.read_f32()?;
        let aspect_ratio = r.read_f32()?;
        let ortho_size = r.read_f32()?;
        let is_perspective = r.read_i32()?;
        r.align(4)?;

        Ok(SketchUpImportCamera {
            position,
            look_at,
            up,
            fov,
            aspect_ratio,
            ortho_size,
            is_perspective,
        })
    }
}
