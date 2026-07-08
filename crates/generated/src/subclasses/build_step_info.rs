//! 自动生成。请勿手动修改。

use crate::subclasses::build_step_message::BuildStepMessage;

#[derive(Debug, Clone)]
pub struct BuildStepInfo {
    pub step_name: String,
    pub duration: Option<u64>,
    pub duration_ticks: Option<u64>,
    pub messages: Vec<BuildStepMessage>,
    pub depth: Option<i32>,
}

impl crate::Parse for BuildStepInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut step_name = None;
        let mut duration = None;
        let mut duration_ticks = None;
        let mut messages = None;
        let mut depth = None;
            step_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            duration_ticks = Some(rdr.read_u64()?);
            depth = Some(rdr.read_i32()?);
        } else {
            duration = Some(rdr.read_u64()?);
        }
            messages = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BuildStepMessage::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;

        Ok(BuildStepInfo {
            step_name: step_name.unwrap(),
            duration,
            duration_ticks,
            messages: messages.unwrap(),
            depth,
        })
    }
}
