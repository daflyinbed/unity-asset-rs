//! 自动生成。请勿手动修改。

use crate::subclasses::layout_data_one::LayoutDataOne;

#[derive(Debug, Clone)]
pub struct TestObjectWithSpecialLayoutOne {
    pub m_object_hide_flags: u32,
    pub different_layout: LayoutDataOne,
    pub same_layout: LayoutDataOne,
}

impl crate::Parse for TestObjectWithSpecialLayoutOne {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let different_layout = LayoutDataOne::parse(r, ver)?;
        r.align(4)?;
        let same_layout = LayoutDataOne::parse(r, ver)?;
        r.align(4)?;

        Ok(TestObjectWithSpecialLayoutOne {
            m_object_hide_flags,
            different_layout,
            same_layout,
        })
    }
}
