//! 自动生成。请勿手动修改。

use crate::subclasses::sketch_up_import_camera::SketchUpImportCamera;

#[derive(Debug, Clone)]
pub struct SketchUpImportScene {
    pub camera: SketchUpImportCamera,
    pub name: String,
}

impl crate::Parse for SketchUpImportScene {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let camera = SketchUpImportCamera::parse(r, ver)?;
        r.align(4)?;
        let name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(SketchUpImportScene {
            camera,
            name,
        })
    }
}
