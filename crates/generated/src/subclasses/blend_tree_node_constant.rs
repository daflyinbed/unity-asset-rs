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
            m_child_indices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_blend1d_data = Some(OffsetPtr::parse(rdr, ver)?);
            m_blend2d_data = Some(OffsetPtr::parse(rdr, ver)?);
            m_blend_direct_data = Some(OffsetPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            m_blend1d_data = Some(OffsetPtr::parse(rdr, ver)?);
            m_blend2d_data = Some(OffsetPtr::parse(rdr, ver)?);
        } else {
            m_child_threshold_array = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
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
