//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxLayoutOffset {
    pub bucket: u32,
    pub structure: u32,
    pub element: u32,
}

impl crate::Parse for VfxLayoutOffset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bucket = rdr.read_u32()?;
        let structure = rdr.read_u32()?;
        let element = rdr.read_u32()?;

        Ok(VfxLayoutOffset {
            bucket,
            structure,
            element,
        })
    }
}
