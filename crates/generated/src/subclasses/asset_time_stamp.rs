//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AssetTimeStamp {
    pub modification_date_0: u32,
    pub modification_date_1: u32,
    pub meta_modification_date_0: u32,
    pub meta_modification_date_1: u32,
}

impl crate::Parse for AssetTimeStamp {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let modification_date_0 = rdr.read_u32()?;
        let modification_date_1 = rdr.read_u32()?;
        let meta_modification_date_0 = rdr.read_u32()?;
        let meta_modification_date_1 = rdr.read_u32()?;

        Ok(AssetTimeStamp {
            modification_date_0,
            modification_date_1,
            meta_modification_date_0,
            meta_modification_date_1,
        })
    }
}
