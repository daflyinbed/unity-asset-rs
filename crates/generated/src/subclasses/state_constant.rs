//! 自动生成。请勿手动修改。

use crate::subclasses::leaf_info_constant::LeafInfoConstant;
use crate::subclasses::offset_ptr::OffsetPtr;

/// 版本枚举：StateConstantMLoopVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StateConstantMLoopVariant {
    Default(bool),
    V4_1(bool),
}

impl crate::Parse for StateConstantMLoopVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for StateConstantMLoopVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct StateConstant {
    pub m_transition_constant_array: Vec<OffsetPtr>,
    pub m_blend_tree_constant_index_array: Vec<i32>,
    pub m_leaf_info_array: Option<Vec<LeafInfoConstant>>,
    pub m_blend_tree_constant_array: Vec<OffsetPtr>,
    pub m_name_id: Option<u32>,
    pub m_id: Option<u32>,
    pub m_path_id: Option<u32>,
    pub m_tag_id: u32,
    pub m_full_path_id: Option<u32>,
    pub m_speed: f32,
    pub m_ik_on_feet: bool,
    pub m_cycle_offset: Option<f32>,
    pub m_speed_param_id: Option<u32>,
    pub m_loop: StateConstantMLoopVariant,
    pub m_mirror_param_id: Option<u32>,
    pub m_cycle_offset_param_id: Option<u32>,
    pub m_mirror: Option<bool>,
    pub m_time_param_id: Option<u32>,
    pub m_write_default_values: Option<bool>,
}

impl crate::Parse for StateConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_transition_constant_array = None;
        let mut m_blend_tree_constant_index_array = None;
        let mut m_leaf_info_array = None;
        let mut m_blend_tree_constant_array = None;
        let mut m_name_id = None;
        let mut m_id = None;
        let mut m_path_id = None;
        let mut m_tag_id = None;
        let mut m_full_path_id = None;
        let mut m_speed = None;
        let mut m_ik_on_feet = None;
        let mut m_cycle_offset = None;
        let mut m_speed_param_id = None;
        let mut m_loop = None;
        let mut m_mirror_param_id = None;
        let mut m_cycle_offset_param_id = None;
        let mut m_mirror = None;
        let mut m_time_param_id = None;
        let mut m_write_default_values = None;
        m_transition_constant_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| OffsetPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        m_blend_tree_constant_index_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_i32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
        } else {
            m_leaf_info_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| LeafInfoConstant::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }
        m_blend_tree_constant_array = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| OffsetPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_name_id = Some(rdr.read_u32()?);
            m_path_id = Some(rdr.read_u32()?);
            m_full_path_id = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_name_id = Some(rdr.read_u32()?);
            m_path_id = Some(rdr.read_u32()?);
        } else {
            m_id = Some(rdr.read_u32()?);
        }
        m_tag_id = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_speed_param_id = Some(rdr.read_u32()?);
            m_mirror_param_id = Some(rdr.read_u32()?);
            m_cycle_offset_param_id = Some(rdr.read_u32()?);
            m_time_param_id = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_speed_param_id = Some(rdr.read_u32()?);
            m_mirror_param_id = Some(rdr.read_u32()?);
            m_cycle_offset_param_id = Some(rdr.read_u32()?);
        } else {
        }
        m_speed = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_cycle_offset = Some(rdr.read_f32()?);
        } else {
        }
        m_ik_on_feet = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_write_default_values = Some(rdr.read_bool()?);
        } else {
        }
        m_loop = Some(StateConstantMLoopVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_mirror = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(StateConstant {
            m_transition_constant_array: m_transition_constant_array.unwrap(),
            m_blend_tree_constant_index_array: m_blend_tree_constant_index_array.unwrap(),
            m_leaf_info_array,
            m_blend_tree_constant_array: m_blend_tree_constant_array.unwrap(),
            m_name_id,
            m_id,
            m_path_id,
            m_tag_id: m_tag_id.unwrap(),
            m_full_path_id,
            m_speed: m_speed.unwrap(),
            m_ik_on_feet: m_ik_on_feet.unwrap(),
            m_cycle_offset,
            m_speed_param_id,
            m_loop: m_loop.unwrap(),
            m_mirror_param_id,
            m_cycle_offset_param_id,
            m_mirror,
            m_time_param_id,
            m_write_default_values,
        })
    }
}

