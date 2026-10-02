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

impl crate::ToValue for WheelFrictionCurve {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("extremumSlip", &self.extremum_slip),
            crate::opt_field("m_ExtremumSlip", &self.m_extremum_slip),
            crate::opt_field("extremumValue", &self.extremum_value),
            crate::opt_field("m_ExtremumValue", &self.m_extremum_value),
            crate::opt_field("asymptoteSlip", &self.asymptote_slip),
            crate::opt_field("m_AsymptoteSlip", &self.m_asymptote_slip),
            crate::opt_field("asymptoteValue", &self.asymptote_value),
            crate::opt_field("m_AsymptoteValue", &self.m_asymptote_value),
            crate::opt_field("stiffnessFactor", &self.stiffness_factor),
            crate::opt_field("m_Stiffness", &self.m_stiffness),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "extremumSlip" => (self.extremum_slip).inspect_value(path, offset, limit),
                "m_ExtremumSlip" => (self.m_extremum_slip).inspect_value(path, offset, limit),
                "extremumValue" => (self.extremum_value).inspect_value(path, offset, limit),
                "m_ExtremumValue" => (self.m_extremum_value).inspect_value(path, offset, limit),
                "asymptoteSlip" => (self.asymptote_slip).inspect_value(path, offset, limit),
                "m_AsymptoteSlip" => (self.m_asymptote_slip).inspect_value(path, offset, limit),
                "asymptoteValue" => (self.asymptote_value).inspect_value(path, offset, limit),
                "m_AsymptoteValue" => (self.m_asymptote_value).inspect_value(path, offset, limit),
                "stiffnessFactor" => (self.stiffness_factor).inspect_value(path, offset, limit),
                "m_Stiffness" => (self.m_stiffness).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.extremum_slip).inspect_value(path, offset, limit))
                .map(|v| ("extremumSlip".to_string(), v)),
            ((self.m_extremum_slip).inspect_value(path, offset, limit))
                .map(|v| ("m_ExtremumSlip".to_string(), v)),
            ((self.extremum_value).inspect_value(path, offset, limit))
                .map(|v| ("extremumValue".to_string(), v)),
            ((self.m_extremum_value).inspect_value(path, offset, limit))
                .map(|v| ("m_ExtremumValue".to_string(), v)),
            ((self.asymptote_slip).inspect_value(path, offset, limit))
                .map(|v| ("asymptoteSlip".to_string(), v)),
            ((self.m_asymptote_slip).inspect_value(path, offset, limit))
                .map(|v| ("m_AsymptoteSlip".to_string(), v)),
            ((self.asymptote_value).inspect_value(path, offset, limit))
                .map(|v| ("asymptoteValue".to_string(), v)),
            ((self.m_asymptote_value).inspect_value(path, offset, limit))
                .map(|v| ("m_AsymptoteValue".to_string(), v)),
            ((self.stiffness_factor).inspect_value(path, offset, limit))
                .map(|v| ("stiffnessFactor".to_string(), v)),
            ((self.m_stiffness).inspect_value(path, offset, limit))
                .map(|v| ("m_Stiffness".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
