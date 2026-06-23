//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::ray_tracing_shader_variant::RayTracingShaderVariant;
use crate::subclasses::shader_error::ShaderError;

#[derive(Debug, Clone)]
pub struct RayTracingShader {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_name: String,
    pub variants: Vec<RayTracingShaderVariant>,
    pub m_max_recursion_depth: u32,
    pub errors: Vec<ShaderError>,
}

impl crate::Parse for RayTracingShader {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let variants = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { RayTracingShaderVariant::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_max_recursion_depth = r.read_u32()?;
        let errors = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ShaderError::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(RayTracingShader {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_name,
            variants,
            m_max_recursion_depth,
            errors,
        })
    }
}
