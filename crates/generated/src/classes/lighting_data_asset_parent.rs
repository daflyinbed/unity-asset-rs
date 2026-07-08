//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LightingDataAssetParent {
    pub m_name: String,
}

impl crate::Parse for LightingDataAssetParent {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(LightingDataAssetParent {
            m_name,
        })
    }
}
