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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let guid = Guid::parse(r, ver)?;
        let dependencies = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let generated_paths = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let blob = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let state = r.read_i32()?;

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
