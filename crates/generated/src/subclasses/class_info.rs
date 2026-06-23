//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub m_assembly_name_index: i32,
    pub m_namespace_index: i32,
    pub m_class_name: String,
    pub m_num_of_methods: i32,
    pub m_method_index: i32,
    pub m_is_unity_class: bool,
}

impl crate::Parse for ClassInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_assembly_name_index = r.read_i32()?;
        let m_namespace_index = r.read_i32()?;
        r.align(4)?;
        let m_class_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_num_of_methods = r.read_i32()?;
        let m_method_index = r.read_i32()?;
        r.align(4)?;
        let m_is_unity_class = r.read_bool()?;
        r.align(4)?;

        Ok(ClassInfo {
            m_assembly_name_index,
            m_namespace_index,
            m_class_name,
            m_num_of_methods,
            m_method_index,
            m_is_unity_class,
        })
    }
}
