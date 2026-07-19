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
