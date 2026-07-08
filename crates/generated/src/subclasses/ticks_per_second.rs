//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TicksPerSecond {
    pub m_denominator: u32,
    pub m_numerator: u32,
}

impl crate::Parse for TicksPerSecond {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_denominator = rdr.read_u32()?;
        let m_numerator = rdr.read_u32()?;

        Ok(TicksPerSecond {
            m_denominator,
            m_numerator,
        })
    }
}
