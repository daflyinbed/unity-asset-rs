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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_blob_index = rdr.read_u32()?;
        let m_keyword_indices = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u16())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_shader_requirements = rdr.read_i64()?;
        let m_gpu_program_type = rdr.read_i8()?;
        rdr.align(4)?;

        Ok(SerializedPlayerSubProgram {
            m_blob_index,
            m_keyword_indices,
            m_shader_requirements,
            m_gpu_program_type,
        })
    }
}

impl crate::ToValue for SerializedPlayerSubProgram {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BlobIndex".to_string(), self.m_blob_index.to_value())),
            Some((
                "m_KeywordIndices".to_string(),
                self.m_keyword_indices.to_value(),
            )),
            Some((
                "m_ShaderRequirements".to_string(),
                self.m_shader_requirements.to_value(),
            )),
            Some((
                "m_GpuProgramType".to_string(),
                self.m_gpu_program_type.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BlobIndex" => (self.m_blob_index).inspect_value(path, offset, limit),
                "m_KeywordIndices" => (self.m_keyword_indices).inspect_value(path, offset, limit),
                "m_ShaderRequirements" => {
                    (self.m_shader_requirements).inspect_value(path, offset, limit)
                }
                "m_GpuProgramType" => (self.m_gpu_program_type).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_blob_index).inspect_value(path, offset, limit))
                .map(|v| ("m_BlobIndex".to_string(), v)),
            ((self.m_keyword_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_KeywordIndices".to_string(), v)),
            ((self.m_shader_requirements).inspect_value(path, offset, limit))
                .map(|v| ("m_ShaderRequirements".to_string(), v)),
            ((self.m_gpu_program_type).inspect_value(path, offset, limit))
                .map(|v| ("m_GpuProgramType".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
