//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TakeInfo {
    pub name: String,
    pub default_clip_name: String,
    pub start_time: f32,
    pub stop_time: f32,
    pub bake_start_time: f32,
    pub bake_stop_time: f32,
    pub sample_rate: f32,
    pub clip: PPtr,
}

impl crate::Parse for TakeInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let default_clip_name = r.read_aligned_string()?;
        r.align(4)?;
        let start_time = r.read_f32()?;
        let stop_time = r.read_f32()?;
        let bake_start_time = r.read_f32()?;
        let bake_stop_time = r.read_f32()?;
        let sample_rate = r.read_f32()?;
        let clip = crate::PPtr::parse(r, ver)?;

        Ok(TakeInfo {
            name,
            default_clip_name,
            start_time,
            stop_time,
            bake_start_time,
            bake_stop_time,
            sample_rate,
            clip,
        })
    }
}
