//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SiblingDerived {
    pub m_object_hide_flags: u32,
}

impl crate::Parse for SiblingDerived {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;

        Ok(SiblingDerived {
            m_object_hide_flags,
        })
    }
}
