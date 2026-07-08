//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PackedBitVector {
    pub m_num_items: u32,
    pub m_range: Option<f32>,
    pub m_data: Vec<u8>,
    pub m_start: Option<f32>,
    pub m_bit_size: u8,
}

impl crate::Parse for PackedBitVector {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_num_items = None;
        let mut m_range = None;
        let mut m_data = None;
        let mut m_start = None;
        let mut m_bit_size = None;
            m_num_items = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            m_range = Some(rdr.read_f32()?);
            m_start = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_range = Some(rdr.read_f32()?);
            m_start = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 5) {
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_range = Some(rdr.read_f32()?);
            m_start = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 1) {
        } else {
            m_range = Some(rdr.read_f32()?);
            m_start = Some(rdr.read_f32()?);
        }
            m_data = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_bit_size = Some(rdr.read_u8()?);
            rdr.align(4)?;

        Ok(PackedBitVector {
            m_num_items: m_num_items.unwrap(),
            m_range,
            m_data: m_data.unwrap(),
            m_start,
            m_bit_size: m_bit_size.unwrap(),
        })
    }
}
