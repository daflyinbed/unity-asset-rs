//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::audio_mixer_group_view::AudioMixerGroupView;
use crate::subclasses::exposed_audio_parameter::ExposedAudioParameter;

#[derive(Debug, Clone)]
pub struct AudioMixerController {
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
    pub m_exposed_parameters: Vec<ExposedAudioParameter>,
    pub m_audio_mixer_group_views: Vec<AudioMixerGroupView>,
    pub m_current_view_index: i32,
    pub m_target_snapshot: PPtr,
}

impl crate::Parse for AudioMixerController {
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
        let m_exposed_parameters = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ExposedAudioParameter::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_audio_mixer_group_views = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AudioMixerGroupView::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_current_view_index = r.read_i32()?;
        let m_target_snapshot = crate::PPtr::parse(r, ver)?;
        r.align(4)?;

        Ok(AudioMixerController {
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
            m_exposed_parameters,
            m_audio_mixer_group_views,
            m_current_view_index,
            m_target_snapshot,
        })
    }
}
