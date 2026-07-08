//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedTextureProperty {
    pub m_default_name: String,
    pub m_tex_dim: i32,
}

impl crate::Parse for SerializedTextureProperty {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_default_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_tex_dim = rdr.read_i32()?;

        Ok(SerializedTextureProperty {
            m_default_name,
            m_tex_dim,
        })
    }
}
