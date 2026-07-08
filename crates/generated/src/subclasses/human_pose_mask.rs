//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct HumanPoseMask {
    pub word0: u32,
    pub word1: u32,
    pub word2: Option<u32>,
}

impl crate::Parse for HumanPoseMask {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let word0 = rdr.read_u32()?;
        let word1 = rdr.read_u32()?;
        let word2 = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };

        Ok(HumanPoseMask {
            word0,
            word1,
            word2,
        })
    }
}
