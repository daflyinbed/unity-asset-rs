//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct XboxOneSocketDefinition {
    pub m_name: String,
    pub m_port: String,
    pub m_protocol: i32,
    pub m_usages: Vec<i32>,
    pub m_template_name: String,
    pub m_session_requirment: i32,
    pub m_device_usages: Vec<i32>,
}

impl crate::Parse for XboxOneSocketDefinition {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_port = rdr.read_aligned_string()?;
        let m_protocol = rdr.read_i32()?;
        let m_usages = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_template_name = rdr.read_aligned_string()?;
        let m_session_requirment = rdr.read_i32()?;
        let m_device_usages = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(XboxOneSocketDefinition {
            m_name,
            m_port,
            m_protocol,
            m_usages,
            m_template_name,
            m_session_requirment,
            m_device_usages,
        })
    }
}

impl crate::ToValue for XboxOneSocketDefinition {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Port".to_string(), self.m_port.to_value())),
            Some(("m_Protocol".to_string(), self.m_protocol.to_value())),
            Some(("m_Usages".to_string(), self.m_usages.to_value())),
            Some((
                "m_TemplateName".to_string(),
                self.m_template_name.to_value(),
            )),
            Some((
                "m_SessionRequirment".to_string(),
                self.m_session_requirment.to_value(),
            )),
            Some((
                "m_DeviceUsages".to_string(),
                self.m_device_usages.to_value(),
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
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Port" => (self.m_port).inspect_value(path, offset, limit),
                "m_Protocol" => (self.m_protocol).inspect_value(path, offset, limit),
                "m_Usages" => (self.m_usages).inspect_value(path, offset, limit),
                "m_TemplateName" => (self.m_template_name).inspect_value(path, offset, limit),
                "m_SessionRequirment" => {
                    (self.m_session_requirment).inspect_value(path, offset, limit)
                }
                "m_DeviceUsages" => (self.m_device_usages).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_port).inspect_value(path, offset, limit)).map(|v| ("m_Port".to_string(), v)),
            ((self.m_protocol).inspect_value(path, offset, limit))
                .map(|v| ("m_Protocol".to_string(), v)),
            ((self.m_usages).inspect_value(path, offset, limit))
                .map(|v| ("m_Usages".to_string(), v)),
            ((self.m_template_name).inspect_value(path, offset, limit))
                .map(|v| ("m_TemplateName".to_string(), v)),
            ((self.m_session_requirment).inspect_value(path, offset, limit))
                .map(|v| ("m_SessionRequirment".to_string(), v)),
            ((self.m_device_usages).inspect_value(path, offset, limit))
                .map(|v| ("m_DeviceUsages".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
