//! 自动生成。请勿手动修改。

use crate::subclasses::error_log::ErrorLog;
use crate::subclasses::reflected_function::ReflectedFunction;

#[derive(Debug, Clone)]
pub struct ShaderIncludeReflection {
    pub m_functions: Vec<ReflectedFunction>,
    pub m_reflection_log: ErrorLog,
}

impl crate::Parse for ShaderIncludeReflection {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_functions = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ReflectedFunction::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_reflection_log = ErrorLog::parse(rdr, ver)?;

        Ok(ShaderIncludeReflection {
            m_functions,
            m_reflection_log,
        })
    }
}

impl crate::ToValue for ShaderIncludeReflection {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Functions".to_string(), self.m_functions.to_value())),
            Some((
                "m_ReflectionLog".to_string(),
                self.m_reflection_log.to_value(),
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
                "m_Functions" => (self.m_functions).inspect_value(path, offset, limit),
                "m_ReflectionLog" => (self.m_reflection_log).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_functions).inspect_value(path, offset, limit))
                .map(|v| ("m_Functions".to_string(), v)),
            ((self.m_reflection_log).inspect_value(path, offset, limit))
                .map(|v| ("m_ReflectionLog".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
