//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ClusterInput {
    pub m_name: String,
    pub m_device_name: String,
    pub m_server_url: String,
    pub m_index: i32,
    pub m_type: i32,
}

impl crate::Parse for ClusterInput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_device_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_server_url = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_index = rdr.read_i32()?;
        let m_type = rdr.read_i32()?;

        Ok(ClusterInput {
            m_name,
            m_device_name,
            m_server_url,
            m_index,
            m_type,
        })
    }
}
