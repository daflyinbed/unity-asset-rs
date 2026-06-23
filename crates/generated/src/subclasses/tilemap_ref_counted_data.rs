//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TilemapRefCountedData {
    pub m_ref_count: u32,
    pub m_data: PPtr,
}

impl crate::Parse for TilemapRefCountedData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_ref_count = r.read_u32()?;
        let m_data = crate::PPtr::parse(r, ver)?;

        Ok(TilemapRefCountedData {
            m_ref_count,
            m_data,
        })
    }
}
