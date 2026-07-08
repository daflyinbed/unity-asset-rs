//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub m_assembly_name_index: i32,
    pub m_namespace_index: i32,
    pub m_class_name: String,
    pub m_num_of_methods: i32,
    pub m_namespace_name: Option<String>,
    pub m_method_index: i32,
    pub m_is_unity_class: bool,
}

impl crate::Parse for ClassInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_assembly_name_index = None;
        let mut m_namespace_index = None;
        let mut m_class_name = None;
        let mut m_num_of_methods = None;
        let mut m_namespace_name = None;
        let mut m_method_index = None;
        let mut m_is_unity_class = None;
            m_assembly_name_index = Some(rdr.read_i32()?);
            m_namespace_index = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_class_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2019, 4, 29) {
            m_namespace_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else {
        }
            m_num_of_methods = Some(rdr.read_i32()?);
            m_method_index = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_is_unity_class = Some(rdr.read_bool()?);
            rdr.align(4)?;

        Ok(ClassInfo {
            m_assembly_name_index: m_assembly_name_index.unwrap(),
            m_namespace_index: m_namespace_index.unwrap(),
            m_class_name: m_class_name.unwrap(),
            m_num_of_methods: m_num_of_methods.unwrap(),
            m_namespace_name,
            m_method_index: m_method_index.unwrap(),
            m_is_unity_class: m_is_unity_class.unwrap(),
        })
    }
}
