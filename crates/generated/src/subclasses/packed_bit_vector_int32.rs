//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PackedBitVector_Int32 {
    pub m_num_items: u32,
    pub m_data: Vec<u8>,
    pub m_bit_size: u8,
}

impl crate::Parse for PackedBitVector_Int32 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_num_items = rdr.read_u32()?;
        let m_data = ({
            let n = rdr.read_i32()? as usize;
            (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_bit_size = rdr.read_u8()?;
        rdr.align(4)?;

        Ok(PackedBitVector_Int32 {
            m_num_items,
            m_data,
            m_bit_size,
        })
    }
}
