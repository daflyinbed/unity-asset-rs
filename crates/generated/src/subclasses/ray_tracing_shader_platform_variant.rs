//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_reflection_data::RayTracingShaderReflectionData;

#[derive(Debug, Clone)]
pub struct RayTracingShaderPlatformVariant {
    pub target_renderer: i32,
    pub unique_variants: Vec<RayTracingShaderReflectionData>,
    pub variant_indices: Vec<(String, u32)>,
    pub global_keywords: Vec<String>,
    pub local_keywords: Vec<String>,
    pub dynamic_keywords: Vec<String>,
    pub editor_only_variant: bool,
}

impl crate::Parse for RayTracingShaderPlatformVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_renderer = rdr.read_i32()?;
        let unique_variants = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| RayTracingShaderReflectionData::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let variant_indices = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_aligned_string()?;
                        let b = rdr.read_u32()?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let global_keywords = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let local_keywords = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let dynamic_keywords = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let editor_only_variant = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(RayTracingShaderPlatformVariant {
            target_renderer,
            unique_variants,
            variant_indices,
            global_keywords,
            local_keywords,
            dynamic_keywords,
            editor_only_variant,
        })
    }
}

impl crate::ToValue for RayTracingShaderPlatformVariant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "targetRenderer".to_string(),
                self.target_renderer.to_value(),
            )),
            Some((
                "uniqueVariants".to_string(),
                self.unique_variants.to_value(),
            )),
            Some((
                "variantIndices".to_string(),
                self.variant_indices.to_value(),
            )),
            Some((
                "globalKeywords".to_string(),
                self.global_keywords.to_value(),
            )),
            Some(("localKeywords".to_string(), self.local_keywords.to_value())),
            Some((
                "dynamicKeywords".to_string(),
                self.dynamic_keywords.to_value(),
            )),
            Some((
                "editorOnlyVariant".to_string(),
                self.editor_only_variant.to_value(),
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
                "targetRenderer" => (self.target_renderer).inspect_value(path, offset, limit),
                "uniqueVariants" => (self.unique_variants).inspect_value(path, offset, limit),
                "variantIndices" => (self.variant_indices).inspect_value(path, offset, limit),
                "globalKeywords" => (self.global_keywords).inspect_value(path, offset, limit),
                "localKeywords" => (self.local_keywords).inspect_value(path, offset, limit),
                "dynamicKeywords" => (self.dynamic_keywords).inspect_value(path, offset, limit),
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
            ((self.unique_variants).inspect_value(path, offset, limit))
                .map(|v| ("uniqueVariants".to_string(), v)),
            ((self.variant_indices).inspect_value(path, offset, limit))
                .map(|v| ("variantIndices".to_string(), v)),
            ((self.global_keywords).inspect_value(path, offset, limit))
                .map(|v| ("globalKeywords".to_string(), v)),
            ((self.local_keywords).inspect_value(path, offset, limit))
                .map(|v| ("localKeywords".to_string(), v)),
            ((self.dynamic_keywords).inspect_value(path, offset, limit))
                .map(|v| ("dynamicKeywords".to_string(), v)),
            ((self.editor_only_variant).inspect_value(path, offset, limit))
                .map(|v| ("editorOnlyVariant".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
