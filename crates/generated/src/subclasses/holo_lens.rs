//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct HoloLens {
    pub depth_format: i32,
}

impl crate::Parse for HoloLens {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let depth_format = r.read_i32()?;

        Ok(HoloLens {
            depth_format,
        })
    }
}
