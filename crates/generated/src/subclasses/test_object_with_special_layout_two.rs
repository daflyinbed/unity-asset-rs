//! 自动生成。请勿手动修改。

use crate::subclasses::layout_data_three::LayoutDataThree;
use crate::subclasses::layout_data_two::LayoutDataTwo;

#[derive(Debug, Clone)]
pub struct TestObjectWithSpecialLayoutTwo {
    pub m_object_hide_flags: u32,
    pub different_layout: LayoutDataTwo,
    pub same_layout: LayoutDataThree,
}

impl crate::Parse for TestObjectWithSpecialLayoutTwo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let different_layout = LayoutDataTwo::parse(r, ver)?;
        let same_layout = LayoutDataThree::parse(r, ver)?;
        r.align(4)?;

        Ok(TestObjectWithSpecialLayoutTwo {
            m_object_hide_flags,
            different_layout,
            same_layout,
        })
    }
}
