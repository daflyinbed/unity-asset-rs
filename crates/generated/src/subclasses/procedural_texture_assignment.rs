//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::fast_property_name::FastPropertyName;

/// 版本枚举：ProceduralTextureAssignmentShaderPropVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ProceduralTextureAssignmentShaderPropVariant {
    Default(Box<FastPropertyName>),
    V5_6(String),
}

impl crate::Parse for ProceduralTextureAssignmentShaderPropVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_aligned_string()?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({ Ok(Box::new(FastPropertyName::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ProceduralTextureAssignmentShaderPropVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProceduralTextureAssignment {
    pub shader_prop: ProceduralTextureAssignmentShaderPropVariant,
    pub material: PPtr,
    pub base_uid: u32,
}

impl crate::Parse for ProceduralTextureAssignment {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let shader_prop = ProceduralTextureAssignmentShaderPropVariant::parse(rdr, ver)?;
        let material = crate::PPtr::parse(rdr, ver)?;
        let base_uid = rdr.read_u32()?;

        Ok(ProceduralTextureAssignment {
            shader_prop,
            material,
            base_uid,
        })
    }
}

impl crate::ToValue for ProceduralTextureAssignment {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("shaderProp".to_string(), self.shader_prop.to_value())),
            Some(("material".to_string(), self.material.to_value())),
            Some(("baseUID".to_string(), self.base_uid.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "shaderProp" => (self.shader_prop).inspect_value(path, offset, limit),
                "material" => (self.material).inspect_value(path, offset, limit),
                "baseUID" => (self.base_uid).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.shader_prop).inspect_value(path, offset, limit))
                .map(|v| ("shaderProp".to_string(), v)),
            ((self.material).inspect_value(path, offset, limit))
                .map(|v| ("material".to_string(), v)),
            ((self.base_uid).inspect_value(path, offset, limit))
                .map(|v| ("baseUID".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
