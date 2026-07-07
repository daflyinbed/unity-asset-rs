//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_platform_variant::RayTracingShaderPlatformVariant;

#[derive(Debug, Clone)]
pub struct RayTracingShader {
    pub m_name: String,
    pub variants: Vec<RayTracingShaderPlatformVariant>,
    pub m_max_recursion_depth: u32,
    pub m_enable_ray_payload_size_checks: Option<bool>,
}

impl crate::Parse for RayTracingShader {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let variants = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { RayTracingShaderPlatformVariant::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_max_recursion_depth = r.read_u32()?;
        let m_enable_ray_payload_size_checks = if *ver >= crate::UnityVersion::new(6000, 0, 17) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(RayTracingShader {
            m_name,
            variants,
            m_max_recursion_depth,
            m_enable_ray_payload_size_checks,
        })
    }
}
