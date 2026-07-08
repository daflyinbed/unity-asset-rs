//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;


/// 版本枚举：SketchUpImportCameraIsPerspectiveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImportCameraIsPerspectiveVariant {
    Default(i32),
    V2019_1(i32),
}

impl crate::Parse for SketchUpImportCameraIsPerspectiveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V2019_1(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SketchUpImportCamera {
    pub position: Vector3f,
    pub look_at: Vector3f,
    pub up: Vector3f,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub ortho_size: f32,
    pub is_perspective: SketchUpImportCameraIsPerspectiveVariant,
    pub near_plane: Option<f32>,
    pub far_plane: Option<f32>,
}

impl crate::Parse for SketchUpImportCamera {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let position = Vector3f::parse(rdr, ver)?;
        let look_at = Vector3f::parse(rdr, ver)?;
        let up = Vector3f::parse(rdr, ver)?;
        let fov = rdr.read_f32()?;
        let aspect_ratio = rdr.read_f32()?;
        let ortho_size = rdr.read_f32()?;
        let is_perspective = SketchUpImportCameraIsPerspectiveVariant::parse(rdr, ver)?;
        let near_plane = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let far_plane = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_f32()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SketchUpImportCamera {
            position,
            look_at,
            up,
            fov,
            aspect_ratio,
            ortho_size,
            is_perspective,
            near_plane,
            far_plane,
        })
    }
}
