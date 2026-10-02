//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_function_desc::RayTracingShaderFunctionDesc;
use crate::subclasses::ray_tracing_shader_resources::RayTracingShaderResources;

#[derive(Debug, Clone)]
pub struct RayTracingShaderReflectionData {
    pub functions: Vec<RayTracingShaderFunctionDesc>,
    pub local_resources: RayTracingShaderResources,
    pub global_resources: RayTracingShaderResources,
    pub has_errors: bool,
    pub code: Vec<u8>,
    pub precompiled: Option<Vec<u8>>,
    pub requirements: Option<i64>,
    pub code_hash: Option<u32>,
}

impl crate::Parse for RayTracingShaderReflectionData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let functions = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| RayTracingShaderFunctionDesc::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let local_resources = RayTracingShaderResources::parse(rdr, ver)?;
        let global_resources = RayTracingShaderResources::parse(rdr, ver)?;
        let has_errors = rdr.read_bool()?;
        rdr.align(4)?;
        let code = ({
            let n = rdr.read_i32()? as usize;
            (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let precompiled = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let requirements = if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            Some(rdr.read_i64()?)
        } else {
            None
        };
        let code_hash = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };

        Ok(RayTracingShaderReflectionData {
            functions,
            local_resources,
            global_resources,
            has_errors,
            code,
            precompiled,
            requirements,
            code_hash,
        })
    }
}

impl crate::ToValue for RayTracingShaderReflectionData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("functions".to_string(), self.functions.to_value())),
            Some((
                "localResources".to_string(),
                self.local_resources.to_value(),
            )),
            Some((
                "globalResources".to_string(),
                self.global_resources.to_value(),
            )),
            Some(("hasErrors".to_string(), self.has_errors.to_value())),
            Some(("code".to_string(), self.code.to_value())),
            crate::opt_field("precompiled", &self.precompiled),
            crate::opt_field("requirements", &self.requirements),
            crate::opt_field("codeHash", &self.code_hash),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "functions" => (self.functions).inspect_value(path, offset, limit),
                "localResources" => (self.local_resources).inspect_value(path, offset, limit),
                "globalResources" => (self.global_resources).inspect_value(path, offset, limit),
                "hasErrors" => (self.has_errors).inspect_value(path, offset, limit),
                "code" => (self.code).inspect_value(path, offset, limit),
                "precompiled" => (self.precompiled).inspect_value(path, offset, limit),
                "requirements" => (self.requirements).inspect_value(path, offset, limit),
                "codeHash" => (self.code_hash).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.functions).inspect_value(path, offset, limit))
                .map(|v| ("functions".to_string(), v)),
            ((self.local_resources).inspect_value(path, offset, limit))
                .map(|v| ("localResources".to_string(), v)),
            ((self.global_resources).inspect_value(path, offset, limit))
                .map(|v| ("globalResources".to_string(), v)),
            ((self.has_errors).inspect_value(path, offset, limit))
                .map(|v| ("hasErrors".to_string(), v)),
            ((self.code).inspect_value(path, offset, limit)).map(|v| ("code".to_string(), v)),
            ((self.precompiled).inspect_value(path, offset, limit))
                .map(|v| ("precompiled".to_string(), v)),
            ((self.requirements).inspect_value(path, offset, limit))
                .map(|v| ("requirements".to_string(), v)),
            ((self.code_hash).inspect_value(path, offset, limit))
                .map(|v| ("codeHash".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
