//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_param::RayTracingShaderParam;

#[derive(Debug, Clone)]
pub struct RayTracingShaderConstantBuffer {
    pub name: String,
    pub byte_size: i32,
    pub params: Vec<RayTracingShaderParam>,
    pub hash: Option<u32>,
}

impl crate::Parse for RayTracingShaderConstantBuffer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut byte_size = None;
        let mut params = None;
        let mut hash = None;
        name = Some(rdr.read_aligned_string()?);
        byte_size = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            hash = Some(rdr.read_u32()?);
        } else {
        }
        params = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| RayTracingShaderParam::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;

        Ok(RayTracingShaderConstantBuffer {
            name: name.unwrap(),
            byte_size: byte_size.unwrap(),
            params: params.unwrap(),
            hash,
        })
    }
}

impl crate::ToValue for RayTracingShaderConstantBuffer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("byteSize".to_string(), self.byte_size.to_value())),
            Some(("params".to_string(), self.params.to_value())),
            crate::opt_field("hash", &self.hash),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "byteSize" => (self.byte_size).inspect_value(path, offset, limit),
                "params" => (self.params).inspect_value(path, offset, limit),
                "hash" => (self.hash).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.byte_size).inspect_value(path, offset, limit))
                .map(|v| ("byteSize".to_string(), v)),
            ((self.params).inspect_value(path, offset, limit)).map(|v| ("params".to_string(), v)),
            ((self.hash).inspect_value(path, offset, limit)).map(|v| ("hash".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
