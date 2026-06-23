//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ClassMethodInfo {
    pub m_class_index: i32,
    pub m_method_name: String,
    pub m_order_number: i32,
}

impl crate::Parse for ClassMethodInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_class_index = r.read_i32()?;
        let m_method_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_order_number = r.read_i32()?;
        r.align(4)?;

        Ok(ClassMethodInfo {
            m_class_index,
            m_method_name,
            m_order_number,
        })
    }
}
