//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Hint {
    pub m_key: String,
    pub m_value: String,
}

impl crate::Parse for Hint {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_key = r.read_aligned_string()?;
        r.align(4)?;
        let m_value = r.read_aligned_string()?;
        r.align(4)?;

        Ok(Hint {
            m_key,
            m_value,
        })
    }
}
