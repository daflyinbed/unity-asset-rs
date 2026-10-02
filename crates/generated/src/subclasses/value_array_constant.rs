//! 自动生成。请勿手动修改。

use crate::subclasses::value_constant::ValueConstant;

#[derive(Debug, Clone)]
pub struct ValueArrayConstant {
    pub m_value_array: Vec<ValueConstant>,
}

impl crate::Parse for ValueArrayConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_value_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ValueConstant::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(ValueArrayConstant { m_value_array })
    }
}

impl crate::ToValue for ValueArrayConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "m_ValueArray".to_string(),
            self.m_value_array.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_ValueArray" => (self.m_value_array).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_value_array).inspect_value(path, offset, limit))
                .map(|v| ("m_ValueArray".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
