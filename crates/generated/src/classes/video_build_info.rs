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

impl crate::ToValue for VideoBuildInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_VideoClipCount".to_string(),
                self.m_video_clip_count.to_value(),
            )),
            Some((
                "m_IsVideoModuleDisabled".to_string(),
                self.m_is_video_module_disabled.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_VideoClipCount" => (self.m_video_clip_count).inspect_value(path, offset, limit),
                "m_IsVideoModuleDisabled" => {
                    (self.m_is_video_module_disabled).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_video_clip_count).inspect_value(path, offset, limit))
                .map(|v| ("m_VideoClipCount".to_string(), v)),
            ((self.m_is_video_module_disabled).inspect_value(path, offset, limit))
                .map(|v| ("m_IsVideoModuleDisabled".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
