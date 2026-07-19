//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GroupConstant {
    pub parent_constant_index: i32,
    pub volume_index: u32,
    pub pitch_index: u32,
    pub mute: bool,
    pub send_index: Option<u32>,
    pub solo: bool,
    pub bypass_effects: bool,
}

impl crate::Parse for GroupConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut parent_constant_index = None;
        let mut volume_index = None;
        let mut pitch_index = None;
        let mut mute = None;
        let mut send_index = None;
        let mut solo = None;
        let mut bypass_effects = None;
        parent_constant_index = Some(rdr.read_i32()?);
        volume_index = Some(rdr.read_u32()?);
        pitch_index = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 1) {
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            send_index = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 26) {
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            send_index = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 21) {
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            send_index = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 4, 32) {
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            send_index = Some(rdr.read_u32()?);
        } else {
        }
        mute = Some(rdr.read_bool()?);
        solo = Some(rdr.read_bool()?);
        bypass_effects = Some(rdr.read_bool()?);
        rdr.align(4)?;

        Ok(GroupConstant {
            parent_constant_index: parent_constant_index.unwrap(),
            volume_index: volume_index.unwrap(),
            pitch_index: pitch_index.unwrap(),
            mute: mute.unwrap(),
            send_index,
            solo: solo.unwrap(),
            bypass_effects: bypass_effects.unwrap(),
        })
    }
}
