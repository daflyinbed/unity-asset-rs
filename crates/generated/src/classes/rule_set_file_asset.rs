//! 自动生成。请勿手动修改。

use super::text_asset::TextAsset;

#[derive(Debug, Clone)]
pub struct RuleSetFileAsset {
    pub parent: TextAsset,
    pub m_name: String,
    pub m_script: String,
}

impl crate::Parse for RuleSetFileAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = TextAsset::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_script = r.read_aligned_string()?;
        r.align(4)?;

        Ok(RuleSetFileAsset {
            parent,
            m_name,
            m_script,
        })
    }
}
