//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Google {
    pub depth_format: i32,
    pub enable_transition_view: bool,
}

impl crate::Parse for Google {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let depth_format = r.read_i32()?;
        r.align(4)?;
        let enable_transition_view = r.read_bool()?;

        Ok(Google {
            depth_format,
            enable_transition_view,
        })
    }
}
