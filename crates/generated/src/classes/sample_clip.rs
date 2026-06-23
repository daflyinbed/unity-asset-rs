//! 自动生成。请勿手动修改。

use super::audio_resource::AudioResource;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub enum SampleClipParent {
    NamedObject(NamedObject),
    AudioResource(AudioResource),
}

#[derive(Debug, Clone)]
pub struct SampleClip {
    pub parent: SampleClipParent,
    pub m_name: String,
}

impl crate::Parse for SampleClip {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            SampleClipParent::NamedObject(NamedObject::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            SampleClipParent::AudioResource(AudioResource::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(SampleClip {
            parent,
            m_name,
        })
    }
}
