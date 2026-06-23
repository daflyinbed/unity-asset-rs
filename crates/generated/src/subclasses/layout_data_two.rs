//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LayoutDataTwo {
    pub m_integer_value: i32,
    pub m_float_value: f32,
}

impl crate::Parse for LayoutDataTwo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_integer_value = r.read_i32()?;
        let m_float_value = r.read_f32()?;

        Ok(LayoutDataTwo {
            m_integer_value,
            m_float_value,
        })
    }
}
