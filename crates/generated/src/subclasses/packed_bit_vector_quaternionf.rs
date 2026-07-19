//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PackedBitVector_Quaternionf {
    pub m_num_items: u32,
    pub m_data: Vec<u8>,
}

impl crate::Parse for PackedBitVector_Quaternionf {
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

        Ok(PackedBitVector_Quaternionf {
            m_num_items,
            m_data,
        })
    }
}
