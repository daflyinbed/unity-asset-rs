//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct BlockShaderContainer {
    pub m_name: String,
    pub guid: Guid,
    pub dependencies: Vec<PPtr>,
    pub generated_paths: Vec<String>,
    pub blob: Vec<u32>,
    pub state: i32,
}

impl crate::Parse for BlockShaderContainer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let guid = Guid::parse(rdr, ver)?;
        let dependencies = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let generated_paths = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let blob = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let state = rdr.read_i32()?;

        Ok(BlockShaderContainer {
            m_name,
            guid,
            dependencies,
            generated_paths,
            blob,
            state,
        })
    }
}
