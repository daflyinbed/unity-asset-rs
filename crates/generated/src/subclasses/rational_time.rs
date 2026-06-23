//! 自动生成。请勿手动修改。

use crate::subclasses::ticks_per_second::TicksPerSecond;

#[derive(Debug, Clone)]
pub struct RationalTime {
    pub m_count: i64,
    pub m_rate: TicksPerSecond,
}

impl crate::Parse for RationalTime {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_count = r.read_i64()?;
        let m_rate = TicksPerSecond::parse(r, ver)?;

        Ok(RationalTime {
            m_count,
            m_rate,
        })
    }
}
