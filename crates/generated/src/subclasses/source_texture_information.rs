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
        let mut width = None;
        let mut height = None;
        let mut does_texture_contain_alpha = None;
        let mut does_texture_contain_color = None;
        let mut source_was_hdr = None;
        width = Some(rdr.read_i32()?);
        height = Some(rdr.read_i32()?);
        does_texture_contain_alpha = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            source_was_hdr = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            does_texture_contain_color = Some(rdr.read_bool()?);
        } else {
        }

        Ok(SourceTextureInformation {
            width: width.unwrap(),
            height: height.unwrap(),
            does_texture_contain_alpha: does_texture_contain_alpha.unwrap(),
            does_texture_contain_color,
            source_was_hdr,
        })
    }
}
