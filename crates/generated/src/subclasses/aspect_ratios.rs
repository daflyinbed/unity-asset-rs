//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AspectRatios {
    pub f_4_3: bool,
    pub f_5_4: bool,
    pub f_16_10: bool,
    pub f_16_9: bool,
    pub others: bool,
}

impl crate::Parse for AspectRatios {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let f_4_3 = rdr.read_bool()?;
        let f_5_4 = rdr.read_bool()?;
        let f_16_10 = rdr.read_bool()?;
        let f_16_9 = rdr.read_bool()?;
        let others = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(AspectRatios {
            f_4_3,
            f_5_4,
            f_16_10,
            f_16_9,
            others,
        })
    }
}
