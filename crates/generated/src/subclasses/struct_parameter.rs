//! 自动生成。请勿手动修改。

use crate::subclasses::matrix_parameter::MatrixParameter;
use crate::subclasses::vector_parameter::VectorParameter;

#[derive(Debug, Clone)]
pub struct StructParameter {
    pub m_name_index: i32,
    pub m_index: i32,
    pub m_array_size: i32,
    pub m_struct_size: i32,
    pub m_vector_members: Vec<VectorParameter>,
    pub m_matrix_members: Vec<MatrixParameter>,
}

impl crate::Parse for StructParameter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name_index = r.read_i32()?;
        let m_index = r.read_i32()?;
        let m_array_size = r.read_i32()?;
        let m_struct_size = r.read_i32()?;
        let m_vector_members = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VectorParameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_matrix_members = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MatrixParameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(StructParameter {
            m_name_index,
            m_index,
            m_array_size,
            m_struct_size,
            m_vector_members,
            m_matrix_members,
        })
    }
}
