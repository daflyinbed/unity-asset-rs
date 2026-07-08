//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MotionNeighborList {
    pub m_neighbor_array: Vec<u32>,
}

impl crate::Parse for MotionNeighborList {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_neighbor_array = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(MotionNeighborList {
            m_neighbor_array,
        })
    }
}
