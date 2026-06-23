//! 自动生成。请勿手动修改。

use crate::subclasses::child::Child;
use super::motion::Motion;

#[derive(Debug, Clone)]
pub struct BlendTree {
    pub parent: Motion,
    pub m_name: String,
    pub m_childs: Vec<Child>,
    pub m_blend_event: Option<String>,
    pub m_blend_parameter: Option<String>,
    pub m_min_threshold: f32,
    pub m_blend_event_y: Option<String>,
    pub m_blend_parameter_y: Option<String>,
    pub m_max_threshold: f32,
    pub m_use_automatic_thresholds: bool,
    pub m_blend_type: Option<u32>,
    pub m_normalized_blend_values: Option<bool>,
}

impl crate::Parse for BlendTree {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Motion::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_childs = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Child::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_blend_event = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_blend_parameter = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_min_threshold = r.read_f32()?;
        let m_blend_event_y = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_blend_parameter_y = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_max_threshold = r.read_f32()?;
        let m_use_automatic_thresholds = r.read_bool()?;
        r.align(4)?;
        let m_blend_type = if *ver >= crate::UnityVersion::new(4, 1, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_normalized_blend_values = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(BlendTree {
            parent,
            m_name,
            m_childs,
            m_blend_event,
            m_blend_parameter,
            m_min_threshold,
            m_blend_event_y,
            m_blend_parameter_y,
            m_max_threshold,
            m_use_automatic_thresholds,
            m_blend_type,
            m_normalized_blend_values,
        })
    }
}
