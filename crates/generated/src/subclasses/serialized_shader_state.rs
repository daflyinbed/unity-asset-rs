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
    pub z_clip: Option<SerializedShaderFloatValue>,
    pub z_write: SerializedShaderFloatValue,
    pub culling: SerializedShaderFloatValue,
    pub offset_factor: SerializedShaderFloatValue,
    pub offset_units: SerializedShaderFloatValue,
    pub conservative: Option<SerializedShaderFloatValue>,
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut rt_blend0 = None;
        let mut rt_blend1 = None;
        let mut rt_blend2 = None;
        let mut rt_blend3 = None;
        let mut rt_blend4 = None;
        let mut rt_blend5 = None;
        let mut rt_blend6 = None;
        let mut rt_blend7 = None;
        let mut rt_separate_blend = None;
        let mut z_test = None;
        let mut z_clip = None;
        let mut z_write = None;
        let mut culling = None;
        let mut offset_factor = None;
        let mut offset_units = None;
        let mut conservative = None;
        let mut alpha_to_mask = None;
        let mut stencil_op = None;
        let mut stencil_op_front = None;
        let mut stencil_op_back = None;
        let mut stencil_read_mask = None;
        let mut stencil_write_mask = None;
        let mut stencil_ref = None;
        let mut fog_start = None;
        let mut fog_end = None;
        let mut fog_density = None;
        let mut fog_color = None;
        let mut fog_mode = None;
        let mut gpu_program_id = None;
        let mut m_tags = None;
        let mut m_lod = None;
        let mut lighting = None;
            m_name = Some(rdr.read_aligned_string()?);
            rt_blend0 = Some(SerializedShaderRtBlendState::parse(rdr, ver)?);
            rt_blend1 = Some(SerializedShaderRtBlendState::parse(rdr, ver)?);
            rt_blend2 = Some(SerializedShaderRtBlendState::parse(rdr, ver)?);
            rt_blend3 = Some(SerializedShaderRtBlendState::parse(rdr, ver)?);
            rt_blend4 = Some(SerializedShaderRtBlendState::parse(rdr, ver)?);
            rt_blend5 = Some(SerializedShaderRtBlendState::parse(rdr, ver)?);
            rt_blend6 = Some(SerializedShaderRtBlendState::parse(rdr, ver)?);
            rt_blend7 = Some(SerializedShaderRtBlendState::parse(rdr, ver)?);
            rt_separate_blend = Some(rdr.read_bool()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            z_clip = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
        } else {
        }
            z_test = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
            z_write = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
            culling = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            conservative = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
        } else {
        }
            offset_factor = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
            offset_units = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
            alpha_to_mask = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
            stencil_op = Some(SerializedStencilOp::parse(rdr, ver)?);
            stencil_op_front = Some(SerializedStencilOp::parse(rdr, ver)?);
            stencil_op_back = Some(SerializedStencilOp::parse(rdr, ver)?);
            stencil_read_mask = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
            stencil_write_mask = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
            stencil_ref = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
            fog_start = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
            fog_end = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
            fog_density = Some(SerializedShaderFloatValue::parse(rdr, ver)?);
            fog_color = Some(SerializedShaderVectorValue::parse(rdr, ver)?);
            fog_mode = Some(rdr.read_i32()?);
            gpu_program_id = Some(rdr.read_i32()?);
            m_tags = Some(SerializedTagMap::parse(rdr, ver)?);
            m_lod = Some(rdr.read_i32()?);
            lighting = Some(rdr.read_bool()?);
            rdr.align(4)?;

        Ok(SerializedShaderState {
            m_name: m_name.unwrap(),
            rt_blend0: rt_blend0.unwrap(),
            rt_blend1: rt_blend1.unwrap(),
            rt_blend2: rt_blend2.unwrap(),
            rt_blend3: rt_blend3.unwrap(),
            rt_blend4: rt_blend4.unwrap(),
            rt_blend5: rt_blend5.unwrap(),
            rt_blend6: rt_blend6.unwrap(),
            rt_blend7: rt_blend7.unwrap(),
            rt_separate_blend: rt_separate_blend.unwrap(),
            z_test: z_test.unwrap(),
            z_clip,
            z_write: z_write.unwrap(),
            culling: culling.unwrap(),
            offset_factor: offset_factor.unwrap(),
            offset_units: offset_units.unwrap(),
            conservative,
            alpha_to_mask: alpha_to_mask.unwrap(),
            stencil_op: stencil_op.unwrap(),
            stencil_op_front: stencil_op_front.unwrap(),
            stencil_op_back: stencil_op_back.unwrap(),
            stencil_read_mask: stencil_read_mask.unwrap(),
            stencil_write_mask: stencil_write_mask.unwrap(),
            stencil_ref: stencil_ref.unwrap(),
            fog_start: fog_start.unwrap(),
            fog_end: fog_end.unwrap(),
            fog_density: fog_density.unwrap(),
            fog_color: fog_color.unwrap(),
            fog_mode: fog_mode.unwrap(),
            gpu_program_id: gpu_program_id.unwrap(),
            m_tags: m_tags.unwrap(),
            m_lod: m_lod.unwrap(),
            lighting: lighting.unwrap(),
        })
    }
}
