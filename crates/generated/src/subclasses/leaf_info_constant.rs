//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LeafInfoConstant {
    pub m_id_array: Vec<u32>,
    pub m_index_offset: u32,
}

impl crate::Parse for LeafInfoConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_id_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_index_offset = r.read_u32()?;

        Ok(LeafInfoConstant {
            m_id_array,
            m_index_offset,
        })
    }
}
