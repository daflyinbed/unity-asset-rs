//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_shader_float_value::SerializedShaderFloatValue;
use crate::subclasses::serialized_shader_rt_blend_state::SerializedShaderRtBlendState;
use crate::subclasses::serialized_shader_vector_value::SerializedShaderVectorValue;
use crate::subclasses::serialized_stencil_op::SerializedStencilOp;
use crate::subclasses::serialized_tag_map::SerializedTagMap;

#[derive(Debug, Clone)]
pub struct SerializedShaderState {
    pub m_name: String,
    pub rt_blend0: SerializedShaderRtBlendState,
    pub rt_blend1: SerializedShaderRtBlendState,
    pub rt_blend2: SerializedShaderRtBlendState,
    pub rt_blend3: SerializedShaderRtBlendState,
    pub rt_blend4: SerializedShaderRtBlendState,
    pub rt_blend5: SerializedShaderRtBlendState,
    pub rt_blend6: SerializedShaderRtBlendState,
    pub rt_blend7: SerializedShaderRtBlendState,
    pub rt_separate_blend: bool,
    pub z_test: SerializedShaderFloatValue,
    pub z_write: SerializedShaderFloatValue,
    pub culling: SerializedShaderFloatValue,
    pub offset_factor: SerializedShaderFloatValue,
    pub offset_units: SerializedShaderFloatValue,
    pub alpha_to_mask: SerializedShaderFloatValue,
    pub stencil_op: SerializedStencilOp,
    pub stencil_op_front: SerializedStencilOp,
    pub stencil_op_back: SerializedStencilOp,
    pub stencil_read_mask: SerializedShaderFloatValue,
    pub stencil_write_mask: SerializedShaderFloatValue,
    pub stencil_ref: SerializedShaderFloatValue,
    pub fog_start: SerializedShaderFloatValue,
    pub fog_end: SerializedShaderFloatValue,
    pub fog_density: SerializedShaderFloatValue,
    pub fog_color: SerializedShaderVectorValue,
    pub fog_mode: i32,
    pub gpu_program_id: i32,
    pub m_tags: SerializedTagMap,
    pub m_lod: i32,
    pub lighting: bool,
}

impl crate::Parse for SerializedShaderState {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let rt_blend0 = SerializedShaderRtBlendState::parse(r, ver)?;
        r.align(4)?;
        let rt_blend1 = SerializedShaderRtBlendState::parse(r, ver)?;
        r.align(4)?;
        let rt_blend2 = SerializedShaderRtBlendState::parse(r, ver)?;
        r.align(4)?;
        let rt_blend3 = SerializedShaderRtBlendState::parse(r, ver)?;
        r.align(4)?;
        let rt_blend4 = SerializedShaderRtBlendState::parse(r, ver)?;
        r.align(4)?;
        let rt_blend5 = SerializedShaderRtBlendState::parse(r, ver)?;
        r.align(4)?;
        let rt_blend6 = SerializedShaderRtBlendState::parse(r, ver)?;
        r.align(4)?;
        let rt_blend7 = SerializedShaderRtBlendState::parse(r, ver)?;
        r.align(4)?;
        let rt_separate_blend = r.read_bool()?;
        r.align(4)?;
        let z_test = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let z_write = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let culling = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let offset_factor = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let offset_units = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let alpha_to_mask = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let stencil_op = SerializedStencilOp::parse(r, ver)?;
        r.align(4)?;
        let stencil_op_front = SerializedStencilOp::parse(r, ver)?;
        r.align(4)?;
        let stencil_op_back = SerializedStencilOp::parse(r, ver)?;
        r.align(4)?;
        let stencil_read_mask = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let stencil_write_mask = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let stencil_ref = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let fog_start = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let fog_end = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let fog_density = SerializedShaderFloatValue::parse(r, ver)?;
        r.align(4)?;
        let fog_color = SerializedShaderVectorValue::parse(r, ver)?;
        r.align(4)?;
        let fog_mode = r.read_i32()?;
        let gpu_program_id = r.read_i32()?;
        let m_tags = SerializedTagMap::parse(r, ver)?;
        r.align(4)?;
        let m_lod = r.read_i32()?;
        let lighting = r.read_bool()?;
        r.align(4)?;

        Ok(SerializedShaderState {
            m_name,
            rt_blend0,
            rt_blend1,
            rt_blend2,
            rt_blend3,
            rt_blend4,
            rt_blend5,
            rt_blend6,
            rt_blend7,
            rt_separate_blend,
            z_test,
            z_write,
            culling,
            offset_factor,
            offset_units,
            alpha_to_mask,
            stencil_op,
            stencil_op_front,
            stencil_op_back,
            stencil_read_mask,
            stencil_write_mask,
            stencil_ref,
            fog_start,
            fog_end,
            fog_density,
            fog_color,
            fog_mode,
            gpu_program_id,
            m_tags,
            m_lod,
            lighting,
        })
    }
}
