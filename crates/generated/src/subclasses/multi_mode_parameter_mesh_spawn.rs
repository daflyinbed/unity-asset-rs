//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct MultiModeParameter_MeshSpawn {
    pub value: f32,
    pub mode: i32,
    pub spread: f32,
    pub speed: MinMaxCurve,
}

impl crate::Parse for MultiModeParameter_MeshSpawn {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let value = rdr.read_f32()?;
        let mode = rdr.read_i32()?;
        let spread = rdr.read_f32()?;
        let speed = MinMaxCurve::parse(rdr, ver)?;

        Ok(MultiModeParameter_MeshSpawn {
            value,
            mode,
            spread,
            speed,
        })
    }
}
