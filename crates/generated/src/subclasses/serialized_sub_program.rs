//! 自动生成。请勿手动修改。

use crate::subclasses::buffer_binding::BufferBinding;
use crate::subclasses::constant_buffer::ConstantBuffer;
use crate::subclasses::matrix_parameter::MatrixParameter;
use crate::subclasses::parser_bind_channels::ParserBindChannels;
use crate::subclasses::texture_parameter::TextureParameter;
use crate::subclasses::uav_parameter::UavParameter;
use crate::subclasses::vector_parameter::VectorParameter;

#[derive(Debug, Clone)]
pub struct SerializedSubProgram {
    pub m_blob_index: u32,
    pub m_channels: ParserBindChannels,
    pub m_keyword_indices: Vec<u16>,
    pub m_shader_hardware_tier: i8,
    pub m_gpu_program_type: i8,
    pub m_vector_params: Vec<VectorParameter>,
    pub m_matrix_params: Vec<MatrixParameter>,
    pub m_texture_params: Vec<TextureParameter>,
    pub m_buffer_params: Vec<BufferBinding>,
    pub m_constant_buffers: Vec<ConstantBuffer>,
    pub m_constant_buffer_bindings: Vec<BufferBinding>,
    pub m_uav_params: Vec<UavParameter>,
}

impl crate::Parse for SerializedSubProgram {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_blob_index = r.read_u32()?;
        let m_channels = ParserBindChannels::parse(r, ver)?;
        r.align(4)?;
        let m_keyword_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_shader_hardware_tier = r.read_i8()?;
        let m_gpu_program_type = r.read_i8()?;
        r.align(4)?;
        let m_vector_params = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VectorParameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_matrix_params = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MatrixParameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_texture_params = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TextureParameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_buffer_params = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BufferBinding::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_constant_buffers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ConstantBuffer::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_constant_buffer_bindings = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BufferBinding::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_uav_params = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { UavParameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(SerializedSubProgram {
            m_blob_index,
            m_channels,
            m_keyword_indices,
            m_shader_hardware_tier,
            m_gpu_program_type,
            m_vector_params,
            m_matrix_params,
            m_texture_params,
            m_buffer_params,
            m_constant_buffers,
            m_constant_buffer_bindings,
            m_uav_params,
        })
    }
}
