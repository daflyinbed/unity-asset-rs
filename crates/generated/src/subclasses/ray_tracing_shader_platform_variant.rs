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
    pub compiler_platform: i32,
}

impl crate::Parse for RayTracingShaderPlatformVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_renderer = r.read_i32()?;
        let unique_variants = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { RayTracingShaderReflectionData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let variant_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_u32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let global_keywords = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let local_keywords = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let dynamic_keywords = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let editor_only_variant = r.read_bool()?;
        r.align(4)?;
        let compiler_platform = r.read_i32()?;

        Ok(RayTracingShaderPlatformVariant {
            target_renderer,
            unique_variants,
            variant_indices,
            global_keywords,
            local_keywords,
            dynamic_keywords,
            editor_only_variant,
            compiler_platform,
        })
    }
}
