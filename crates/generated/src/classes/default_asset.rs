//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct DefaultAsset {
    pub m_name: String,
    pub m_message: Option<String>,
    pub m_is_warning: Option<bool>,
}

impl crate::Parse for DefaultAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_message = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_aligned_string()?)
        } else {
            None
        };
        let m_is_warning = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };

        Ok(DefaultAsset {
            m_name,
            m_message,
            m_is_warning,
        })
    }
}

impl crate::ToValue for DefaultAsset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_Message", &self.m_message),
            crate::opt_field("m_IsWarning", &self.m_is_warning),
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
                "m_Message" => (self.m_message).inspect_value(path, offset, limit),
                "m_IsWarning" => (self.m_is_warning).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_message).inspect_value(path, offset, limit))
                .map(|v| ("m_Message".to_string(), v)),
            ((self.m_is_warning).inspect_value(path, offset, limit))
                .map(|v| ("m_IsWarning".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
