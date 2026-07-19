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
        let mut m_name_index = None;
        let mut m_index = None;
        let mut m_offset_in_constant_buffer = None;
        let mut m_array_size = None;
        let mut m_struct_size = None;
        let mut m_vector_members = None;
        let mut m_matrix_members = None;
        m_name_index = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_offset_in_constant_buffer = Some(rdr.read_i32()?);
        } else {
            m_index = Some(rdr.read_i32()?);
        }
        m_array_size = Some(rdr.read_i32()?);
        m_struct_size = Some(rdr.read_i32()?);
        m_vector_members = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VectorParameter::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_matrix_members = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| MatrixParameter::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;

        Ok(StructParameter {
            m_name_index: m_name_index.unwrap(),
            m_index,
            m_offset_in_constant_buffer,
            m_array_size: m_array_size.unwrap(),
            m_struct_size: m_struct_size.unwrap(),
            m_vector_members: m_vector_members.unwrap(),
            m_matrix_members: m_matrix_members.unwrap(),
        })
    }
}
