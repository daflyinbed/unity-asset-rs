//! 自动生成。请勿手动修改。

use crate::subclasses::sketch_up_import_camera::SketchUpImportCamera;

#[derive(Debug, Clone)]
pub struct SketchUpImportScene {
    pub camera: SketchUpImportCamera,
    pub name: String,
}

impl crate::Parse for SketchUpImportScene {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let camera = SketchUpImportCamera::parse(rdr, ver)?;
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(SketchUpImportScene { camera, name })
    }
}

impl crate::ToValue for SketchUpImportScene {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("camera".to_string(), self.camera.to_value())),
            Some(("name".to_string(), self.name.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "camera" => (self.camera).inspect_value(path, offset, limit),
                "name" => (self.name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.camera).inspect_value(path, offset, limit)).map(|v| ("camera".to_string(), v)),
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
