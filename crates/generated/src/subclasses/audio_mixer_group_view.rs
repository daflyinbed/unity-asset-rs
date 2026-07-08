//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct AudioMixerGroupView {
    pub guids: Vec<Guid>,
    pub name: String,
}

impl crate::Parse for AudioMixerGroupView {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guids = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let name = rdr.read_aligned_string()?;

        Ok(AudioMixerGroupView {
            guids,
            name,
        })
    }
}
