//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ColorRgba {
    pub rgba: u32,
}

impl crate::Parse for ColorRgba {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let rgba = r.read_u32()?;

        Ok(ColorRgba {
            rgba,
        })
    }
}
