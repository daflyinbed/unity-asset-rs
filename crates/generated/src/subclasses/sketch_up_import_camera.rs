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

impl crate::ToValue for SketchUpImportCameraIsPerspectiveVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for SketchUpImportCamera {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("position".to_string(), self.position.to_value())),
            Some(("lookAt".to_string(), self.look_at.to_value())),
            Some(("up".to_string(), self.up.to_value())),
            Some(("fov".to_string(), self.fov.to_value())),
            Some(("aspectRatio".to_string(), self.aspect_ratio.to_value())),
            Some(("orthoSize".to_string(), self.ortho_size.to_value())),
            Some(("isPerspective".to_string(), self.is_perspective.to_value())),
            crate::opt_field("nearPlane", &self.near_plane),
            crate::opt_field("farPlane", &self.far_plane),
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
                "lookAt" => (self.look_at).inspect_value(path, offset, limit),
                "up" => (self.up).inspect_value(path, offset, limit),
                "fov" => (self.fov).inspect_value(path, offset, limit),
                "aspectRatio" => (self.aspect_ratio).inspect_value(path, offset, limit),
                "orthoSize" => (self.ortho_size).inspect_value(path, offset, limit),
                "isPerspective" => (self.is_perspective).inspect_value(path, offset, limit),
                "nearPlane" => (self.near_plane).inspect_value(path, offset, limit),
                "farPlane" => (self.far_plane).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.position).inspect_value(path, offset, limit))
                .map(|v| ("position".to_string(), v)),
            ((self.look_at).inspect_value(path, offset, limit)).map(|v| ("lookAt".to_string(), v)),
            ((self.up).inspect_value(path, offset, limit)).map(|v| ("up".to_string(), v)),
            ((self.fov).inspect_value(path, offset, limit)).map(|v| ("fov".to_string(), v)),
            ((self.aspect_ratio).inspect_value(path, offset, limit))
                .map(|v| ("aspectRatio".to_string(), v)),
            ((self.ortho_size).inspect_value(path, offset, limit))
                .map(|v| ("orthoSize".to_string(), v)),
            ((self.is_perspective).inspect_value(path, offset, limit))
                .map(|v| ("isPerspective".to_string(), v)),
            ((self.near_plane).inspect_value(path, offset, limit))
                .map(|v| ("nearPlane".to_string(), v)),
            ((self.far_plane).inspect_value(path, offset, limit))
                .map(|v| ("farPlane".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
