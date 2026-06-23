//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GroupConnection {
    pub source_group_index: u32,
    pub target_group_index: u32,
    pub send_effect_index: u32,
}

impl crate::Parse for GroupConnection {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let source_group_index = r.read_u32()?;
        let target_group_index = r.read_u32()?;
        let send_effect_index = r.read_u32()?;

        Ok(GroupConnection {
            source_group_index,
            target_group_index,
            send_effect_index,
        })
    }
}
