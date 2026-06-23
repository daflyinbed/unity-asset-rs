//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AttachmentInfo {
    pub format: i32,
    pub needs_resolve: bool,
}

impl crate::Parse for AttachmentInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let format = r.read_i32()?;
        let needs_resolve = r.read_bool()?;
        r.align(4)?;

        Ok(AttachmentInfo {
            format,
            needs_resolve,
        })
    }
}
