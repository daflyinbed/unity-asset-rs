//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

/// 版本枚举：AudioMixerGroupMChildrenVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioMixerGroupMChildrenVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AudioMixerGroupMChildrenVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AudioMixerGroupMChildrenVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioMixerGroup {
    pub m_name: String,
    pub m_audio_mixer: PPtr,
    pub m_group_id: Guid,
    pub m_children: AudioMixerGroupMChildrenVariant,
}

impl crate::Parse for AudioMixerGroup {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_audio_mixer = crate::PPtr::parse(rdr, ver)?;
        let m_group_id = Guid::parse(rdr, ver)?;
        let m_children = AudioMixerGroupMChildrenVariant::parse(rdr, ver)?;

        Ok(AudioMixerGroup {
            m_name,
            m_audio_mixer,
            m_group_id,
            m_children,
        })
    }
}

impl crate::ToValue for AudioMixerGroup {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_AudioMixer".to_string(), self.m_audio_mixer.to_value())),
            Some(("m_GroupID".to_string(), self.m_group_id.to_value())),
            Some(("m_Children".to_string(), self.m_children.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_AudioMixer" => (self.m_audio_mixer).inspect_value(path, offset, limit),
                "m_GroupID" => (self.m_group_id).inspect_value(path, offset, limit),
                "m_Children" => (self.m_children).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_audio_mixer).inspect_value(path, offset, limit))
                .map(|v| ("m_AudioMixer".to_string(), v)),
            ((self.m_group_id).inspect_value(path, offset, limit))
                .map(|v| ("m_GroupID".to_string(), v)),
            ((self.m_children).inspect_value(path, offset, limit))
                .map(|v| ("m_Children".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
