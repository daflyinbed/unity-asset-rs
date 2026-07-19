//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct CgProgram {
    pub m_name: String,
    pub m_script: String,
    pub m_path_name: Option<String>,
}

impl crate::Parse for CgProgram {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_script = None;
        let mut m_path_name = None;
        m_name = Some(rdr.read_aligned_string()?);
        m_script = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
        } else {
            m_path_name = Some(rdr.read_aligned_string()?);
        }

        Ok(CgProgram {
            m_name: m_name.unwrap(),
            m_script: m_script.unwrap(),
            m_path_name,
        })
    }
}
