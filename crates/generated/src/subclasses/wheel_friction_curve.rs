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
        let extremum_slip = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let m_extremum_slip = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let extremum_value = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let m_extremum_value = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let asymptote_slip = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let m_asymptote_slip = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let asymptote_value = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let m_asymptote_value = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let stiffness_factor = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let m_stiffness = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

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
