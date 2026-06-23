//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct bitset {
    pub bit_count: i32,
    pub bitblocks: Vec<u8>,
}

impl crate::Parse for bitset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bit_count = r.read_i32()?;
        let bitblocks = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
        r.align(4)?;

        Ok(bitset {
            bit_count,
            bitblocks,
        })
    }
}
