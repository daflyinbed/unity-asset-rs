//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_package_requirement::SerializedPackageRequirement;

#[derive(Debug, Clone)]
pub struct SerializedPackageRequirements {
    pub m_requirements: Vec<SerializedPackageRequirement>,
    pub m_status_message: String,
    pub m_combined_status: i8,
}

impl crate::Parse for SerializedPackageRequirements {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_requirements = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| SerializedPackageRequirement::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_status_message = rdr.read_aligned_string()?;
        let m_combined_status = rdr.read_i8()?;

        Ok(SerializedPackageRequirements {
            m_requirements,
            m_status_message,
            m_combined_status,
        })
    }
}

impl crate::ToValue for SerializedPackageRequirements {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Requirements".to_string(), self.m_requirements.to_value())),
            Some((
                "m_StatusMessage".to_string(),
                self.m_status_message.to_value(),
            )),
            Some((
                "m_CombinedStatus".to_string(),
                self.m_combined_status.to_value(),
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
                "m_Requirements" => (self.m_requirements).inspect_value(path, offset, limit),
                "m_StatusMessage" => (self.m_status_message).inspect_value(path, offset, limit),
                "m_CombinedStatus" => (self.m_combined_status).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_requirements).inspect_value(path, offset, limit))
                .map(|v| ("m_Requirements".to_string(), v)),
            ((self.m_status_message).inspect_value(path, offset, limit))
                .map(|v| ("m_StatusMessage".to_string(), v)),
            ((self.m_combined_status).inspect_value(path, offset, limit))
                .map(|v| ("m_CombinedStatus".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
