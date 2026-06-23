//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_gradient::MinMaxGradient;

#[derive(Debug, Clone)]
pub struct ColorModule {
    pub enabled: bool,
    pub gradient: MinMaxGradient,
}

impl crate::Parse for ColorModule {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let gradient = MinMaxGradient::parse(r, ver)?;
        r.align(4)?;

        Ok(ColorModule {
            enabled,
            gradient,
        })
    }
}
