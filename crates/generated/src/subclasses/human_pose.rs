//! 自动生成。请勿手动修改。

use crate::subclasses::float4::float4;
use crate::subclasses::hand_pose::HandPose;
use crate::subclasses::human_goal::HumanGoal;
use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct HumanPose {
    pub m_root_x: xform,
    pub m_look_at_position: float4,
    pub m_look_at_weight: float4,
    pub m_goal_array: Vec<HumanGoal>,
    pub m_left_hand_pose: HandPose,
    pub m_right_hand_pose: HandPose,
    pub m_do_f_array: Vec<f32>,
}

impl crate::Parse for HumanPose {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_root_x = xform::parse(r, ver)?;
        let m_look_at_position = float4::parse(r, ver)?;
        let m_look_at_weight = float4::parse(r, ver)?;
        let m_goal_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { HumanGoal::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_left_hand_pose = HandPose::parse(r, ver)?;
        let m_right_hand_pose = HandPose::parse(r, ver)?;
        let m_do_f_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(HumanPose {
            m_root_x,
            m_look_at_position,
            m_look_at_weight,
            m_goal_array,
            m_left_hand_pose,
            m_right_hand_pose,
            m_do_f_array,
        })
    }
}
