//! 自动生成。请勿手动修改。

use crate::subclasses::float3::float3;
use crate::subclasses::float4::float4;
use crate::subclasses::hand_pose::HandPose;
use crate::subclasses::human_goal::HumanGoal;
use crate::subclasses::xform::xform;

/// 版本枚举：HumanPoseMLookAtPositionVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HumanPoseMLookAtPositionVariant {
    Default(Box<float4>),
    V5_4(Box<float3>),
}

impl crate::Parse for HumanPoseMLookAtPositionVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ({ Ok(Box::new(float3::parse(rdr, ver)?)) })?;
            Ok(Self::V5_4(v))
        } else {
            let v = ({ Ok(Box::new(float4::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for HumanPoseMLookAtPositionVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：HumanPoseMTDoFArrayVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HumanPoseMTDoFArrayVariant {
    Default(Vec<float4>),
    V5_4(Vec<float3>),
}

impl crate::Parse for HumanPoseMTDoFArrayVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| float3::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_4(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| float4::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for HumanPoseMTDoFArrayVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct HumanPose {
    pub m_root_x: xform,
    pub m_look_at_position: HumanPoseMLookAtPositionVariant,
    pub m_look_at_weight: float4,
    pub m_goal_array: Vec<HumanGoal>,
    pub m_left_hand_pose: HandPose,
    pub m_right_hand_pose: HandPose,
    pub m_do_f_array: Vec<f32>,
    pub m_t_do_f_array: Option<HumanPoseMTDoFArrayVariant>,
}

impl crate::Parse for HumanPose {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_root_x = xform::parse(rdr, ver)?;
        let m_look_at_position = HumanPoseMLookAtPositionVariant::parse(rdr, ver)?;
        let m_look_at_weight = float4::parse(rdr, ver)?;
        let m_goal_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| HumanGoal::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_left_hand_pose = HandPose::parse(rdr, ver)?;
        let m_right_hand_pose = HandPose::parse(rdr, ver)?;
        let m_do_f_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_f32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_t_do_f_array = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(HumanPoseMTDoFArrayVariant::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(HumanPose {
            m_root_x,
            m_look_at_position,
            m_look_at_weight,
            m_goal_array,
            m_left_hand_pose,
            m_right_hand_pose,
            m_do_f_array,
            m_t_do_f_array,
        })
    }
}

impl crate::ToValue for HumanPose {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_RootX".to_string(), self.m_root_x.to_value())),
            Some((
                "m_LookAtPosition".to_string(),
                self.m_look_at_position.to_value(),
            )),
            Some((
                "m_LookAtWeight".to_string(),
                self.m_look_at_weight.to_value(),
            )),
            Some(("m_GoalArray".to_string(), self.m_goal_array.to_value())),
            Some((
                "m_LeftHandPose".to_string(),
                self.m_left_hand_pose.to_value(),
            )),
            Some((
                "m_RightHandPose".to_string(),
                self.m_right_hand_pose.to_value(),
            )),
            Some(("m_DoFArray".to_string(), self.m_do_f_array.to_value())),
            crate::opt_field("m_TDoFArray", &self.m_t_do_f_array),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_RootX" => (self.m_root_x).inspect_value(path, offset, limit),
                "m_LookAtPosition" => (self.m_look_at_position).inspect_value(path, offset, limit),
                "m_LookAtWeight" => (self.m_look_at_weight).inspect_value(path, offset, limit),
                "m_GoalArray" => (self.m_goal_array).inspect_value(path, offset, limit),
                "m_LeftHandPose" => (self.m_left_hand_pose).inspect_value(path, offset, limit),
                "m_RightHandPose" => (self.m_right_hand_pose).inspect_value(path, offset, limit),
                "m_DoFArray" => (self.m_do_f_array).inspect_value(path, offset, limit),
                "m_TDoFArray" => (self.m_t_do_f_array).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_root_x).inspect_value(path, offset, limit))
                .map(|v| ("m_RootX".to_string(), v)),
            ((self.m_look_at_position).inspect_value(path, offset, limit))
                .map(|v| ("m_LookAtPosition".to_string(), v)),
            ((self.m_look_at_weight).inspect_value(path, offset, limit))
                .map(|v| ("m_LookAtWeight".to_string(), v)),
            ((self.m_goal_array).inspect_value(path, offset, limit))
                .map(|v| ("m_GoalArray".to_string(), v)),
            ((self.m_left_hand_pose).inspect_value(path, offset, limit))
                .map(|v| ("m_LeftHandPose".to_string(), v)),
            ((self.m_right_hand_pose).inspect_value(path, offset, limit))
                .map(|v| ("m_RightHandPose".to_string(), v)),
            ((self.m_do_f_array).inspect_value(path, offset, limit))
                .map(|v| ("m_DoFArray".to_string(), v)),
            ((self.m_t_do_f_array).inspect_value(path, offset, limit))
                .map(|v| ("m_TDoFArray".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
