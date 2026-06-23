//! 自动生成。请勿手动修改。

use crate::subclasses::matrix_parameter::MatrixParameter;
use crate::subclasses::struct_parameter::StructParameter;
use crate::subclasses::vector_parameter::VectorParameter;

#[derive(Debug, Clone)]
pub struct ConstantBufferParameter {
    pub m_name_index: i32,
    pub m_matrix_params: Vec<MatrixParameter>,
    pub m_vector_params: Vec<VectorParameter>,
    pub m_struct_params: Vec<StructParameter>,
    pub m_size: i32,
    pub m_is_partial_cb: bool,
}

impl crate::Parse for ConstantBufferParameter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name_index = r.read_i32()?;
        let m_matrix_params = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MatrixParameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_vector_params = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VectorParameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_struct_params = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { StructParameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_size = r.read_i32()?;
        let m_is_partial_cb = r.read_bool()?;
        r.align(4)?;

        Ok(ConstantBufferParameter {
            m_name_index,
            m_matrix_params,
            m_vector_params,
            m_struct_params,
            m_size,
            m_is_partial_cb,
        })
    }
}
