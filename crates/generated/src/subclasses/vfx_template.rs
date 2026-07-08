//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct VfxTemplate {
    pub name: String,
    pub category: String,
    pub description: String,
    pub icon: PPtr,
    pub thumbnail: PPtr,
    pub order: Option<i32>,
}

impl crate::Parse for VfxTemplate {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let category = rdr.read_aligned_string()?;
        let description = rdr.read_aligned_string()?;
        let icon = crate::PPtr::parse(rdr, ver)?;
        let thumbnail = crate::PPtr::parse(rdr, ver)?;
        let order = if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(VfxTemplate {
            name,
            category,
            description,
            icon,
            thumbnail,
            order,
        })
    }
}
