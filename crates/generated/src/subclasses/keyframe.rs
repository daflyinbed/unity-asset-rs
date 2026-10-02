//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: f32,
    pub value: f32,
    pub in_slope: f32,
    pub out_slope: f32,
    pub weighted_mode: Option<i32>,
    pub in_weight: Option<f32>,
    pub out_weight: Option<f32>,
}

impl crate::Parse for Keyframe {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let time = rdr.read_f32()?;
        let value = rdr.read_f32()?;
        let in_slope = rdr.read_f32()?;
        let out_slope = rdr.read_f32()?;
        let weighted_mode = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let in_weight = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let out_weight = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(Keyframe {
            time,
            value,
            in_slope,
            out_slope,
            weighted_mode,
            in_weight,
            out_weight,
        })
    }
}

impl crate::ToValue for Keyframe {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("time".to_string(), self.time.to_value())),
            Some(("value".to_string(), self.value.to_value())),
            Some(("inSlope".to_string(), self.in_slope.to_value())),
            Some(("outSlope".to_string(), self.out_slope.to_value())),
            crate::opt_field("weightedMode", &self.weighted_mode),
            crate::opt_field("inWeight", &self.in_weight),
            crate::opt_field("outWeight", &self.out_weight),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "time" => (self.time).inspect_value(path, offset, limit),
                "value" => (self.value).inspect_value(path, offset, limit),
                "inSlope" => (self.in_slope).inspect_value(path, offset, limit),
                "outSlope" => (self.out_slope).inspect_value(path, offset, limit),
                "weightedMode" => (self.weighted_mode).inspect_value(path, offset, limit),
                "inWeight" => (self.in_weight).inspect_value(path, offset, limit),
                "outWeight" => (self.out_weight).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.time).inspect_value(path, offset, limit)).map(|v| ("time".to_string(), v)),
            ((self.value).inspect_value(path, offset, limit)).map(|v| ("value".to_string(), v)),
            ((self.in_slope).inspect_value(path, offset, limit))
                .map(|v| ("inSlope".to_string(), v)),
            ((self.out_slope).inspect_value(path, offset, limit))
                .map(|v| ("outSlope".to_string(), v)),
            ((self.weighted_mode).inspect_value(path, offset, limit))
                .map(|v| ("weightedMode".to_string(), v)),
            ((self.in_weight).inspect_value(path, offset, limit))
                .map(|v| ("inWeight".to_string(), v)),
            ((self.out_weight).inspect_value(path, offset, limit))
                .map(|v| ("outWeight".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
