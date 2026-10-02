//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LuminVersionInfo {
    pub m_version_code: i32,
    pub m_version_name: String,
}

impl crate::Parse for LuminVersionInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_version_code = rdr.read_i32()?;
        let m_version_name = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(LuminVersionInfo {
            m_version_code,
            m_version_name,
        })
    }
}

impl crate::ToValue for LuminVersionInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_VersionCode".to_string(), self.m_version_code.to_value())),
            Some(("m_VersionName".to_string(), self.m_version_name.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_VersionCode" => (self.m_version_code).inspect_value(path, offset, limit),
                "m_VersionName" => (self.m_version_name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_version_code).inspect_value(path, offset, limit))
                .map(|v| ("m_VersionCode".to_string(), v)),
            ((self.m_version_name).inspect_value(path, offset, limit))
                .map(|v| ("m_VersionName".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
