//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MdFour {
    pub md4_hash: Vec<u8>,
}

impl crate::Parse for MdFour {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let md4_hash = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
        r.align(4)?;

        Ok(MdFour {
            md4_hash,
        })
    }
}
