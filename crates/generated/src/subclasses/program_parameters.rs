//! 自动生成。请勿手动修改。

use crate::subclasses::buffer_binding_parameter::BufferBindingParameter;
use crate::subclasses::constant_buffer_parameter::ConstantBufferParameter;
use crate::subclasses::matrix_parameter::MatrixParameter;
use crate::subclasses::sampler_parameter::SamplerParameter;
use crate::subclasses::texture_parameter::TextureParameter;
use crate::subclasses::uav_parameter::UavParameter;
use crate::subclasses::vector_parameter::VectorParameter;

#[derive(Debug, Clone)]
pub struct ProgramParameters {
    pub m_vector_params: Vec<VectorParameter>,
    pub m_matrix_params: Vec<MatrixParameter>,
    pub m_texture_params: Vec<TextureParameter>,
    pub m_buffer_params: Vec<BufferBindingParameter>,
    pub m_constant_buffers: Vec<ConstantBufferParameter>,
    pub m_constant_buffer_bindings: Vec<BufferBindingParameter>,
    pub m_uav_params: Vec<UavParameter>,
    pub m_samplers: Vec<SamplerParameter>,
}

impl crate::Parse for ProgramParameters {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_vector_params = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VectorParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_matrix_params = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MatrixParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_texture_params = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TextureParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_buffer_params = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BufferBindingParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_constant_buffers = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ConstantBufferParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_constant_buffer_bindings = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BufferBindingParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_uav_params = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { UavParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_samplers = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SamplerParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(ProgramParameters {
            m_vector_params,
            m_matrix_params,
            m_texture_params,
            m_buffer_params,
            m_constant_buffers,
            m_constant_buffer_bindings,
            m_uav_params,
            m_samplers,
        })
    }
}
