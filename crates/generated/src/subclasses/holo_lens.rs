//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct HoloLens {
    pub depth_format: i32,
    pub depth_buffer_sharing_enabled: Option<bool>,
}

impl crate::Parse for HoloLens {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let depth_format = rdr.read_i32()?;
        let depth_buffer_sharing_enabled = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };

        Ok(HoloLens {
            depth_format,
            depth_buffer_sharing_enabled,
        })
    }
}
