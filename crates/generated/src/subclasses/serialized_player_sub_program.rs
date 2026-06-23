//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedPlayerSubProgram {
    pub m_blob_index: u32,
    pub m_keyword_indices: Vec<u16>,
    pub m_shader_requirements: i64,
    pub m_gpu_program_type: i8,
}

impl crate::Parse for SerializedPlayerSubProgram {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_blob_index = r.read_u32()?;
        let m_keyword_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_shader_requirements = r.read_i64()?;
        let m_gpu_program_type = r.read_i8()?;
        r.align(4)?;

        Ok(SerializedPlayerSubProgram {
            m_blob_index,
            m_keyword_indices,
            m_shader_requirements,
            m_gpu_program_type,
        })
    }
}
