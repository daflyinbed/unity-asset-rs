//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ComputeBufferCounter {
    pub bindpoint: i32,
    pub offset: i32,
}

impl crate::Parse for ComputeBufferCounter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bindpoint = rdr.read_i32()?;
        let offset = rdr.read_i32()?;

        Ok(ComputeBufferCounter { bindpoint, offset })
    }
}
