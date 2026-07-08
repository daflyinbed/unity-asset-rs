//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct DateTime {
    pub ticks: i64,
}

impl crate::Parse for DateTime {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let ticks = rdr.read_i64()?;

        Ok(DateTime {
            ticks,
        })
    }
}
