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
        let m_name_index = rdr.read_i32()?;
        let m_index = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_binding = if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            Some(Binding::parse(rdr, ver)?)
        } else {
            None
        };
        let m_array_size = rdr.read_i32()?;

        Ok(BufferBindingParameter {
            m_name_index,
            m_index,
            m_binding,
            m_array_size,
        })
    }
}
