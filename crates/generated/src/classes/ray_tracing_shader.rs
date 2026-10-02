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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| RayTracingShaderPlatformVariant::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V6000_3(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| RayTracingShaderVariant::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RayTracingShaderVariantsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_3(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for RayTracingShaderMEnableRayPayloadSizeChecksVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RayTracingShader {
    pub m_name: String,
    pub variants: RayTracingShaderVariantsVariant,
    pub m_max_recursion_depth: u32,
    pub m_enable_ray_payload_size_checks:
        Option<RayTracingShaderMEnableRayPayloadSizeChecksVariant>,
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
            Some(RayTracingShaderMEnableRayPayloadSizeChecksVariant::parse(
                rdr, ver,
            )?)
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

impl crate::ToValue for RayTracingShader {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("variants".to_string(), self.variants.to_value())),
            Some((
                "m_MaxRecursionDepth".to_string(),
                self.m_max_recursion_depth.to_value(),
            )),
            crate::opt_field(
                "m_EnableRayPayloadSizeChecks",
                &self.m_enable_ray_payload_size_checks,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "variants" => (self.variants).inspect_value(path, offset, limit),
                "m_MaxRecursionDepth" => {
                    (self.m_max_recursion_depth).inspect_value(path, offset, limit)
                }
                "m_EnableRayPayloadSizeChecks" => {
                    (self.m_enable_ray_payload_size_checks).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.variants).inspect_value(path, offset, limit))
                .map(|v| ("variants".to_string(), v)),
            ((self.m_max_recursion_depth).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxRecursionDepth".to_string(), v)),
            ((self.m_enable_ray_payload_size_checks).inspect_value(path, offset, limit))
                .map(|v| ("m_EnableRayPayloadSizeChecks".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
