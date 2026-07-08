//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedTagMap {
    pub tags: Vec<(String, String)>,
}

impl crate::Parse for SerializedTagMap {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let tags = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(SerializedTagMap {
            tags,
        })
    }
}
