//! 自动生成。请勿手动修改。

use crate::subclasses::source_location::SourceLocation;

#[derive(Debug, Clone)]
pub struct Message {
    pub m_severity: i32,
    pub m_code: i32,
    pub m_location: SourceLocation,
}

impl crate::Parse for Message {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_severity = r.read_i32()?;
        let m_code = r.read_i32()?;
        let m_location = SourceLocation::parse(r, ver)?;
        r.align(4)?;

        Ok(Message {
            m_severity,
            m_code,
            m_location,
        })
    }
}
