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
        let m_functions = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ReflectedFunction::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_reflection_log = ErrorLog::parse(rdr, ver)?;

        Ok(ShaderIncludeReflection {
            m_functions,
            m_reflection_log,
        })
    }
}
