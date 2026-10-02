//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct InputAxis {
    pub m_name: String,
    pub descriptive_name: String,
    pub descriptive_negative_name: String,
    pub negative_button: String,
    pub positive_button: String,
    pub alt_negative_button: String,
    pub alt_positive_button: String,
    pub gravity: f32,
    pub dead: f32,
    pub sensitivity: f32,
    pub snap: bool,
    pub invert: bool,
    pub r#type: i32,
    pub axis: i32,
    pub joy_num: i32,
}

impl crate::Parse for InputAxis {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let descriptive_name = rdr.read_aligned_string()?;
        let descriptive_negative_name = rdr.read_aligned_string()?;
        let negative_button = rdr.read_aligned_string()?;
        let positive_button = rdr.read_aligned_string()?;
        let alt_negative_button = rdr.read_aligned_string()?;
        let alt_positive_button = rdr.read_aligned_string()?;
        let gravity = rdr.read_f32()?;
        let dead = rdr.read_f32()?;
        let sensitivity = rdr.read_f32()?;
        let snap = rdr.read_bool()?;
        let invert = rdr.read_bool()?;
        rdr.align(4)?;
        let r#type = rdr.read_i32()?;
        let axis = rdr.read_i32()?;
        let joy_num = rdr.read_i32()?;

        Ok(InputAxis {
            m_name,
            descriptive_name,
            descriptive_negative_name,
            negative_button,
            positive_button,
            alt_negative_button,
            alt_positive_button,
            gravity,
            dead,
            sensitivity,
            snap,
            invert,
            r#type,
            axis,
            joy_num,
        })
    }
}

impl crate::ToValue for InputAxis {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "descriptiveName".to_string(),
                self.descriptive_name.to_value(),
            )),
            Some((
                "descriptiveNegativeName".to_string(),
                self.descriptive_negative_name.to_value(),
            )),
            Some((
                "negativeButton".to_string(),
                self.negative_button.to_value(),
            )),
            Some((
                "positiveButton".to_string(),
                self.positive_button.to_value(),
            )),
            Some((
                "altNegativeButton".to_string(),
                self.alt_negative_button.to_value(),
            )),
            Some((
                "altPositiveButton".to_string(),
                self.alt_positive_button.to_value(),
            )),
            Some(("gravity".to_string(), self.gravity.to_value())),
            Some(("dead".to_string(), self.dead.to_value())),
            Some(("sensitivity".to_string(), self.sensitivity.to_value())),
            Some(("snap".to_string(), self.snap.to_value())),
            Some(("invert".to_string(), self.invert.to_value())),
            Some(("type".to_string(), self.r#type.to_value())),
            Some(("axis".to_string(), self.axis.to_value())),
            Some(("joyNum".to_string(), self.joy_num.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "descriptiveName" => (self.descriptive_name).inspect_value(path, offset, limit),
                "descriptiveNegativeName" => {
                    (self.descriptive_negative_name).inspect_value(path, offset, limit)
                }
                "negativeButton" => (self.negative_button).inspect_value(path, offset, limit),
                "positiveButton" => (self.positive_button).inspect_value(path, offset, limit),
                "altNegativeButton" => {
                    (self.alt_negative_button).inspect_value(path, offset, limit)
                }
                "altPositiveButton" => {
                    (self.alt_positive_button).inspect_value(path, offset, limit)
                }
                "gravity" => (self.gravity).inspect_value(path, offset, limit),
                "dead" => (self.dead).inspect_value(path, offset, limit),
                "sensitivity" => (self.sensitivity).inspect_value(path, offset, limit),
                "snap" => (self.snap).inspect_value(path, offset, limit),
                "invert" => (self.invert).inspect_value(path, offset, limit),
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "axis" => (self.axis).inspect_value(path, offset, limit),
                "joyNum" => (self.joy_num).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.descriptive_name).inspect_value(path, offset, limit))
                .map(|v| ("descriptiveName".to_string(), v)),
            ((self.descriptive_negative_name).inspect_value(path, offset, limit))
                .map(|v| ("descriptiveNegativeName".to_string(), v)),
            ((self.negative_button).inspect_value(path, offset, limit))
                .map(|v| ("negativeButton".to_string(), v)),
            ((self.positive_button).inspect_value(path, offset, limit))
                .map(|v| ("positiveButton".to_string(), v)),
            ((self.alt_negative_button).inspect_value(path, offset, limit))
                .map(|v| ("altNegativeButton".to_string(), v)),
            ((self.alt_positive_button).inspect_value(path, offset, limit))
                .map(|v| ("altPositiveButton".to_string(), v)),
            ((self.gravity).inspect_value(path, offset, limit)).map(|v| ("gravity".to_string(), v)),
            ((self.dead).inspect_value(path, offset, limit)).map(|v| ("dead".to_string(), v)),
            ((self.sensitivity).inspect_value(path, offset, limit))
                .map(|v| ("sensitivity".to_string(), v)),
            ((self.snap).inspect_value(path, offset, limit)).map(|v| ("snap".to_string(), v)),
            ((self.invert).inspect_value(path, offset, limit)).map(|v| ("invert".to_string(), v)),
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.axis).inspect_value(path, offset, limit)).map(|v| ("axis".to_string(), v)),
            ((self.joy_num).inspect_value(path, offset, limit)).map(|v| ("joyNum".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
