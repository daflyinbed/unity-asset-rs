//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PropertyNameNested {
    pub id: String,
}

impl crate::Parse for PropertyNameNested {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let id = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(PropertyNameNested {
            id,
        })
    }
}
