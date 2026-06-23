//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TextureParameter {
    pub m_name_index: i32,
    pub m_index: i32,
    pub m_sampler_index: i32,
    pub m_dim: i8,
}

impl crate::Parse for TextureParameter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name_index = r.read_i32()?;
        let m_index = r.read_i32()?;
        let m_sampler_index = r.read_i32()?;
        let m_dim = r.read_i8()?;
        r.align(4)?;

        Ok(TextureParameter {
            m_name_index,
            m_index,
            m_sampler_index,
            m_dim,
        })
    }
}
