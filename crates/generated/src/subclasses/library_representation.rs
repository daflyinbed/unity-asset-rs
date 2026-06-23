//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::image::Image;

#[derive(Debug, Clone)]
pub struct LibraryRepresentation {
    pub name: String,
    pub thumbnail: Image,
    pub object: PPtr,
    pub thumbnail_class_id: i32,
    pub script_class_name: String,
}

impl crate::Parse for LibraryRepresentation {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let thumbnail = Image::parse(r, ver)?;
        r.align(4)?;
        let object = crate::PPtr::parse(r, ver)?;
        let thumbnail_class_id = r.read_i32()?;
        let script_class_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(LibraryRepresentation {
            name,
            thumbnail,
            object,
            thumbnail_class_id,
            script_class_name,
        })
    }
}
