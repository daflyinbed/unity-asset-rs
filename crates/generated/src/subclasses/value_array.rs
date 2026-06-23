//! 自动生成。请勿手动修改。

use crate::subclasses::float4::float4;

#[derive(Debug, Clone)]
pub struct ValueArray {
    pub m_bool_values: Vec<bool>,
    pub m_int_values: Vec<i32>,
    pub m_float_values: Vec<f32>,
    pub m_vector_values: Vec<float4>,
}

impl crate::Parse for ValueArray {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_bool_values = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_int_values = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_float_values = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_vector_values = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { float4::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(ValueArray {
            m_bool_values,
            m_int_values,
            m_float_values,
            m_vector_values,
        })
    }
}
