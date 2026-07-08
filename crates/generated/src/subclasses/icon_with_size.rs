//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct IconWithSize {
    pub m_icon: PPtr,
    pub m_size: i32,
}

impl crate::Parse for IconWithSize {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_icon = crate::PPtr::parse(rdr, ver)?;
        let m_size = rdr.read_i32()?;

        Ok(IconWithSize {
            m_icon,
            m_size,
        })
    }
}
