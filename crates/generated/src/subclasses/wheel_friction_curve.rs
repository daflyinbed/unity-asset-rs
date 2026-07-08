//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct WheelFrictionCurve {
    pub extremum_slip: Option<f32>,
    pub m_extremum_slip: Option<f32>,
    pub extremum_value: Option<f32>,
    pub m_extremum_value: Option<f32>,
    pub asymptote_slip: Option<f32>,
    pub m_asymptote_slip: Option<f32>,
    pub asymptote_value: Option<f32>,
    pub m_asymptote_value: Option<f32>,
    pub stiffness_factor: Option<f32>,
    pub m_stiffness: Option<f32>,
}

impl crate::Parse for WheelFrictionCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut extremum_slip = None;
        let mut m_extremum_slip = None;
        let mut extremum_value = None;
        let mut m_extremum_value = None;
        let mut asymptote_slip = None;
        let mut m_asymptote_slip = None;
        let mut asymptote_value = None;
        let mut m_asymptote_value = None;
        let mut stiffness_factor = None;
        let mut m_stiffness = None;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_extremum_slip = Some(rdr.read_f32()?);
            m_extremum_value = Some(rdr.read_f32()?);
            m_asymptote_slip = Some(rdr.read_f32()?);
            m_asymptote_value = Some(rdr.read_f32()?);
            m_stiffness = Some(rdr.read_f32()?);
        } else {
            extremum_slip = Some(rdr.read_f32()?);
            extremum_value = Some(rdr.read_f32()?);
            asymptote_slip = Some(rdr.read_f32()?);
            asymptote_value = Some(rdr.read_f32()?);
            stiffness_factor = Some(rdr.read_f32()?);
        }

        Ok(WheelFrictionCurve {
            extremum_slip,
            m_extremum_slip,
            extremum_value,
            m_extremum_value,
            asymptote_slip,
            m_asymptote_slip,
            asymptote_value,
            m_asymptote_value,
            stiffness_factor,
            m_stiffness,
        })
    }
}
