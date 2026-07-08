//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_layout_offset::VfxLayoutOffset;

#[derive(Debug, Clone)]
pub struct VfxLayoutElementDesc {
    pub name: String,
    pub r#type: i32,
    pub offset: VfxLayoutOffset,
}

impl crate::Parse for VfxLayoutElementDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let r#type = rdr.read_i32()?;
        let offset = VfxLayoutOffset::parse(rdr, ver)?;

        Ok(VfxLayoutElementDesc {
            name,
            r#type,
            offset,
        })
    }
}
