//! 自动生成。请勿手动修改。

use crate::subclasses::sketch_up_import_camera::SketchUpImportCamera;
use crate::subclasses::sketch_up_import_scene::SketchUpImportScene;

/// 版本枚举：SketchUpImportDataScenesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SketchUpImportDataScenesVariant {
    Default(Vec<SketchUpImportScene>),
    V2017_1(Vec<SketchUpImportScene>),
}

impl crate::Parse for SketchUpImportDataScenesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SketchUpImportScene::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SketchUpImportScene::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SketchUpImportDataScenesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SketchUpImportData {
    pub default_camera: SketchUpImportCamera,
    pub scenes: SketchUpImportDataScenesVariant,
}

impl crate::Parse for SketchUpImportData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let default_camera = SketchUpImportCamera::parse(rdr, ver)?;
        let scenes = SketchUpImportDataScenesVariant::parse(rdr, ver)?;

        Ok(SketchUpImportData {
            default_camera,
            scenes,
        })
    }
}

impl crate::ToValue for SketchUpImportData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("defaultCamera".to_string(), self.default_camera.to_value())),
            Some(("scenes".to_string(), self.scenes.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "defaultCamera" => (self.default_camera).inspect_value(path, offset, limit),
                "scenes" => (self.scenes).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.default_camera).inspect_value(path, offset, limit))
                .map(|v| ("defaultCamera".to_string(), v)),
            ((self.scenes).inspect_value(path, offset, limit)).map(|v| ("scenes".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
