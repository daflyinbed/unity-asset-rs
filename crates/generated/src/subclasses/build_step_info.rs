//! 自动生成。请勿手动修改。

use crate::subclasses::build_step_message::BuildStepMessage;


/// 版本枚举：BuildStepInfoMessagesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuildStepInfoMessagesVariant {
    Default(Vec<BuildStepMessage>),
    V2017_1(Vec<BuildStepMessage>),
}

impl crate::Parse for BuildStepInfoMessagesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BuildStepMessage::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { BuildStepMessage::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuildStepInfo {
    pub step_name: String,
    pub duration: Option<u64>,
    pub duration_ticks: Option<u64>,
    pub messages: BuildStepInfoMessagesVariant,
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
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            duration_ticks = Some(rdr.read_u64()?);
            depth = Some(rdr.read_i32()?);
        } else {
            duration = Some(rdr.read_u64()?);
        }
            messages = Some(BuildStepInfoMessagesVariant::parse(rdr, ver)?);

        Ok(BuildStepInfo {
            step_name: step_name.unwrap(),
            duration,
            duration_ticks,
            messages: messages.unwrap(),
            depth,
        })
    }
}
