//! 自动生成。请勿手动修改。

use crate::subclasses::buffer_binding_parameter::BufferBindingParameter;
use crate::subclasses::constant_buffer::ConstantBuffer;
use crate::subclasses::matrix_parameter::MatrixParameter;
use crate::subclasses::sampler_parameter::SamplerParameter;
use crate::subclasses::texture_parameter::TextureParameter;
use crate::subclasses::uav_parameter::UavParameter;
use crate::subclasses::vector_parameter::VectorParameter;

#[derive(Debug, Clone)]
pub struct SerializedProgramParameters {
    pub m_vector_params: Vec<VectorParameter>,
    pub m_matrix_params: Vec<MatrixParameter>,
    pub m_texture_params: Vec<TextureParameter>,
    pub m_buffer_params: Vec<BufferBindingParameter>,
    pub m_constant_buffers: Vec<ConstantBuffer>,
    pub m_constant_buffer_bindings: Vec<BufferBindingParameter>,
    pub m_uav_params: Vec<UavParameter>,
    pub m_samplers: Vec<SamplerParameter>,
}

impl crate::Parse for SerializedProgramParameters {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_vector_params = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| VectorParameter::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_matrix_params = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| MatrixParameter::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_texture_params = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| TextureParameter::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_buffer_params = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| BufferBindingParameter::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_constant_buffers = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ConstantBuffer::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_constant_buffer_bindings = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| BufferBindingParameter::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_uav_params = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| UavParameter::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_samplers = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| SamplerParameter::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(SerializedProgramParameters {
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

impl crate::ToValue for SerializedProgramParameters {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_VectorParams".to_string(),
                self.m_vector_params.to_value(),
            )),
            Some((
                "m_MatrixParams".to_string(),
                self.m_matrix_params.to_value(),
            )),
            Some((
                "m_TextureParams".to_string(),
                self.m_texture_params.to_value(),
            )),
            Some((
                "m_BufferParams".to_string(),
                self.m_buffer_params.to_value(),
            )),
            Some((
                "m_ConstantBuffers".to_string(),
                self.m_constant_buffers.to_value(),
            )),
            Some((
                "m_ConstantBufferBindings".to_string(),
                self.m_constant_buffer_bindings.to_value(),
            )),
            Some(("m_UAVParams".to_string(), self.m_uav_params.to_value())),
            Some(("m_Samplers".to_string(), self.m_samplers.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_VectorParams" => (self.m_vector_params).inspect_value(path, offset, limit),
                "m_MatrixParams" => (self.m_matrix_params).inspect_value(path, offset, limit),
                "m_TextureParams" => (self.m_texture_params).inspect_value(path, offset, limit),
                "m_BufferParams" => (self.m_buffer_params).inspect_value(path, offset, limit),
                "m_ConstantBuffers" => (self.m_constant_buffers).inspect_value(path, offset, limit),
                "m_ConstantBufferBindings" => {
                    (self.m_constant_buffer_bindings).inspect_value(path, offset, limit)
                }
                "m_UAVParams" => (self.m_uav_params).inspect_value(path, offset, limit),
                "m_Samplers" => (self.m_samplers).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_vector_params).inspect_value(path, offset, limit))
                .map(|v| ("m_VectorParams".to_string(), v)),
            ((self.m_matrix_params).inspect_value(path, offset, limit))
                .map(|v| ("m_MatrixParams".to_string(), v)),
            ((self.m_texture_params).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureParams".to_string(), v)),
            ((self.m_buffer_params).inspect_value(path, offset, limit))
                .map(|v| ("m_BufferParams".to_string(), v)),
            ((self.m_constant_buffers).inspect_value(path, offset, limit))
                .map(|v| ("m_ConstantBuffers".to_string(), v)),
            ((self.m_constant_buffer_bindings).inspect_value(path, offset, limit))
                .map(|v| ("m_ConstantBufferBindings".to_string(), v)),
            ((self.m_uav_params).inspect_value(path, offset, limit))
                .map(|v| ("m_UAVParams".to_string(), v)),
            ((self.m_samplers).inspect_value(path, offset, limit))
                .map(|v| ("m_Samplers".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
