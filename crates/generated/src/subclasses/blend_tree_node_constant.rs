//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;

#[derive(Debug, Clone)]
pub struct BlendTreeNodeConstant {
    pub m_blend_event_id: u32,
    pub m_blend_type: Option<u32>,
    pub m_child_indices: Vec<u32>,
    pub m_child_threshold_array: Option<Vec<f32>>,
    pub m_blend_event_yid: Option<u32>,
    pub m_clip_id: u32,
    pub m_duration: f32,
    pub m_blend1d_data: Option<OffsetPtr>,
    pub m_blend2d_data: Option<OffsetPtr>,
    pub m_blend_direct_data: Option<OffsetPtr>,
    pub m_clip_index: Option<u32>,
    pub m_cycle_offset: Option<f32>,
    pub m_mirror: Option<bool>,
}

impl crate::Parse for BlendTreeNodeConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_blend_event_id = None;
        let mut m_blend_type = None;
        let mut m_child_indices = None;
        let mut m_child_threshold_array = None;
        let mut m_blend_event_yid = None;
        let mut m_clip_id = None;
        let mut m_duration = None;
        let mut m_blend1d_data = None;
        let mut m_blend2d_data = None;
        let mut m_blend_direct_data = None;
        let mut m_clip_index = None;
        let mut m_cycle_offset = None;
        let mut m_mirror = None;
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_blend_type = Some(rdr.read_u32()?);
        } else {
        }
        m_blend_event_id = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_blend_event_yid = Some(rdr.read_u32()?);
        } else {
        }
        m_child_indices = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_blend1d_data = Some(OffsetPtr::parse(rdr, ver)?);
            m_blend2d_data = Some(OffsetPtr::parse(rdr, ver)?);
            m_blend_direct_data = Some(OffsetPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_blend1d_data = Some(OffsetPtr::parse(rdr, ver)?);
            m_blend2d_data = Some(OffsetPtr::parse(rdr, ver)?);
        } else {
            m_child_threshold_array = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_f32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        }
        m_clip_id = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else if *ver >= crate::UnityVersion::new(4, 5, 2) {
            m_clip_index = Some(rdr.read_u32()?);
        } else {
        }
        m_duration = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 1, 3) {
            m_cycle_offset = Some(rdr.read_f32()?);
            m_mirror = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(BlendTreeNodeConstant {
            m_blend_event_id: m_blend_event_id.unwrap(),
            m_blend_type,
            m_child_indices: m_child_indices.unwrap(),
            m_child_threshold_array,
            m_blend_event_yid,
            m_clip_id: m_clip_id.unwrap(),
            m_duration: m_duration.unwrap(),
            m_blend1d_data,
            m_blend2d_data,
            m_blend_direct_data,
            m_clip_index,
            m_cycle_offset,
            m_mirror,
        })
    }
}

impl crate::ToValue for BlendTreeNodeConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_BlendEventID".to_string(),
                self.m_blend_event_id.to_value(),
            )),
            crate::opt_field("m_BlendType", &self.m_blend_type),
            Some((
                "m_ChildIndices".to_string(),
                self.m_child_indices.to_value(),
            )),
            crate::opt_field("m_ChildThresholdArray", &self.m_child_threshold_array),
            crate::opt_field("m_BlendEventYID", &self.m_blend_event_yid),
            Some(("m_ClipID".to_string(), self.m_clip_id.to_value())),
            Some(("m_Duration".to_string(), self.m_duration.to_value())),
            crate::opt_field("m_Blend1dData", &self.m_blend1d_data),
            crate::opt_field("m_Blend2dData", &self.m_blend2d_data),
            crate::opt_field("m_BlendDirectData", &self.m_blend_direct_data),
            crate::opt_field("m_ClipIndex", &self.m_clip_index),
            crate::opt_field("m_CycleOffset", &self.m_cycle_offset),
            crate::opt_field("m_Mirror", &self.m_mirror),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BlendEventID" => (self.m_blend_event_id).inspect_value(path, offset, limit),
                "m_BlendType" => (self.m_blend_type).inspect_value(path, offset, limit),
                "m_ChildIndices" => (self.m_child_indices).inspect_value(path, offset, limit),
                "m_ChildThresholdArray" => {
                    (self.m_child_threshold_array).inspect_value(path, offset, limit)
                }
                "m_BlendEventYID" => (self.m_blend_event_yid).inspect_value(path, offset, limit),
                "m_ClipID" => (self.m_clip_id).inspect_value(path, offset, limit),
                "m_Duration" => (self.m_duration).inspect_value(path, offset, limit),
                "m_Blend1dData" => (self.m_blend1d_data).inspect_value(path, offset, limit),
                "m_Blend2dData" => (self.m_blend2d_data).inspect_value(path, offset, limit),
                "m_BlendDirectData" => {
                    (self.m_blend_direct_data).inspect_value(path, offset, limit)
                }
                "m_ClipIndex" => (self.m_clip_index).inspect_value(path, offset, limit),
                "m_CycleOffset" => (self.m_cycle_offset).inspect_value(path, offset, limit),
                "m_Mirror" => (self.m_mirror).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_blend_event_id).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendEventID".to_string(), v)),
            ((self.m_blend_type).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendType".to_string(), v)),
            ((self.m_child_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_ChildIndices".to_string(), v)),
            ((self.m_child_threshold_array).inspect_value(path, offset, limit))
                .map(|v| ("m_ChildThresholdArray".to_string(), v)),
            ((self.m_blend_event_yid).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendEventYID".to_string(), v)),
            ((self.m_clip_id).inspect_value(path, offset, limit))
                .map(|v| ("m_ClipID".to_string(), v)),
            ((self.m_duration).inspect_value(path, offset, limit))
                .map(|v| ("m_Duration".to_string(), v)),
            ((self.m_blend1d_data).inspect_value(path, offset, limit))
                .map(|v| ("m_Blend1dData".to_string(), v)),
            ((self.m_blend2d_data).inspect_value(path, offset, limit))
                .map(|v| ("m_Blend2dData".to_string(), v)),
            ((self.m_blend_direct_data).inspect_value(path, offset, limit))
                .map(|v| ("m_BlendDirectData".to_string(), v)),
            ((self.m_clip_index).inspect_value(path, offset, limit))
                .map(|v| ("m_ClipIndex".to_string(), v)),
            ((self.m_cycle_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_CycleOffset".to_string(), v)),
            ((self.m_mirror).inspect_value(path, offset, limit))
                .map(|v| ("m_Mirror".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
