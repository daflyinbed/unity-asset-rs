//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct VfxTemplate {
    pub name: String,
    pub category: String,
    pub description: String,
    pub icon: PPtr,
    pub thumbnail: PPtr,
}

impl crate::Parse for VfxTemplate {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let category = r.read_aligned_string()?;
        r.align(4)?;
        let description = r.read_aligned_string()?;
        r.align(4)?;
        let icon = crate::PPtr::parse(r, ver)?;
        let thumbnail = crate::PPtr::parse(r, ver)?;

        Ok(VfxTemplate {
            name,
            category,
            description,
            icon,
            thumbnail,
        })
    }
}
