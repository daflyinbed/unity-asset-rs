//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_reflection_data::RayTracingShaderReflectionData;

#[derive(Debug, Clone)]
pub struct RayTracingShaderVariant {
    pub target_renderer: i32,
    pub resource_reflection_data: RayTracingShaderReflectionData,
    pub editor_only_variant: Option<bool>,
}

impl crate::Parse for RayTracingShaderVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_renderer = rdr.read_i32()?;
        let resource_reflection_data = RayTracingShaderReflectionData::parse(rdr, ver)?;
        rdr.align(4)?;
        let editor_only_variant = if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(RayTracingShaderVariant {
            target_renderer,
            resource_reflection_data,
            editor_only_variant,
        })
    }
}

impl crate::ToValue for RayTracingShaderVariant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "targetRenderer".to_string(),
                self.target_renderer.to_value(),
            )),
            Some((
                "resourceReflectionData".to_string(),
                self.resource_reflection_data.to_value(),
            )),
            crate::opt_field("editorOnlyVariant", &self.editor_only_variant),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "targetRenderer" => (self.target_renderer).inspect_value(path, offset, limit),
                "resourceReflectionData" => {
                    (self.resource_reflection_data).inspect_value(path, offset, limit)
                }
                "editorOnlyVariant" => {
                    (self.editor_only_variant).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.target_renderer).inspect_value(path, offset, limit))
                .map(|v| ("targetRenderer".to_string(), v)),
            ((self.resource_reflection_data).inspect_value(path, offset, limit))
                .map(|v| ("resourceReflectionData".to_string(), v)),
            ((self.editor_only_variant).inspect_value(path, offset, limit))
                .map(|v| ("editorOnlyVariant".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
