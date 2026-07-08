//! 自动生成。请勿手动修改。

use crate::subclasses::sketch_up_import_camera::SketchUpImportCamera;
use crate::subclasses::sketch_up_import_scene::SketchUpImportScene;

#[derive(Debug, Clone)]
pub struct SketchUpImportData {
    pub default_camera: SketchUpImportCamera,
    pub scenes: Vec<SketchUpImportScene>,
}

impl crate::Parse for SketchUpImportData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let default_camera = SketchUpImportCamera::parse(rdr, ver)?;
        rdr.align(4)?;
        let scenes = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SketchUpImportScene::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(SketchUpImportData {
            default_camera,
            scenes,
        })
    }
}
