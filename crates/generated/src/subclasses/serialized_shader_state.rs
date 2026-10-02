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

impl crate::ToValue for SerializedShaderState {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("rtBlend0".to_string(), self.rt_blend0.to_value())),
            Some(("rtBlend1".to_string(), self.rt_blend1.to_value())),
            Some(("rtBlend2".to_string(), self.rt_blend2.to_value())),
            Some(("rtBlend3".to_string(), self.rt_blend3.to_value())),
            Some(("rtBlend4".to_string(), self.rt_blend4.to_value())),
            Some(("rtBlend5".to_string(), self.rt_blend5.to_value())),
            Some(("rtBlend6".to_string(), self.rt_blend6.to_value())),
            Some(("rtBlend7".to_string(), self.rt_blend7.to_value())),
            Some((
                "rtSeparateBlend".to_string(),
                self.rt_separate_blend.to_value(),
            )),
            Some(("zTest".to_string(), self.z_test.to_value())),
            crate::opt_field("zClip", &self.z_clip),
            Some(("zWrite".to_string(), self.z_write.to_value())),
            Some(("culling".to_string(), self.culling.to_value())),
            Some(("offsetFactor".to_string(), self.offset_factor.to_value())),
            Some(("offsetUnits".to_string(), self.offset_units.to_value())),
            crate::opt_field("conservative", &self.conservative),
            Some(("alphaToMask".to_string(), self.alpha_to_mask.to_value())),
            Some(("stencilOp".to_string(), self.stencil_op.to_value())),
            Some((
                "stencilOpFront".to_string(),
                self.stencil_op_front.to_value(),
            )),
            Some(("stencilOpBack".to_string(), self.stencil_op_back.to_value())),
            Some((
                "stencilReadMask".to_string(),
                self.stencil_read_mask.to_value(),
            )),
            Some((
                "stencilWriteMask".to_string(),
                self.stencil_write_mask.to_value(),
            )),
            Some(("stencilRef".to_string(), self.stencil_ref.to_value())),
            Some(("fogStart".to_string(), self.fog_start.to_value())),
            Some(("fogEnd".to_string(), self.fog_end.to_value())),
            Some(("fogDensity".to_string(), self.fog_density.to_value())),
            Some(("fogColor".to_string(), self.fog_color.to_value())),
            Some(("fogMode".to_string(), self.fog_mode.to_value())),
            Some(("gpuProgramID".to_string(), self.gpu_program_id.to_value())),
            Some(("m_Tags".to_string(), self.m_tags.to_value())),
            Some(("m_LOD".to_string(), self.m_lod.to_value())),
            Some(("lighting".to_string(), self.lighting.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "rtBlend0" => (self.rt_blend0).inspect_value(path, offset, limit),
                "rtBlend1" => (self.rt_blend1).inspect_value(path, offset, limit),
                "rtBlend2" => (self.rt_blend2).inspect_value(path, offset, limit),
                "rtBlend3" => (self.rt_blend3).inspect_value(path, offset, limit),
                "rtBlend4" => (self.rt_blend4).inspect_value(path, offset, limit),
                "rtBlend5" => (self.rt_blend5).inspect_value(path, offset, limit),
                "rtBlend6" => (self.rt_blend6).inspect_value(path, offset, limit),
                "rtBlend7" => (self.rt_blend7).inspect_value(path, offset, limit),
                "rtSeparateBlend" => (self.rt_separate_blend).inspect_value(path, offset, limit),
                "zTest" => (self.z_test).inspect_value(path, offset, limit),
                "zClip" => (self.z_clip).inspect_value(path, offset, limit),
                "zWrite" => (self.z_write).inspect_value(path, offset, limit),
                "culling" => (self.culling).inspect_value(path, offset, limit),
                "offsetFactor" => (self.offset_factor).inspect_value(path, offset, limit),
                "offsetUnits" => (self.offset_units).inspect_value(path, offset, limit),
                "conservative" => (self.conservative).inspect_value(path, offset, limit),
                "alphaToMask" => (self.alpha_to_mask).inspect_value(path, offset, limit),
                "stencilOp" => (self.stencil_op).inspect_value(path, offset, limit),
                "stencilOpFront" => (self.stencil_op_front).inspect_value(path, offset, limit),
                "stencilOpBack" => (self.stencil_op_back).inspect_value(path, offset, limit),
                "stencilReadMask" => (self.stencil_read_mask).inspect_value(path, offset, limit),
                "stencilWriteMask" => (self.stencil_write_mask).inspect_value(path, offset, limit),
                "stencilRef" => (self.stencil_ref).inspect_value(path, offset, limit),
                "fogStart" => (self.fog_start).inspect_value(path, offset, limit),
                "fogEnd" => (self.fog_end).inspect_value(path, offset, limit),
                "fogDensity" => (self.fog_density).inspect_value(path, offset, limit),
                "fogColor" => (self.fog_color).inspect_value(path, offset, limit),
                "fogMode" => (self.fog_mode).inspect_value(path, offset, limit),
                "gpuProgramID" => (self.gpu_program_id).inspect_value(path, offset, limit),
                "m_Tags" => (self.m_tags).inspect_value(path, offset, limit),
                "m_LOD" => (self.m_lod).inspect_value(path, offset, limit),
                "lighting" => (self.lighting).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.rt_blend0).inspect_value(path, offset, limit))
                .map(|v| ("rtBlend0".to_string(), v)),
            ((self.rt_blend1).inspect_value(path, offset, limit))
                .map(|v| ("rtBlend1".to_string(), v)),
            ((self.rt_blend2).inspect_value(path, offset, limit))
                .map(|v| ("rtBlend2".to_string(), v)),
            ((self.rt_blend3).inspect_value(path, offset, limit))
                .map(|v| ("rtBlend3".to_string(), v)),
            ((self.rt_blend4).inspect_value(path, offset, limit))
                .map(|v| ("rtBlend4".to_string(), v)),
            ((self.rt_blend5).inspect_value(path, offset, limit))
                .map(|v| ("rtBlend5".to_string(), v)),
            ((self.rt_blend6).inspect_value(path, offset, limit))
                .map(|v| ("rtBlend6".to_string(), v)),
            ((self.rt_blend7).inspect_value(path, offset, limit))
                .map(|v| ("rtBlend7".to_string(), v)),
            ((self.rt_separate_blend).inspect_value(path, offset, limit))
                .map(|v| ("rtSeparateBlend".to_string(), v)),
            ((self.z_test).inspect_value(path, offset, limit)).map(|v| ("zTest".to_string(), v)),
            ((self.z_clip).inspect_value(path, offset, limit)).map(|v| ("zClip".to_string(), v)),
            ((self.z_write).inspect_value(path, offset, limit)).map(|v| ("zWrite".to_string(), v)),
            ((self.culling).inspect_value(path, offset, limit)).map(|v| ("culling".to_string(), v)),
            ((self.offset_factor).inspect_value(path, offset, limit))
                .map(|v| ("offsetFactor".to_string(), v)),
            ((self.offset_units).inspect_value(path, offset, limit))
                .map(|v| ("offsetUnits".to_string(), v)),
            ((self.conservative).inspect_value(path, offset, limit))
                .map(|v| ("conservative".to_string(), v)),
            ((self.alpha_to_mask).inspect_value(path, offset, limit))
                .map(|v| ("alphaToMask".to_string(), v)),
            ((self.stencil_op).inspect_value(path, offset, limit))
                .map(|v| ("stencilOp".to_string(), v)),
            ((self.stencil_op_front).inspect_value(path, offset, limit))
                .map(|v| ("stencilOpFront".to_string(), v)),
            ((self.stencil_op_back).inspect_value(path, offset, limit))
                .map(|v| ("stencilOpBack".to_string(), v)),
            ((self.stencil_read_mask).inspect_value(path, offset, limit))
                .map(|v| ("stencilReadMask".to_string(), v)),
            ((self.stencil_write_mask).inspect_value(path, offset, limit))
                .map(|v| ("stencilWriteMask".to_string(), v)),
            ((self.stencil_ref).inspect_value(path, offset, limit))
                .map(|v| ("stencilRef".to_string(), v)),
            ((self.fog_start).inspect_value(path, offset, limit))
                .map(|v| ("fogStart".to_string(), v)),
            ((self.fog_end).inspect_value(path, offset, limit)).map(|v| ("fogEnd".to_string(), v)),
            ((self.fog_density).inspect_value(path, offset, limit))
                .map(|v| ("fogDensity".to_string(), v)),
            ((self.fog_color).inspect_value(path, offset, limit))
                .map(|v| ("fogColor".to_string(), v)),
            ((self.fog_mode).inspect_value(path, offset, limit))
                .map(|v| ("fogMode".to_string(), v)),
            ((self.gpu_program_id).inspect_value(path, offset, limit))
                .map(|v| ("gpuProgramID".to_string(), v)),
            ((self.m_tags).inspect_value(path, offset, limit)).map(|v| ("m_Tags".to_string(), v)),
            ((self.m_lod).inspect_value(path, offset, limit)).map(|v| ("m_LOD".to_string(), v)),
            ((self.lighting).inspect_value(path, offset, limit))
                .map(|v| ("lighting".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
