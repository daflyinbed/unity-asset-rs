//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::sub_emitter_data::SubEmitterData;

/// 版本枚举：SubModuleSubEmittersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SubModuleSubEmittersVariant {
    Default(Vec<SubEmitterData>),
    V2017_1(Vec<SubEmitterData>),
}

impl crate::Parse for SubModuleSubEmittersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubEmitterData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubEmitterData::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SubModuleSubEmittersVariant {
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
pub struct SubModule {
    pub enabled: bool,
    pub sub_emitter_birth: Option<PPtr>,
    pub sub_emitters: Option<SubModuleSubEmittersVariant>,
    pub sub_emitter_death: Option<PPtr>,
    pub sub_emitter_birth1: Option<PPtr>,
    pub sub_emitter_collision: Option<PPtr>,
    pub sub_emitter_collision1: Option<PPtr>,
    pub sub_emitter_death1: Option<PPtr>,
}

impl crate::Parse for SubModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut sub_emitter_birth = None;
        let mut sub_emitters = None;
        let mut sub_emitter_death = None;
        let mut sub_emitter_birth1 = None;
        let mut sub_emitter_collision = None;
        let mut sub_emitter_collision1 = None;
        let mut sub_emitter_death1 = None;
        enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            sub_emitters = Some(SubModuleSubEmittersVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            sub_emitter_birth = Some(crate::PPtr::parse(rdr, ver)?);
            sub_emitter_birth1 = Some(crate::PPtr::parse(rdr, ver)?);
            sub_emitter_collision = Some(crate::PPtr::parse(rdr, ver)?);
            sub_emitter_collision1 = Some(crate::PPtr::parse(rdr, ver)?);
            sub_emitter_death = Some(crate::PPtr::parse(rdr, ver)?);
            sub_emitter_death1 = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
            sub_emitter_birth = Some(crate::PPtr::parse(rdr, ver)?);
            sub_emitter_death = Some(crate::PPtr::parse(rdr, ver)?);
            sub_emitter_collision = Some(crate::PPtr::parse(rdr, ver)?);
        }

        Ok(SubModule {
            enabled: enabled.unwrap(),
            sub_emitter_birth,
            sub_emitters,
            sub_emitter_death,
            sub_emitter_birth1,
            sub_emitter_collision,
            sub_emitter_collision1,
            sub_emitter_death1,
        })
    }
}

impl crate::ToValue for SubModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            crate::opt_field("subEmitterBirth", &self.sub_emitter_birth),
            crate::opt_field("subEmitters", &self.sub_emitters),
            crate::opt_field("subEmitterDeath", &self.sub_emitter_death),
            crate::opt_field("subEmitterBirth1", &self.sub_emitter_birth1),
            crate::opt_field("subEmitterCollision", &self.sub_emitter_collision),
            crate::opt_field("subEmitterCollision1", &self.sub_emitter_collision1),
            crate::opt_field("subEmitterDeath1", &self.sub_emitter_death1),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "subEmitterBirth" => (self.sub_emitter_birth).inspect_value(path, offset, limit),
                "subEmitters" => (self.sub_emitters).inspect_value(path, offset, limit),
                "subEmitterDeath" => (self.sub_emitter_death).inspect_value(path, offset, limit),
                "subEmitterBirth1" => (self.sub_emitter_birth1).inspect_value(path, offset, limit),
                "subEmitterCollision" => {
                    (self.sub_emitter_collision).inspect_value(path, offset, limit)
                }
                "subEmitterCollision1" => {
                    (self.sub_emitter_collision1).inspect_value(path, offset, limit)
                }
                "subEmitterDeath1" => (self.sub_emitter_death1).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.sub_emitter_birth).inspect_value(path, offset, limit))
                .map(|v| ("subEmitterBirth".to_string(), v)),
            ((self.sub_emitters).inspect_value(path, offset, limit))
                .map(|v| ("subEmitters".to_string(), v)),
            ((self.sub_emitter_death).inspect_value(path, offset, limit))
                .map(|v| ("subEmitterDeath".to_string(), v)),
            ((self.sub_emitter_birth1).inspect_value(path, offset, limit))
                .map(|v| ("subEmitterBirth1".to_string(), v)),
            ((self.sub_emitter_collision).inspect_value(path, offset, limit))
                .map(|v| ("subEmitterCollision".to_string(), v)),
            ((self.sub_emitter_collision1).inspect_value(path, offset, limit))
                .map(|v| ("subEmitterCollision1".to_string(), v)),
            ((self.sub_emitter_death1).inspect_value(path, offset, limit))
                .map(|v| ("subEmitterDeath1".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
