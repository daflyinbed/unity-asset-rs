//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct StreamedClip {
    pub data: Vec<u32>,
    pub curve_count: u32,
}

impl crate::Parse for StreamedClip {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let curve_count = r.read_u32()?;

        Ok(StreamedClip {
            data,
            curve_count,
        })
    }
}
