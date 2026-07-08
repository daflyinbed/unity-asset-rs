//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetSelection {
    pub platform: i32,
    pub sub_target: i32,
    pub extended_platform: i32,
}

impl crate::Parse for BuildTargetSelection {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let platform = rdr.read_i32()?;
        let sub_target = rdr.read_i32()?;
        let extended_platform = rdr.read_i32()?;

        Ok(BuildTargetSelection {
            platform,
            sub_target,
            extended_platform,
        })
    }
}
