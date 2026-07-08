//! 自动生成。请勿手动修改。

use crate::subclasses::matrix_parameter::MatrixParameter;
use crate::subclasses::vector_parameter::VectorParameter;

#[derive(Debug, Clone)]
pub struct StructParameter {
    pub m_name_index: i32,
    pub m_index: Option<i32>,
    pub m_offset_in_constant_buffer: Option<i32>,
    pub m_array_size: i32,
    pub m_struct_size: i32,
    pub m_vector_members: Vec<VectorParameter>,
    pub m_matrix_members: Vec<MatrixParameter>,
}

impl crate::Parse for StructParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name_index = rdr.read_i32()?;
        let m_index = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_offset_in_constant_buffer = if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_array_size = rdr.read_i32()?;
        let m_struct_size = rdr.read_i32()?;
        let m_vector_members = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VectorParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_matrix_members = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MatrixParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(StructParameter {
            m_name_index,
            m_index,
            m_offset_in_constant_buffer,
            m_array_size,
            m_struct_size,
            m_vector_members,
            m_matrix_members,
        })
    }
}
