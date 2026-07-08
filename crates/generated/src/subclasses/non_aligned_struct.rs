//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct NonAlignedStruct {
    pub m_bool: bool,
}

impl crate::Parse for NonAlignedStruct {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_bool = rdr.read_bool()?;

        Ok(NonAlignedStruct {
            m_bool,
        })
    }
}
