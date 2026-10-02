//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_kernel::ComputeShaderKernel;

#[derive(Debug, Clone)]
pub struct ComputeShaderKernelParent {
    pub name: String,
    pub variant_map: Option<Vec<(String, ComputeShaderKernel)>>,
    pub unique_variants: Option<Vec<ComputeShaderKernel>>,
    pub valid_keywords: Option<Vec<String>>,
    pub global_keywords: Option<Vec<String>>,
    pub variant_indices: Option<Vec<(String, u32)>>,
    pub local_keywords: Option<Vec<String>>,
    pub dynamic_keywords: Option<Vec<String>>,
}

impl crate::Parse for ComputeShaderKernelParent {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut variant_map = None;
        let mut unique_variants = None;
        let mut valid_keywords = None;
        let mut global_keywords = None;
        let mut variant_indices = None;
        let mut local_keywords = None;
        let mut dynamic_keywords = None;
        name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            unique_variants = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| ComputeShaderKernel::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            variant_indices = Some(
                ({
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
                })?,
            );
            rdr.align(4)?;
            global_keywords = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            local_keywords = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            dynamic_keywords = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            variant_map = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = ComputeShaderKernel::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            global_keywords = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            local_keywords = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
            variant_map = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = ComputeShaderKernel::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            valid_keywords = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        }

        Ok(ComputeShaderKernelParent {
            name: name.unwrap(),
            variant_map,
            unique_variants,
            valid_keywords,
            global_keywords,
            variant_indices,
            local_keywords,
            dynamic_keywords,
        })
    }
}

impl crate::ToValue for ComputeShaderKernelParent {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            crate::opt_field("variantMap", &self.variant_map),
            crate::opt_field("uniqueVariants", &self.unique_variants),
            crate::opt_field("validKeywords", &self.valid_keywords),
            crate::opt_field("globalKeywords", &self.global_keywords),
            crate::opt_field("variantIndices", &self.variant_indices),
            crate::opt_field("localKeywords", &self.local_keywords),
            crate::opt_field("dynamicKeywords", &self.dynamic_keywords),
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
                "variantMap" => (self.variant_map).inspect_value(path, offset, limit),
                "uniqueVariants" => (self.unique_variants).inspect_value(path, offset, limit),
                "validKeywords" => (self.valid_keywords).inspect_value(path, offset, limit),
                "globalKeywords" => (self.global_keywords).inspect_value(path, offset, limit),
                "variantIndices" => (self.variant_indices).inspect_value(path, offset, limit),
                "localKeywords" => (self.local_keywords).inspect_value(path, offset, limit),
                "dynamicKeywords" => (self.dynamic_keywords).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.variant_map).inspect_value(path, offset, limit))
                .map(|v| ("variantMap".to_string(), v)),
            ((self.unique_variants).inspect_value(path, offset, limit))
                .map(|v| ("uniqueVariants".to_string(), v)),
            ((self.valid_keywords).inspect_value(path, offset, limit))
                .map(|v| ("validKeywords".to_string(), v)),
            ((self.global_keywords).inspect_value(path, offset, limit))
                .map(|v| ("globalKeywords".to_string(), v)),
            ((self.variant_indices).inspect_value(path, offset, limit))
                .map(|v| ("variantIndices".to_string(), v)),
            ((self.local_keywords).inspect_value(path, offset, limit))
                .map(|v| ("localKeywords".to_string(), v)),
            ((self.dynamic_keywords).inspect_value(path, offset, limit))
                .map(|v| ("dynamicKeywords".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
