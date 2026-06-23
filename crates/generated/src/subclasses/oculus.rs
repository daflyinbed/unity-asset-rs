//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Oculus {
    pub shared_depth_buffer: bool,
    pub dash_support: bool,
}

impl crate::Parse for Oculus {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let shared_depth_buffer = r.read_bool()?;
        let dash_support = r.read_bool()?;
        r.align(4)?;

        Ok(Oculus {
            shared_depth_buffer,
            dash_support,
        })
    }
}
