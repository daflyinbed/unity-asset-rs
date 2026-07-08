//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct GuidSerializer {
    pub guid_to_path: Vec<(Guid, String)>,
}

impl crate::Parse for GuidSerializer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guid_to_path = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(rdr, ver)?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(GuidSerializer {
            guid_to_path,
        })
    }
}
