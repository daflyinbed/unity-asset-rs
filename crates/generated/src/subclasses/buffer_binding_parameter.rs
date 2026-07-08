//! 自动生成。请勿手动修改。

use crate::subclasses::binding::Binding;

#[derive(Debug, Clone)]
pub struct BufferBindingParameter {
    pub m_name_index: i32,
    pub m_index: Option<i32>,
    pub m_binding: Option<Binding>,
    pub m_array_size: i32,
}

impl crate::Parse for BufferBindingParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name_index = None;
        let mut m_index = None;
        let mut m_binding = None;
        let mut m_array_size = None;
            m_name_index = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_binding = Some(Binding::parse(rdr, ver)?);
        } else {
            m_index = Some(rdr.read_i32()?);
        }
            m_array_size = Some(rdr.read_i32()?);

        Ok(BufferBindingParameter {
            m_name_index: m_name_index.unwrap(),
            m_index,
            m_binding,
            m_array_size: m_array_size.unwrap(),
        })
    }
}
