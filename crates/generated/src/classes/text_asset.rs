//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TextAsset {
    pub m_name: String,
    pub m_script: String,
    pub m_path_name: Option<String>,
}

impl crate::Parse for TextAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_script = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_path_name = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(TextAsset {
            m_name,
            m_script,
            m_path_name,
        })
    }
}
