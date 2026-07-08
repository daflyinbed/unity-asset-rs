//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SourceTextureInformation {
    pub width: i32,
    pub height: i32,
    pub does_texture_contain_alpha: bool,
    pub does_texture_contain_color: Option<bool>,
    pub source_was_hdr: Option<bool>,
}

impl crate::Parse for SourceTextureInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let width = rdr.read_i32()?;
        let height = rdr.read_i32()?;
        let does_texture_contain_alpha = rdr.read_bool()?;
        let does_texture_contain_color = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };
        let source_was_hdr = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SourceTextureInformation {
            width,
            height,
            does_texture_contain_alpha,
            does_texture_contain_color,
            source_was_hdr,
        })
    }
}
