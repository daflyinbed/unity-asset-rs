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

impl crate::ToValue for ReflectedFunction {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Namespace".to_string(), self.m_namespace.to_value())),
            Some((
                "m_ReturnTypeName".to_string(),
                self.m_return_type_name.to_value(),
            )),
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Parameters".to_string(), self.m_parameters.to_value())),
            Some(("m_Body".to_string(), self.m_body.to_value())),
            Some(("m_Hints".to_string(), self.m_hints.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Namespace" => (self.m_namespace).inspect_value(path, offset, limit),
                "m_ReturnTypeName" => (self.m_return_type_name).inspect_value(path, offset, limit),
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Parameters" => (self.m_parameters).inspect_value(path, offset, limit),
                "m_Body" => (self.m_body).inspect_value(path, offset, limit),
                "m_Hints" => (self.m_hints).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_namespace).inspect_value(path, offset, limit))
                .map(|v| ("m_Namespace".to_string(), v)),
            ((self.m_return_type_name).inspect_value(path, offset, limit))
                .map(|v| ("m_ReturnTypeName".to_string(), v)),
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_parameters).inspect_value(path, offset, limit))
                .map(|v| ("m_Parameters".to_string(), v)),
            ((self.m_body).inspect_value(path, offset, limit)).map(|v| ("m_Body".to_string(), v)),
            ((self.m_hints).inspect_value(path, offset, limit)).map(|v| ("m_Hints".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
