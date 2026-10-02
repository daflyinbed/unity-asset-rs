//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_id::RayTracingShaderId;

#[derive(Debug, Clone)]
pub struct RayTracingShaderFunctionDesc {
    pub identifier: RayTracingShaderId,
    pub payload_size_in_bytes: u32,
    pub attribute_size_in_bytes: u32,
}

impl crate::Parse for RayTracingShaderFunctionDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let identifier = RayTracingShaderId::parse(rdr, ver)?;
        let payload_size_in_bytes = rdr.read_u32()?;
        let attribute_size_in_bytes = rdr.read_u32()?;

        Ok(RayTracingShaderFunctionDesc {
            identifier,
            payload_size_in_bytes,
            attribute_size_in_bytes,
        })
    }
}

impl crate::ToValue for RayTracingShaderFunctionDesc {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("identifier".to_string(), self.identifier.to_value())),
            Some((
                "payloadSizeInBytes".to_string(),
                self.payload_size_in_bytes.to_value(),
            )),
            Some((
                "attributeSizeInBytes".to_string(),
                self.attribute_size_in_bytes.to_value(),
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
                "identifier" => (self.identifier).inspect_value(path, offset, limit),
                "payloadSizeInBytes" => {
                    (self.payload_size_in_bytes).inspect_value(path, offset, limit)
                }
                "attributeSizeInBytes" => {
                    (self.attribute_size_in_bytes).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.identifier).inspect_value(path, offset, limit))
                .map(|v| ("identifier".to_string(), v)),
            ((self.payload_size_in_bytes).inspect_value(path, offset, limit))
                .map(|v| ("payloadSizeInBytes".to_string(), v)),
            ((self.attribute_size_in_bytes).inspect_value(path, offset, limit))
                .map(|v| ("attributeSizeInBytes".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
