//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct CgProgram {
    pub m_name: String,
    pub m_script: Option<String>,
    pub m_path_name: Option<String>,
}

impl crate::Parse for CgProgram {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_script = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_path_name = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(CgProgram {
            m_name,
            m_script,
            m_path_name,
        })
    }
}
