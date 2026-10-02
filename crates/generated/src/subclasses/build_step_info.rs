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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BuildStepMessage::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BuildStepMessage::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BuildStepInfoMessagesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for BuildStepInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("stepName".to_string(), self.step_name.to_value())),
            crate::opt_field("duration", &self.duration),
            crate::opt_field("durationTicks", &self.duration_ticks),
            Some(("messages".to_string(), self.messages.to_value())),
            crate::opt_field("depth", &self.depth),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "stepName" => (self.step_name).inspect_value(path, offset, limit),
                "duration" => (self.duration).inspect_value(path, offset, limit),
                "durationTicks" => (self.duration_ticks).inspect_value(path, offset, limit),
                "messages" => (self.messages).inspect_value(path, offset, limit),
                "depth" => (self.depth).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.step_name).inspect_value(path, offset, limit))
                .map(|v| ("stepName".to_string(), v)),
            ((self.duration).inspect_value(path, offset, limit))
                .map(|v| ("duration".to_string(), v)),
            ((self.duration_ticks).inspect_value(path, offset, limit))
                .map(|v| ("durationTicks".to_string(), v)),
            ((self.messages).inspect_value(path, offset, limit))
                .map(|v| ("messages".to_string(), v)),
            ((self.depth).inspect_value(path, offset, limit)).map(|v| ("depth".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
