//! 自动生成。请勿手动修改。

use crate::subclasses::binding::Binding;

#[derive(Debug, Clone)]
pub struct SamplerParameter {
    pub sampler: u32,
    pub bind_point: Option<i32>,
    pub m_binding: Option<Binding>,
}

impl crate::Parse for SamplerParameter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut sampler = None;
        let mut bind_point = None;
        let mut m_binding = None;
        sampler = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            m_binding = Some(Binding::parse(rdr, ver)?);
        } else {
            bind_point = Some(rdr.read_i32()?);
        }

        Ok(SamplerParameter {
            sampler: sampler.unwrap(),
            bind_point,
            m_binding,
        })
    }
}
