//! 自动生成。请勿手动修改。

use crate::subclasses::value_constant::ValueConstant;

#[derive(Debug, Clone)]
pub struct ValueArrayConstant {
    pub m_value_array: Vec<ValueConstant>,
}

impl crate::Parse for ValueArrayConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_value_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ValueConstant::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(ValueArrayConstant {
            m_value_array,
        })
    }
}
