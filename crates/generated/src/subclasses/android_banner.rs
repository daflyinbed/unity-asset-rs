//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AndroidBanner {
    pub width: i32,
    pub height: i32,
    pub banner: PPtr,
}

impl crate::Parse for AndroidBanner {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let width = r.read_i32()?;
        let height = r.read_i32()?;
        let banner = crate::PPtr::parse(r, ver)?;

        Ok(AndroidBanner {
            width,
            height,
            banner,
        })
    }
}
