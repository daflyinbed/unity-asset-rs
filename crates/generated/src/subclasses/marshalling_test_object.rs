//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MarshallingTestObject {
    pub m_object_hide_flags: u32,
    pub m_prop: i32,
}

impl crate::Parse for MarshallingTestObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prop = r.read_i32()?;

        Ok(MarshallingTestObject {
            m_object_hide_flags,
            m_prop,
        })
    }
}
