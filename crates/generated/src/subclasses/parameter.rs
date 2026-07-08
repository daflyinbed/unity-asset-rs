//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub m_parameter_name: String,
    pub m_guid: Guid,
}

impl crate::Parse for Parameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_parameter_name = rdr.read_aligned_string()?;
        let m_guid = Guid::parse(rdr, ver)?;

        Ok(Parameter {
            m_parameter_name,
            m_guid,
        })
    }
}
