//! 自动生成。请勿手动修改。

use crate::subclasses::build_step_message::BuildStepMessage;

#[derive(Debug, Clone)]
pub struct BuildStepInfo {
    pub step_name: String,
    pub duration: u64,
    pub messages: Vec<BuildStepMessage>,
}

impl crate::Parse for BuildStepInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let step_name = r.read_aligned_string()?;
        r.align(4)?;
        let duration = r.read_u64()?;
        let messages = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildStepMessage::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(BuildStepInfo {
            step_name,
            duration,
            messages,
        })
    }
}
