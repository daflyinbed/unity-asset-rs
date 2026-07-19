//! 自动生成。请勿手动修改。

use crate::subclasses::hint::Hint;
use crate::subclasses::reflected_parameter::ReflectedParameter;

#[derive(Debug, Clone)]
pub struct ReflectedFunction {
    pub m_namespace: Vec<String>,
    pub m_return_type_name: String,
    pub m_name: String,
    pub m_parameters: Vec<ReflectedParameter>,
    pub m_body: String,
    pub m_hints: Vec<Hint>,
}

impl crate::Parse for ReflectedFunction {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_namespace = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_return_type_name = rdr.read_aligned_string()?;
        let m_name = rdr.read_aligned_string()?;
        let m_parameters = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ReflectedParameter::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_body = rdr.read_aligned_string()?;
        let m_hints = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Hint::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(ReflectedFunction {
            m_namespace,
            m_return_type_name,
            m_name,
            m_parameters,
            m_body,
            m_hints,
        })
    }
}
