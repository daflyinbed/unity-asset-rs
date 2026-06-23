//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct GuidSerializer {
    pub parent: Object,
    pub guid_to_path: Vec<(Guid, String)>,
}

impl crate::Parse for GuidSerializer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let guid_to_path = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(r, ver)?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(GuidSerializer {
            parent,
            guid_to_path,
        })
    }
}
