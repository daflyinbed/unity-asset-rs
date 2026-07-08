//! 自动生成。请勿手动修改。

use crate::subclasses::buffer_binding::BufferBinding;
use crate::subclasses::constant_buffer::ConstantBuffer;
use crate::subclasses::matrix_parameter::MatrixParameter;
use crate::subclasses::parser_bind_channels::ParserBindChannels;
use crate::subclasses::program_parameters::ProgramParameters;
use crate::subclasses::sampler_parameter::SamplerParameter;
use crate::subclasses::serialized_program_parameters::SerializedProgramParameters;
use crate::subclasses::texture_parameter::TextureParameter;
use crate::subclasses::uav_parameter::UavParameter;
use crate::subclasses::vector_parameter::VectorParameter;


/// 版本枚举：SerializedSubProgramMKeywordIndicesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedSubProgramMKeywordIndicesVariant {
    Default(Vec<u16>),
    V2017_1(Vec<u16>),
}

impl crate::Parse for SerializedSubProgramMKeywordIndicesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SerializedSubProgramMParametersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedSubProgramMParametersVariant {
    Default(Box<SerializedProgramParameters>),
    V6000_5(Box<ProgramParameters>),
}

impl crate::Parse for SerializedSubProgramMParametersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ( { Ok(Box::new(ProgramParameters::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V6000_5(v))
        } else {
            let v = ( { Ok(Box::new(SerializedProgramParameters::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SerializedSubProgramMShaderRequirementsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedSubProgramMShaderRequirementsVariant {
    Default(i32),
    V2021_1(i64),
}

impl crate::Parse for SerializedSubProgramMShaderRequirementsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_i64()?;
            Ok(Self::V2021_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SerializedSubProgramMBufferParamsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedSubProgramMBufferParamsVariant {
    Default(Vec<BufferBinding>),
    V2017_1(Vec<BufferBinding>),
}

impl crate::Parse for SerializedSubProgramMBufferParamsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BufferBinding::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BufferBinding::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SerializedSubProgramMConstantBufferBindingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedSubProgramMConstantBufferBindingsVariant {
    Default(Vec<BufferBinding>),
    V2017_1(Vec<BufferBinding>),
}

impl crate::Parse for SerializedSubProgramMConstantBufferBindingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BufferBinding::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BufferBinding::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SerializedSubProgramMUavParamsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SerializedSubProgramMUavParamsVariant {
    Default(Vec<UavParameter>),
    V2017_1(Vec<UavParameter>),
}

impl crate::Parse for SerializedSubProgramMUavParamsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { UavParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { UavParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SerializedSubProgram {
    pub m_blob_index: u32,
    pub m_channels: ParserBindChannels,
    pub m_keyword_indices: Option<SerializedSubProgramMKeywordIndicesVariant>,
    pub m_global_keyword_indices: Option<Vec<u16>>,
    pub m_shader_hardware_tier: i8,
    pub m_local_keyword_indices: Option<Vec<u16>>,
    pub m_gpu_program_type: i8,
    pub m_vector_params: Option<Vec<VectorParameter>>,
    pub m_parameters: Option<SerializedSubProgramMParametersVariant>,
    pub m_matrix_params: Option<Vec<MatrixParameter>>,
    pub m_shader_requirements: Option<SerializedSubProgramMShaderRequirementsVariant>,
    pub m_texture_params: Option<Vec<TextureParameter>>,
    pub m_buffer_params: Option<SerializedSubProgramMBufferParamsVariant>,
    pub m_constant_buffers: Option<Vec<ConstantBuffer>>,
    pub m_constant_buffer_bindings: Option<SerializedSubProgramMConstantBufferBindingsVariant>,
    pub m_uav_params: Option<SerializedSubProgramMUavParamsVariant>,
    pub m_samplers: Option<Vec<SamplerParameter>>,
}

impl crate::Parse for SerializedSubProgram {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_blob_index = None;
        let mut m_channels = None;
        let mut m_keyword_indices = None;
        let mut m_global_keyword_indices = None;
        let mut m_shader_hardware_tier = None;
        let mut m_local_keyword_indices = None;
        let mut m_gpu_program_type = None;
        let mut m_vector_params = None;
        let mut m_parameters = None;
        let mut m_matrix_params = None;
        let mut m_shader_requirements = None;
        let mut m_texture_params = None;
        let mut m_buffer_params = None;
        let mut m_constant_buffers = None;
        let mut m_constant_buffer_bindings = None;
        let mut m_uav_params = None;
        let mut m_samplers = None;
            m_blob_index = Some(rdr.read_u32()?);
            m_channels = Some(ParserBindChannels::parse(rdr, ver)?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_keyword_indices = Some(SerializedSubProgramMKeywordIndicesVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_global_keyword_indices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_local_keyword_indices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
            m_keyword_indices = Some(SerializedSubProgramMKeywordIndicesVariant::parse(rdr, ver)?);
        }
            m_shader_hardware_tier = Some(rdr.read_i8()?);
            m_gpu_program_type = Some(rdr.read_i8()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2021, 1, 1) {
            m_parameters = Some(SerializedSubProgramMParametersVariant::parse(rdr, ver)?);
            m_shader_requirements = Some(SerializedSubProgramMShaderRequirementsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_vector_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VectorParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_matrix_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MatrixParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_texture_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TextureParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_buffer_params = Some(SerializedSubProgramMBufferParamsVariant::parse(rdr, ver)?);
            m_constant_buffers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ConstantBuffer::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_constant_buffer_bindings = Some(SerializedSubProgramMConstantBufferBindingsVariant::parse(rdr, ver)?);
            m_uav_params = Some(SerializedSubProgramMUavParamsVariant::parse(rdr, ver)?);
            m_samplers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SamplerParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_shader_requirements = Some(SerializedSubProgramMShaderRequirementsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 2) {
            m_parameters = Some(SerializedSubProgramMParametersVariant::parse(rdr, ver)?);
            m_shader_requirements = Some(SerializedSubProgramMShaderRequirementsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_vector_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VectorParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_matrix_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MatrixParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_texture_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TextureParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_buffer_params = Some(SerializedSubProgramMBufferParamsVariant::parse(rdr, ver)?);
            m_constant_buffers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ConstantBuffer::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_constant_buffer_bindings = Some(SerializedSubProgramMConstantBufferBindingsVariant::parse(rdr, ver)?);
            m_uav_params = Some(SerializedSubProgramMUavParamsVariant::parse(rdr, ver)?);
            m_samplers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SamplerParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_shader_requirements = Some(SerializedSubProgramMShaderRequirementsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_vector_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VectorParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_matrix_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MatrixParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_texture_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TextureParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_buffer_params = Some(SerializedSubProgramMBufferParamsVariant::parse(rdr, ver)?);
            m_constant_buffers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ConstantBuffer::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_constant_buffer_bindings = Some(SerializedSubProgramMConstantBufferBindingsVariant::parse(rdr, ver)?);
            m_uav_params = Some(SerializedSubProgramMUavParamsVariant::parse(rdr, ver)?);
            m_samplers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SamplerParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
            m_vector_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VectorParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_matrix_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { MatrixParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_texture_params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { TextureParameter::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_buffer_params = Some(SerializedSubProgramMBufferParamsVariant::parse(rdr, ver)?);
            m_constant_buffers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ConstantBuffer::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_constant_buffer_bindings = Some(SerializedSubProgramMConstantBufferBindingsVariant::parse(rdr, ver)?);
            m_uav_params = Some(SerializedSubProgramMUavParamsVariant::parse(rdr, ver)?);
        }

        Ok(SerializedSubProgram {
            m_blob_index: m_blob_index.unwrap(),
            m_channels: m_channels.unwrap(),
            m_keyword_indices,
            m_global_keyword_indices,
            m_shader_hardware_tier: m_shader_hardware_tier.unwrap(),
            m_local_keyword_indices,
            m_gpu_program_type: m_gpu_program_type.unwrap(),
            m_vector_params,
            m_parameters,
            m_matrix_params,
            m_shader_requirements,
            m_texture_params,
            m_buffer_params,
            m_constant_buffers,
            m_constant_buffer_bindings,
            m_uav_params,
            m_samplers,
        })
    }
}