impl crate::ToValue for StateConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_TransitionConstantArray".to_string(),
                self.m_transition_constant_array.to_value(),
            )),
            Some((
                "m_BlendTreeConstantIndexArray".to_string(),
                self.m_blend_tree_constant_index_array.to_value(),
            )),
            crate::opt_field("m_LeafInfoArray", &self.m_leaf_info_array),
            Some((
                "m_BlendTreeConstantArray".to_string(),
                self.m_blend_tree_constant_array.to_value(),
            )),
            crate::opt_field("m_NameID", &self.m_name_id),
            crate::opt_field("m_ID", &self.m_id),
            crate::opt_field("m_PathID", &self.m_path_id),
            Some(("m_TagID".to_string(), self.m_tag_id.to_value())),
            crate::opt_field("m_FullPathID", &self.m_full_path_id),
            Some(("m_Speed".to_string(), self.m_speed.to_value())),
            Some(("m_IKOnFeet".to_string(), self.m_ik_on_feet.to_value())),
            crate::opt_field("m_CycleOffset", &self.m_cycle_offset),
            crate::opt_field("m_SpeedParamID", &self.m_speed_param_id),
            Some(("m_Loop".to_string(), self.m_loop.to_value())),
            crate::opt_field("m_MirrorParamID", &self.m_mirror_param_id),
            crate::opt_field("m_CycleOffsetParamID", &self.m_cycle_offset_param_id),
            crate::opt_field("m_Mirror", &self.m_mirror),
            crate::opt_field("m_TimeParamID", &self.m_time_param_id),
            crate::opt_field("m_WriteDefaultValues", &self.m_write_default_values),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_TransitionConstantArray" => {
                    (self.m_transition_constant_array).inspect_value(path, offset, limit)
                }
                "m_BlendTreeConstantIndexArray" => {
                    (self.m_blend_tree_constant_index_array).inspect_value(path, offset, limit)
                }
                "m_LeafInfoArray" => (self.m_leaf_info_array).inspect_value(path, offset, limit),
                "m_BlendTreeConstantArray" => {
                    (self.m_blend_tree_constant_array).inspect_value(path, offset, limit)
                }
                "m_NameID" => (self.m_name_id).inspect_value(path, offset, limit),
                "m_ID" => (self.m_id).inspect_value(path, offset, limit),
                "m_PathID" => (self.m_path_id).inspect_value(path, offset, limit),
                "m_TagID" => (self.m_tag_id).inspect_value(path, offset, limit),
                "m_FullPathID" => (self.m_full_path_id).inspect_value(path, offset, limit),
                "m_Speed" => (self.m_speed).inspect_value(path, offset, limit),
                "m_IKOnFeet" => (self.m_ik_on_feet).inspect_value(path, offset, limit),
                "m_CycleOffset" => (self.m_cycle_offset).inspect_value(path, offset, limit),
                "m_SpeedParamID" => (self.m_speed_param_id).inspect_value(path, offset, limit),
                "m_Loop" => (self.m_loop).inspect_value(path, offset, limit),
                "m_MirrorParamID" => (self.m_mirror_param_id).inspect_value(path, offset, limit),
                "m_CycleOffsetParamID" => {
                    (self.m_cycle_offset_param_id).inspect_value(path, offset, limit)
                }
                "m_Mirror" => (self.m_mirror).inspect_value(path, offset, limit),
                "m_TimeParamID" => (self.m_time_param_id).inspect_value(path, offset, limit),
                "m_WriteDefaultValues" => {
                    (self.m_write_default_values).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_transition_constant_array).inspect_value(path, offset, limit))
                .map(|v| ("m_TransitionConstantArray".to_string(), v)),
            ((self.m_blend_tree_constant_index_array).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendTreeConstantIndexArray".to_string(), v)),
            ((self.m_leaf_info_array).inspect_value(path, offset, limit))
                .map(|v| ("m_LeafInfoArray".to_string(), v)),
            ((self.m_blend_tree_constant_array).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendTreeConstantArray".to_string(), v)),
            ((self.m_name_id).inspect_value(path, offset, limit))
                .map(|v| ("m_NameID".to_string(), v)),
            ((self.m_id).inspect_value(path, offset, limit)).map(|v| ("m_ID".to_string(), v)),
            ((self.m_path_id).inspect_value(path, offset, limit))
                .map(|v| ("m_PathID".to_string(), v)),
            ((self.m_tag_id).inspect_value(path, offset, limit))
                .map(|v| ("m_TagID".to_string(), v)),
            ((self.m_full_path_id).inspect_value(path, offset, limit))
                .map(|v| ("m_FullPathID".to_string(), v)),
            ((self.m_speed).inspect_value(path, offset, limit)).map(|v| ("m_Speed".to_string(), v)),
            ((self.m_ik_on_feet).inspect_value(path, offset, limit))
                .map(|v| ("m_IKOnFeet".to_string(), v)),
            ((self.m_cycle_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_CycleOffset".to_string(), v)),
            ((self.m_speed_param_id).inspect_value(path, offset, limit))
                .map(|v| ("m_SpeedParamID".to_string(), v)),
            ((self.m_loop).inspect_value(path, offset, limit)).map(|v| ("m_Loop".to_string(), v)),
            ((self.m_mirror_param_id).inspect_value(path, offset, limit))
                .map(|v| ("m_MirrorParamID".to_string(), v)),
            ((self.m_cycle_offset_param_id).inspect_value(path, offset, limit))
                .map(|v| ("m_CycleOffsetParamID".to_string(), v)),
            ((self.m_mirror).inspect_value(path, offset, limit))
                .map(|v| ("m_Mirror".to_string(), v)),
            ((self.m_time_param_id).inspect_value(path, offset, limit))
                .map(|v| ("m_TimeParamID".to_string(), v)),
            ((self.m_write_default_values).inspect_value(path, offset, limit))
                .map(|v| ("m_WriteDefaultValues".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
