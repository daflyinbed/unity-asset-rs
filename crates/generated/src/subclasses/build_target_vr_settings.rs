//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetVrSettings {
    pub m_build_target: String,
    pub m_enabled: bool,
    pub m_devices: Vec<String>,
}

impl crate::Parse for BuildTargetVrSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let m_devices = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(BuildTargetVrSettings {
            m_build_target,
            m_enabled,
            m_devices,
        })
    }
}

impl crate::ToValue for BuildTargetVrSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BuildTarget".to_string(), self.m_build_target.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Devices".to_string(), self.m_devices.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BuildTarget" => (self.m_build_target).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_Devices" => (self.m_devices).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTarget".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_devices).inspect_value(path, offset, limit))
                .map(|v| ("m_Devices".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
