//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TakeInfo {
    pub name: String,
    pub default_clip_name: String,
    pub start_time: f32,
    pub internal_id: Option<i64>,
    pub stop_time: f32,
    pub bake_start_time: f32,
    pub bake_stop_time: f32,
    pub sample_rate: f32,
    pub clip: PPtr,
}

impl crate::Parse for TakeInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut default_clip_name = None;
        let mut start_time = None;
        let mut internal_id = None;
        let mut stop_time = None;
        let mut bake_start_time = None;
        let mut bake_stop_time = None;
        let mut sample_rate = None;
        let mut clip = None;
            name = Some(rdr.read_aligned_string()?);
            default_clip_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            internal_id = Some(rdr.read_i64()?);
        } else {
        }
            start_time = Some(rdr.read_f32()?);
            stop_time = Some(rdr.read_f32()?);
            bake_start_time = Some(rdr.read_f32()?);
            bake_stop_time = Some(rdr.read_f32()?);
            sample_rate = Some(rdr.read_f32()?);
            clip = Some(crate::PPtr::parse(rdr, ver)?);

        Ok(TakeInfo {
            name: name.unwrap(),
            default_clip_name: default_clip_name.unwrap(),
            start_time: start_time.unwrap(),
            internal_id,
            stop_time: stop_time.unwrap(),
            bake_start_time: bake_start_time.unwrap(),
            bake_stop_time: bake_stop_time.unwrap(),
            sample_rate: sample_rate.unwrap(),
            clip: clip.unwrap(),
        })
    }
}
