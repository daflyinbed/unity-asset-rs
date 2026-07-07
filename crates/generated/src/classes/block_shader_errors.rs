//! 自动生成。请勿手动修改。

use crate::subclasses::error::Error;

#[derive(Debug, Clone)]
pub struct BlockShaderErrors {
    pub m_name: String,
    pub errors: Vec<Error>,
    pub file_path: String,
}

impl crate::Parse for BlockShaderErrors {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let errors = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Error::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let file_path = r.read_aligned_string()?;
        r.align(4)?;

        Ok(BlockShaderErrors {
            m_name,
            errors,
            file_path,
        })
    }
}
