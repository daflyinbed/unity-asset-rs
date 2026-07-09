//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ComponentPair {
    pub first: Option<i32>,
    pub component: Option<PPtr>,
    pub second: Option<PPtr>,
}

impl crate::Parse for ComponentPair {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut first = None;
        let mut component = None;
        let mut second = None;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            component = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
            first = Some(rdr.read_i32()?);
            second = Some(crate::PPtr::parse(rdr, ver)?);
        }

        Ok(ComponentPair {
            first,
            component,
            second,
        })
    }
}
