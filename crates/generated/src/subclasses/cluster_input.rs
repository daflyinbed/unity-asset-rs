//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ClusterInput {
    pub m_name: String,
    pub m_device_name: String,
    pub m_server_url: String,
    pub m_index: i32,
    pub m_type: i32,
}

impl crate::Parse for ClusterInput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_device_name = rdr.read_aligned_string()?;
        let m_server_url = rdr.read_aligned_string()?;
        let m_index = rdr.read_i32()?;
        let m_type = rdr.read_i32()?;

        Ok(ClusterInput {
            m_name,
            m_device_name,
            m_server_url,
            m_index,
            m_type,
        })
    }
}

impl crate::ToValue for ClusterInput {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_DeviceName".to_string(), self.m_device_name.to_value())),
            Some(("m_ServerUrl".to_string(), self.m_server_url.to_value())),
            Some(("m_Index".to_string(), self.m_index.to_value())),
            Some(("m_Type".to_string(), self.m_type.to_value())),
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
                "m_DeviceName" => (self.m_device_name).inspect_value(path, offset, limit),
                "m_ServerUrl" => (self.m_server_url).inspect_value(path, offset, limit),
                "m_Index" => (self.m_index).inspect_value(path, offset, limit),
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_device_name).inspect_value(path, offset, limit))
                .map(|v| ("m_DeviceName".to_string(), v)),
            ((self.m_server_url).inspect_value(path, offset, limit))
                .map(|v| ("m_ServerUrl".to_string(), v)),
            ((self.m_index).inspect_value(path, offset, limit)).map(|v| ("m_Index".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
