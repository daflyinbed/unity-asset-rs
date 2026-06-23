//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxLayoutOffset {
    pub bucket: u32,
    pub structure: u32,
    pub element: u32,
}

impl crate::Parse for VfxLayoutOffset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bucket = r.read_u32()?;
        let structure = r.read_u32()?;
        let element = r.read_u32()?;

        Ok(VfxLayoutOffset {
            bucket,
            structure,
            element,
        })
    }
}
