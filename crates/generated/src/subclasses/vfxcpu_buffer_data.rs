//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxcpuBufferData {
    pub data: Vec<u32>,
}

impl crate::Parse for VfxcpuBufferData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(VfxcpuBufferData {
            data,
        })
    }
}
