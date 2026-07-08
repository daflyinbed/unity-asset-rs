//! 自动生成。请勿手动修改。

use crate::subclasses::float3::float3;
use crate::subclasses::float4::float4;
use crate::subclasses::xform::xform;


/// 版本枚举：HumanGoalMHintTVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HumanGoalMHintTVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for HumanGoalMHintTVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ( { Ok(Box::new(float3::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_4(v))
        } else {
            let v = ( { Ok(Box::new(float4::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct HumanGoal {
    pub m_x: xform,
    pub m_weight_t: f32,
    pub m_weight_r: f32,
    pub m_hint_t: Option<HumanGoalMHintTVariant>,
    pub m_hint_weight_t: Option<f32>,
}

impl crate::Parse for HumanGoal {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_x = xform::parse(rdr, ver)?;
        let m_weight_t = rdr.read_f32()?;
        let m_weight_r = rdr.read_f32()?;
        let m_hint_t = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(HumanGoalMHintTVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let m_hint_weight_t = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(HumanGoal {
            m_x,
            m_weight_t,
            m_weight_r,
            m_hint_t,
            m_hint_weight_t,
        })
    }
}
