//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BitField {
    pub m_bits: u32,
}

impl crate::Parse for BitField {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_bits = rdr.read_u32()?;

        Ok(BitField {
            m_bits,
        })
    }
}
