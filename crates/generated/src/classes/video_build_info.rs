//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VideoBuildInfo {
    pub m_video_clip_count: i32,
    pub m_is_video_module_disabled: bool,
}

impl crate::Parse for VideoBuildInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_video_clip_count = rdr.read_i32()?;
        let m_is_video_module_disabled = rdr.read_bool()?;

        Ok(VideoBuildInfo {
            m_video_clip_count,
            m_is_video_module_disabled,
        })
    }
}
