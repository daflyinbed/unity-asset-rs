//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct HumanPoseMask {
    pub word0: u32,
    pub word1: u32,
}

impl crate::Parse for HumanPoseMask {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let word0 = r.read_u32()?;
        let word1 = r.read_u32()?;

        Ok(HumanPoseMask {
            word0,
            word1,
        })
    }
}
