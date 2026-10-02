//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::shader_info::ShaderInfo;

#[derive(Debug, Clone)]
pub struct ShaderVariantCollection {
    pub m_name: String,
    pub m_shaders: Vec<(PPtr, ShaderInfo)>,
}

impl crate::Parse for ShaderVariantCollection {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_shaders = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = crate::PPtr::parse(rdr, ver)?;
                        let b = ShaderInfo::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(ShaderVariantCollection { m_name, m_shaders })
    }
}

impl crate::ToValue for ShaderVariantCollection {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Shaders".to_string(), self.m_shaders.to_value())),
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
                "m_Shaders" => (self.m_shaders).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_shaders).inspect_value(path, offset, limit))
                .map(|v| ("m_Shaders".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
