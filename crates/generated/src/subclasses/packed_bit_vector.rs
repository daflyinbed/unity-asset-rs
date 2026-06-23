//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PackedBitVector {
    pub m_num_items: u32,
    pub m_range: f32,
    pub m_start: f32,
    pub m_data: Vec<u8>,
    pub m_bit_size: u8,
}

impl crate::Parse for PackedBitVector {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_num_items = r.read_u32()?;
        let m_range = r.read_f32()?;
        let m_start = r.read_f32()?;
        let m_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_bit_size = r.read_u8()?;
        r.align(4)?;

        Ok(PackedBitVector {
            m_num_items,
            m_range,
            m_start,
            m_data,
            m_bit_size,
        })
    }
}
