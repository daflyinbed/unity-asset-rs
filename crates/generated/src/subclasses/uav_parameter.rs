//! 自动生成。请勿手动修改。

use crate::subclasses::binding::Binding;

#[derive(Debug, Clone)]
pub struct UavParameter {
    pub m_name_index: i32,
    pub m_index: Option<i32>,
    pub m_binding: Option<Binding>,
    pub m_original_index: Option<i32>,
    pub m_original_binding: Option<Binding>,
}

impl crate::Parse for UavParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name_index = None;
        let mut m_index = None;
        let mut m_binding = None;
        let mut m_original_index = None;
        let mut m_original_binding = None;
            m_name_index = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_binding = Some(Binding::parse(rdr, ver)?);
            m_original_binding = Some(Binding::parse(rdr, ver)?);
        } else {
            m_index = Some(rdr.read_i32()?);
            m_original_index = Some(rdr.read_i32()?);
        }

        Ok(UavParameter {
            m_name_index: m_name_index.unwrap(),
            m_index,
            m_binding,
            m_original_index,
            m_original_binding,
        })
    }
}
