//! 自动生成。请勿手动修改。

use crate::subclasses::source_location::SourceLocation;

#[derive(Debug, Clone)]
pub struct Message {
    pub m_severity: i32,
    pub m_code: i32,
    pub m_location: SourceLocation,
    pub m_text: String,
}

impl crate::Parse for Message {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_severity = rdr.read_i32()?;
        let m_code = rdr.read_i32()?;
        let m_location = SourceLocation::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_text = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(Message {
            m_severity,
            m_code,
            m_location,
            m_text,
        })
    }
}
