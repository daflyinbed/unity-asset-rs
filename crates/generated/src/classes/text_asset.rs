//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TextAsset {
    pub m_name: String,
    pub m_script: String,
    pub m_path_name: Option<String>,
}

impl crate::Parse for TextAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_script = None;
        let mut m_path_name = None;
        m_name = Some(rdr.read_aligned_string()?);
        m_script = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
        } else {
            m_path_name = Some(rdr.read_aligned_string()?);
        }

        Ok(TextAsset {
            m_name: m_name.unwrap(),
            m_script: m_script.unwrap(),
            m_path_name,
        })
    }
}

impl crate::ToValue for TextAsset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Script".to_string(), self.m_script.to_value())),
            crate::opt_field("m_PathName", &self.m_path_name),
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
                "m_Script" => (self.m_script).inspect_value(path, offset, limit),
                "m_PathName" => (self.m_path_name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_script).inspect_value(path, offset, limit))
                .map(|v| ("m_Script".to_string(), v)),
            ((self.m_path_name).inspect_value(path, offset, limit))
                .map(|v| ("m_PathName".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
