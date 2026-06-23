//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TicksPerSecond {
    pub m_denominator: u32,
    pub m_numerator: u32,
}

impl crate::Parse for TicksPerSecond {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_denominator = r.read_u32()?;
        let m_numerator = r.read_u32()?;

        Ok(TicksPerSecond {
            m_denominator,
            m_numerator,
        })
    }
}
