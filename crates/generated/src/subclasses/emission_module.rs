//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct EmissionModule {
    pub enabled: bool,
    pub m_type: i32,
    pub rate: MinMaxCurve,
    pub cnt0: u16,
    pub cnt1: u16,
    pub cnt2: u16,
    pub cnt3: u16,
    pub time0: f32,
    pub time1: f32,
    pub time2: f32,
    pub time3: f32,
    pub m_burst_count: u8,
}

impl crate::Parse for EmissionModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let m_type = r.read_i32()?;
        let rate = MinMaxCurve::parse(r, ver)?;
        r.align(4)?;
        let cnt0 = r.read_u16()?;
        let cnt1 = r.read_u16()?;
        let cnt2 = r.read_u16()?;
        let cnt3 = r.read_u16()?;
        let time0 = r.read_f32()?;
        let time1 = r.read_f32()?;
        let time2 = r.read_f32()?;
        let time3 = r.read_f32()?;
        let m_burst_count = r.read_u8()?;
        r.align(4)?;

        Ok(EmissionModule {
            enabled,
            m_type,
            rate,
            cnt0,
            cnt1,
            cnt2,
            cnt3,
            time0,
            time1,
            time2,
            time3,
            m_burst_count,
        })
    }
}
