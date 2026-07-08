//! 自动生成。请勿手动修改。

use crate::subclasses::layout_data_one::LayoutDataOne;

#[derive(Debug, Clone)]
pub struct TestObjectWithSpecialLayoutOne {
    pub different_layout: LayoutDataOne,
    pub same_layout: LayoutDataOne,
}

impl crate::Parse for TestObjectWithSpecialLayoutOne {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let different_layout = LayoutDataOne::parse(rdr, ver)?;
        rdr.align(4)?;
        let same_layout = LayoutDataOne::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(TestObjectWithSpecialLayoutOne {
            different_layout,
            same_layout,
        })
    }
}
