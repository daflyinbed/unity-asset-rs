//! 自动生成。请勿手动修改。

use crate::subclasses::layout_data_three::LayoutDataThree;
use crate::subclasses::layout_data_two::LayoutDataTwo;

#[derive(Debug, Clone)]
pub struct TestObjectWithSpecialLayoutTwo {
    pub different_layout: LayoutDataTwo,
    pub same_layout: LayoutDataThree,
}

impl crate::Parse for TestObjectWithSpecialLayoutTwo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let different_layout = LayoutDataTwo::parse(r, ver)?;
        let same_layout = LayoutDataThree::parse(r, ver)?;
        r.align(4)?;

        Ok(TestObjectWithSpecialLayoutTwo {
            different_layout,
            same_layout,
        })
    }
}
