//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct DefaultAsset {
    pub m_name: String,
    pub m_message: Option<String>,
    pub m_is_warning: Option<bool>,
}

impl crate::Parse for DefaultAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_message = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_warning = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };

        Ok(DefaultAsset {
            m_name,
            m_message,
            m_is_warning,
        })
    }
}
