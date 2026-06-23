//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PropertyModificationsTargetTestNativeObject {
    pub m_integer_value: i32,
    pub m_float_value: f32,
}

impl crate::Parse for PropertyModificationsTargetTestNativeObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_integer_value = r.read_i32()?;
        let m_float_value = r.read_f32()?;

        Ok(PropertyModificationsTargetTestNativeObject {
            m_integer_value,
            m_float_value,
        })
    }
}
