//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_platform_variant::RayTracingShaderPlatformVariant;
use crate::subclasses::ray_tracing_shader_variant::RayTracingShaderVariant;


/// 版本枚举：RayTracingShaderVariantsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RayTracingShaderVariantsVariant {
    Default(Vec<RayTracingShaderVariant>),
    V6000_3(Vec<RayTracingShaderPlatformVariant>),
}

impl crate::Parse for RayTracingShaderVariantsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { RayTracingShaderPlatformVariant::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V6000_3(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { RayTracingShaderVariant::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：RayTracingShaderMEnableRayPayloadSizeChecksVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RayTracingShaderMEnableRayPayloadSizeChecksVariant {
    Default(bool),
    V6000_3(bool),
}

impl crate::Parse for RayTracingShaderMEnableRayPayloadSizeChecksVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V6000_3(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct RayTracingShader {
    pub m_name: String,
    pub variants: RayTracingShaderVariantsVariant,
    pub m_max_recursion_depth: u32,
    pub m_enable_ray_payload_size_checks: Option<RayTracingShaderMEnableRayPayloadSizeChecksVariant>,
}

impl crate::Parse for RayTracingShader {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let variants = RayTracingShaderVariantsVariant::parse(rdr, ver)?;
        let m_max_recursion_depth = rdr.read_u32()?;
        let m_enable_ray_payload_size_checks = if *ver >= crate::UnityVersion::new(6000, 0, 17) {
            Some(RayTracingShaderMEnableRayPayloadSizeChecksVariant::parse(rdr, ver)?)
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
