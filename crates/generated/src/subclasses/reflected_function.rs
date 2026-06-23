//! 自动生成。请勿手动修改。

use crate::subclasses::hint::Hint;
use crate::subclasses::reflected_parameter::ReflectedParameter;

#[derive(Debug, Clone)]
pub struct ReflectedFunction {
    pub m_return_type_name: String,
    pub m_name: String,
    pub m_parameters: Vec<ReflectedParameter>,
    pub m_hints: Vec<Hint>,
}

impl crate::Parse for ReflectedFunction {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_return_type_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_parameters = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ReflectedParameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_hints = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Hint::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ReflectedFunction {
            m_return_type_name,
            m_name,
            m_parameters,
            m_hints,
        })
    }
}
