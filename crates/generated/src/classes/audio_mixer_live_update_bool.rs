//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AudioMixerLiveUpdateBool {
}

impl crate::Parse for AudioMixerLiveUpdateBool {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {

        Ok(AudioMixerLiveUpdateBool {
        })
    }
}
