//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GroupConstant {
    pub parent_constant_index: i32,
    pub volume_index: u32,
    pub pitch_index: u32,
    pub mute: bool,
    pub solo: bool,
    pub bypass_effects: bool,
}

impl crate::Parse for GroupConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent_constant_index = r.read_i32()?;
        let volume_index = r.read_u32()?;
        let pitch_index = r.read_u32()?;
        let mute = r.read_bool()?;
        let solo = r.read_bool()?;
        let bypass_effects = r.read_bool()?;
        r.align(4)?;

        Ok(GroupConstant {
            parent_constant_index,
            volume_index,
            pitch_index,
            mute,
            solo,
            bypass_effects,
        })
    }
}
