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
