//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VisualEffectSubgraphOperator {
    pub m_name: String,
}

impl crate::Parse for VisualEffectSubgraphOperator {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(VisualEffectSubgraphOperator {
            m_name,
        })
    }
}
