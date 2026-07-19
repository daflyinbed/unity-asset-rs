//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ColorRgba32 {
    pub rgba: u32,
}

impl crate::Parse for ColorRgba32 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let rgba = rdr.read_u32()?;

        Ok(ColorRgba32 { rgba })
    }
}
