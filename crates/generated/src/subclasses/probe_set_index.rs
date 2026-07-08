//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct ProbeSetIndex {
    pub m_hash: Hash128,
    pub m_offset: i32,
    pub m_size: i32,
}

impl crate::Parse for ProbeSetIndex {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_hash = Hash128::parse(rdr, ver)?;
        let m_offset = rdr.read_i32()?;
        let m_size = rdr.read_i32()?;

        Ok(ProbeSetIndex {
            m_hash,
            m_offset,
            m_size,
        })
    }
}
