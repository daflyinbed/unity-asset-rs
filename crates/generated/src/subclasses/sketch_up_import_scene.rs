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

        Ok(SketchUpImportScene {
            camera,
            name,
        })
    }
}
