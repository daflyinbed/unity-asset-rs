//! 自动生成。请勿手动修改。

use crate::subclasses::error_log::ErrorLog;
use crate::subclasses::reflected_function::ReflectedFunction;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct ShaderIncludeReflection {
    pub parent: Object,
    pub m_functions: Vec<ReflectedFunction>,
    pub m_reflection_log: ErrorLog,
}

impl crate::Parse for ShaderIncludeReflection {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let m_functions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ReflectedFunction::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_reflection_log = ErrorLog::parse(r, ver)?;
        r.align(4)?;

        Ok(ShaderIncludeReflection {
            parent,
            m_functions,
            m_reflection_log,
        })
    }
}
