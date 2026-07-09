//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct XboxOneSocketDefinition {
    pub m_name: String,
    pub m_port: String,
    pub m_protocol: i32,
    pub m_usages: Vec<i32>,
    pub m_template_name: String,
    pub m_session_requirment: i32,
    pub m_device_usages: Vec<i32>,
}

impl crate::Parse for XboxOneSocketDefinition {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_port = rdr.read_aligned_string()?;
        let m_protocol = rdr.read_i32()?;
        let m_usages = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_template_name = rdr.read_aligned_string()?;
        let m_session_requirment = rdr.read_i32()?;
        let m_device_usages = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(XboxOneSocketDefinition {
            m_name,
            m_port,
            m_protocol,
            m_usages,
            m_template_name,
            m_session_requirment,
            m_device_usages,
        })
    }
}
