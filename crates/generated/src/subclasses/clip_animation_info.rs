//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ClipAnimationInfo {
    pub name: String,
    pub first_frame: i32,
    pub last_frame: i32,
    pub wrap_mode: i32,
    pub r#loop: bool,
}

impl crate::Parse for ClipAnimationInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let first_frame = r.read_i32()?;
        let last_frame = r.read_i32()?;
        let wrap_mode = r.read_i32()?;
        let r#loop = r.read_bool()?;
        r.align(4)?;

        Ok(ClipAnimationInfo {
            name,
            first_frame,
            last_frame,
            wrap_mode,
            r#loop,
        })
    }
}
