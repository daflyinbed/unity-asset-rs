//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PropertyName32 {
    pub id: i32,
}

impl crate::Parse for PropertyName32 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let id = rdr.read_i32()?;
        rdr.align(4)?;

        Ok(PropertyName32 {
            id,
        })
    }
}
