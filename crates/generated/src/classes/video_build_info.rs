//! 自动生成。请勿手动修改。

use super::object::Object;

#[derive(Debug, Clone)]
pub struct VideoBuildInfo {
    pub parent: Object,
    pub m_video_clip_count: i32,
    pub m_is_video_module_disabled: bool,
}

impl crate::Parse for VideoBuildInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_video_clip_count = r.read_i32()?;
        let m_is_video_module_disabled = r.read_bool()?;

        Ok(VideoBuildInfo {
            parent,
            m_video_clip_count,
            m_is_video_module_disabled,
        })
    }
}
