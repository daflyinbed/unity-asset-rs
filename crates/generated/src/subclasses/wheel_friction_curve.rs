//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct WheelFrictionCurve {
    pub extremum_slip: f32,
    pub extremum_value: f32,
    pub asymptote_slip: f32,
    pub asymptote_value: f32,
    pub stiffness_factor: f32,
}

impl crate::Parse for WheelFrictionCurve {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let extremum_slip = r.read_f32()?;
        let extremum_value = r.read_f32()?;
        let asymptote_slip = r.read_f32()?;
        let asymptote_value = r.read_f32()?;
        let stiffness_factor = r.read_f32()?;

        Ok(WheelFrictionCurve {
            extremum_slip,
            extremum_value,
            asymptote_slip,
            asymptote_value,
            stiffness_factor,
        })
    }
}
