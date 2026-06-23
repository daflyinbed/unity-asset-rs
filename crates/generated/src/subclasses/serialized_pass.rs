//! 自动生成。请勿手动修改。

use crate::subclasses::serialized_program::SerializedProgram;
use crate::subclasses::serialized_shader_state::SerializedShaderState;
use crate::subclasses::serialized_tag_map::SerializedTagMap;

#[derive(Debug, Clone)]
pub struct SerializedPass {
    pub m_name_indices: Vec<(String, i32)>,
    pub m_type: i32,
    pub m_state: SerializedShaderState,
    pub m_program_mask: u32,
    pub prog_vertex: SerializedProgram,
    pub prog_fragment: SerializedProgram,
    pub prog_geometry: SerializedProgram,
    pub prog_hull: SerializedProgram,
    pub prog_domain: SerializedProgram,
    pub m_has_instancing_variant: bool,
    pub m_use_name: String,
    pub m_name: String,
    pub m_texture_name: String,
    pub m_tags: SerializedTagMap,
}

impl crate::Parse for SerializedPass {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_type = r.read_i32()?;
        let m_state = SerializedShaderState::parse(r, ver)?;
        r.align(4)?;
        let m_program_mask = r.read_u32()?;
        let prog_vertex = SerializedProgram::parse(r, ver)?;
        r.align(4)?;
        let prog_fragment = SerializedProgram::parse(r, ver)?;
        r.align(4)?;
        let prog_geometry = SerializedProgram::parse(r, ver)?;
        r.align(4)?;
        let prog_hull = SerializedProgram::parse(r, ver)?;
        r.align(4)?;
        let prog_domain = SerializedProgram::parse(r, ver)?;
        r.align(4)?;
        let m_has_instancing_variant = r.read_bool()?;
        r.align(4)?;
        let m_use_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_texture_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_tags = SerializedTagMap::parse(r, ver)?;
        r.align(4)?;

        Ok(SerializedPass {
            m_name_indices,
            m_type,
            m_state,
            m_program_mask,
            prog_vertex,
            prog_fragment,
            prog_geometry,
            prog_hull,
            prog_domain,
            m_has_instancing_variant,
            m_use_name,
            m_name,
            m_texture_name,
            m_tags,
        })
    }
}
