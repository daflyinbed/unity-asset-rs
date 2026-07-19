//! 自动生成。请勿手动修改。

use crate::subclasses::binding::Binding;

#[derive(Debug, Clone)]
pub struct TextureParameter {
    pub m_name_index: i32,
    pub m_index: Option<i32>,
    pub m_binding: Option<Binding>,
    pub m_sampler_index: Option<i32>,
    pub m_sampler_binding: Option<Binding>,
    pub m_dim: i8,
    pub m_multi_sampled: Option<bool>,
}

impl crate::Parse for TextureParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name_index = None;
        let mut m_index = None;
        let mut m_binding = None;
        let mut m_sampler_index = None;
        let mut m_sampler_binding = None;
        let mut m_dim = None;
        let mut m_multi_sampled = None;
        m_name_index = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_binding = Some(Binding::parse(rdr, ver)?);
            m_sampler_binding = Some(Binding::parse(rdr, ver)?);
            m_multi_sampled = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_index = Some(rdr.read_i32()?);
            m_sampler_index = Some(rdr.read_i32()?);
            m_multi_sampled = Some(rdr.read_bool()?);
        } else {
            m_index = Some(rdr.read_i32()?);
            m_sampler_index = Some(rdr.read_i32()?);
        }
        m_dim = Some(rdr.read_i8()?);
        rdr.align(4)?;

        Ok(TextureParameter {
            m_name_index: m_name_index.unwrap(),
            m_index,
            m_binding,
            m_sampler_index,
            m_sampler_binding,
            m_dim: m_dim.unwrap(),
            m_multi_sampled,
        })
    }
}
