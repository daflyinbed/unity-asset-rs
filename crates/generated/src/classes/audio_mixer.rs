//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::audio_mixer_constant::AudioMixerConstant;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct AudioMixer {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_output_group: PPtr,
    pub m_master_group: PPtr,
    pub m_snapshots: Vec<PPtr>,
    pub m_start_snapshot: PPtr,
    pub m_suspend_threshold: f32,
    pub m_enable_suspend: bool,
    pub m_mixer_constant: AudioMixerConstant,
    pub m_update_mode: Option<i32>,
}

impl crate::Parse for AudioMixer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_output_group = crate::PPtr::parse(r, ver)?;
        let m_master_group = crate::PPtr::parse(r, ver)?;
        let m_snapshots = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_start_snapshot = crate::PPtr::parse(r, ver)?;
        let m_suspend_threshold = r.read_f32()?;
        let m_enable_suspend = r.read_bool()?;
        r.align(4)?;
        let m_mixer_constant = AudioMixerConstant::parse(r, ver)?;
        r.align(4)?;
        let m_update_mode = if *ver >= crate::UnityVersion::new(5, 3, 6) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(AudioMixer {
            parent,
            m_name,
            m_output_group,
            m_master_group,
            m_snapshots,
            m_start_snapshot,
            m_suspend_threshold,
            m_enable_suspend,
            m_mixer_constant,
            m_update_mode,
        })
    }
}
