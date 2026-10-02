//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct JointTranslationLimits2D {
    pub m_lower_translation: f32,
    pub m_upper_translation: f32,
}

impl crate::Parse for JointTranslationLimits2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_lower_translation = rdr.read_f32()?;
        let m_upper_translation = rdr.read_f32()?;

        Ok(JointTranslationLimits2D {
            m_lower_translation,
            m_upper_translation,
        })
    }
}

impl crate::ToValue for JointTranslationLimits2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_LowerTranslation".to_string(),
                self.m_lower_translation.to_value(),
            )),
            Some((
                "m_UpperTranslation".to_string(),
                self.m_upper_translation.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_LowerTranslation" => {
                    (self.m_lower_translation).inspect_value(path, offset, limit)
                }
                "m_UpperTranslation" => {
                    (self.m_upper_translation).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_lower_translation).inspect_value(path, offset, limit))
                .map(|v| ("m_LowerTranslation".to_string(), v)),
            ((self.m_upper_translation).inspect_value(path, offset, limit))
                .map(|v| ("m_UpperTranslation".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
