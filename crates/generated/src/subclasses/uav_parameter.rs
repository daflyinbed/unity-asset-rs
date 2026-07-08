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
        let m_name_index = rdr.read_i32()?;
        let m_index = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_binding = if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            Some(Binding::parse(rdr, ver)?)
        } else {
            None
        };
        let m_original_index = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_original_binding = if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            Some(Binding::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(UavParameter {
            m_name_index,
            m_index,
            m_binding,
            m_original_index,
            m_original_binding,
        })
    }
}
