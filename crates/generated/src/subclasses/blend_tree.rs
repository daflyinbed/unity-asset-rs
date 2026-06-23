//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::child::Child;

#[derive(Debug, Clone)]
pub struct BlendTree {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_childs: Vec<Child>,
    pub m_blend_event: String,
    pub m_min_threshold: f32,
    pub m_max_threshold: f32,
    pub m_use_automatic_thresholds: bool,
}

impl crate::Parse for BlendTree {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_childs = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Child::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_blend_event = r.read_aligned_string()?;
        r.align(4)?;
        let m_min_threshold = r.read_f32()?;
        let m_max_threshold = r.read_f32()?;
        let m_use_automatic_thresholds = r.read_bool()?;
        r.align(4)?;

        Ok(BlendTree {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_childs,
            m_blend_event,
            m_min_threshold,
            m_max_threshold,
            m_use_automatic_thresholds,
        })
    }
}
