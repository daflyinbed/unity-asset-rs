//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioMixer {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_output_group: PPtr,
    pub m_master_group: PPtr,
    pub m_snapshots: Vec<PPtr>,
    pub m_start_snapshot: PPtr,
    pub m_suspend_threshold: f32,
    pub m_enable_suspend: bool,
}

impl crate::Parse for AudioMixer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_output_group = crate::PPtr::parse(r, ver)?;
        let m_master_group = crate::PPtr::parse(r, ver)?;
        let m_snapshots = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_start_snapshot = crate::PPtr::parse(r, ver)?;
        let m_suspend_threshold = r.read_f32()?;
        let m_enable_suspend = r.read_bool()?;
        r.align(4)?;

        Ok(AudioMixer {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_output_group,
            m_master_group,
            m_snapshots,
            m_start_snapshot,
            m_suspend_threshold,
            m_enable_suspend,
        })
    }
}
