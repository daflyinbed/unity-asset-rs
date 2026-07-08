//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Guid {
    pub data_0: u32,
    pub data_1: u32,
    pub data_2: u32,
    pub data_3: u32,
}

impl crate::Parse for Guid {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let data_0 = rdr.read_u32()?;
        let data_1 = rdr.read_u32()?;
        let data_2 = rdr.read_u32()?;
        let data_3 = rdr.read_u32()?;

        Ok(Guid {
            data_0,
            data_1,
            data_2,
            data_3,
        })
    }
}
