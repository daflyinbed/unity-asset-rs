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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SubEmitterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SubEmitterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
