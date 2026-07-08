//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_gradient::MinMaxGradient;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct ColorBySpeedModule {
    pub enabled: bool,
    pub gradient: MinMaxGradient,
    pub range: Vector2f,
}

impl crate::Parse for ColorBySpeedModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let gradient = MinMaxGradient::parse(rdr, ver)?;
        let range = Vector2f::parse(rdr, ver)?;

        Ok(ColorBySpeedModule {
            enabled,
            gradient,
            range,
        })
    }
}
