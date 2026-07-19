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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let errors = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Error::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let file_path = rdr.read_aligned_string()?;

        Ok(BlockShaderErrors {
            m_name,
            errors,
            file_path,
        })
    }
}
