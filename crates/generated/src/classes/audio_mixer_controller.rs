//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::audio_mixer_constant::AudioMixerConstant;


/// 版本枚举：AudioMixerControllerMSnapshotsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioMixerControllerMSnapshotsVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AudioMixerControllerMSnapshotsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioMixerController {
    pub m_name: String,
    pub m_output_group: PPtr,
    pub m_master_group: PPtr,
    pub m_snapshots: AudioMixerControllerMSnapshotsVariant,
    pub m_start_snapshot: PPtr,
    pub m_suspend_threshold: f32,
    pub m_enable_suspend: bool,
    pub m_mixer_constant: AudioMixerConstant,
    pub m_update_mode: Option<i32>,
}

impl crate::Parse for AudioMixerController {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_output_group = None;
        let mut m_master_group = None;
        let mut m_snapshots = None;
        let mut m_start_snapshot = None;
        let mut m_suspend_threshold = None;
        let mut m_enable_suspend = None;
        let mut m_mixer_constant = None;
        let mut m_update_mode = None;
            m_name = Some(rdr.read_aligned_string()?);
            m_output_group = Some(crate::PPtr::parse(rdr, ver)?);
            m_master_group = Some(crate::PPtr::parse(rdr, ver)?);
            m_snapshots = Some(AudioMixerControllerMSnapshotsVariant::parse(rdr, ver)?);
            m_start_snapshot = Some(crate::PPtr::parse(rdr, ver)?);
            m_suspend_threshold = Some(rdr.read_f32()?);
            m_enable_suspend = Some(rdr.read_bool()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 3, 6) {
            m_update_mode = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else {
        }
            m_mixer_constant = Some(AudioMixerConstant::parse(rdr, ver)?);
            rdr.align(4)?;

        Ok(AudioMixerController {
            m_name: m_name.unwrap(),
            m_output_group: m_output_group.unwrap(),
            m_master_group: m_master_group.unwrap(),
            m_snapshots: m_snapshots.unwrap(),
            m_start_snapshot: m_start_snapshot.unwrap(),
            m_suspend_threshold: m_suspend_threshold.unwrap(),
            m_enable_suspend: m_enable_suspend.unwrap(),
            m_mixer_constant: m_mixer_constant.unwrap(),
            m_update_mode,
        })
    }
}
