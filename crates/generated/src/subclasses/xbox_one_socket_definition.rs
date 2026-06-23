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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_port = r.read_aligned_string()?;
        r.align(4)?;
        let m_protocol = r.read_i32()?;
        let m_usages = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_template_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_session_requirment = r.read_i32()?;
        let m_device_usages = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

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
