//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct fixed_bitset {
    pub array: Vec<u32>,
}

impl crate::Parse for fixed_bitset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(fixed_bitset { array })
    }
}
