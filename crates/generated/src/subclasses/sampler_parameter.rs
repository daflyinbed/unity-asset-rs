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
        let sampler = rdr.read_u32()?;
        let bind_point = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_binding = if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            Some(Binding::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(SamplerParameter {
            sampler,
            bind_point,
            m_binding,
        })
    }
}
