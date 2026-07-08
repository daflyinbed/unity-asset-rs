//! 自动生成。请勿手动修改。

use crate::subclasses::matrix_parameter::MatrixParameter;
use crate::subclasses::struct_parameter::StructParameter;
use crate::subclasses::vector_parameter::VectorParameter;

#[derive(Debug, Clone)]
pub struct ConstantBuffer {
    pub m_name_index: i32,
    pub m_matrix_params: Vec<MatrixParameter>,
    pub m_vector_params: Vec<VectorParameter>,
    pub m_size: i32,
    pub m_struct_params: Option<Vec<StructParameter>>,
    pub m_is_partial_cb: Option<bool>,
}

impl crate::Parse for ConstantBuffer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name_index = None;
        let mut m_matrix_params = None;
        let mut m_vector_params = None;
        let mut m_size = None;
        let mut m_struct_params = None;
        let mut m_is_partial_cb = None;
            m_name_index = Some(rdr.read_i32()?);
            m_matrix_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MatrixParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_vector_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VectorParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_struct_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { StructParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }
            m_size = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2021, 1, 1) {
            m_is_partial_cb = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2020, 3, 2) {
            m_is_partial_cb = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(ConstantBuffer {
            m_name_index: m_name_index.unwrap(),
            m_matrix_params: m_matrix_params.unwrap(),
            m_vector_params: m_vector_params.unwrap(),
            m_size: m_size.unwrap(),
            m_struct_params,
            m_is_partial_cb,
        })
    }
}
