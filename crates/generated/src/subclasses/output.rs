//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Output {
    pub has_empty_font_data: bool,
}

impl crate::Parse for Output {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let has_empty_font_data = rdr.read_bool()?;

        Ok(Output {
            has_empty_font_data,
        })
    }
}
