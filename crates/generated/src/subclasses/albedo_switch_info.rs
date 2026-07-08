//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgb_af::ColorRgbAf;

#[derive(Debug, Clone)]
pub struct AlbedoSwitchInfo {
    pub name: String,
    pub color: ColorRgbAf,
    pub min_luminance: f32,
    pub max_luminance: f32,
}

impl crate::Parse for AlbedoSwitchInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let color = ColorRgbAf::parse(rdr, ver)?;
        let min_luminance = rdr.read_f32()?;
        let max_luminance = rdr.read_f32()?;
        rdr.align(4)?;

        Ok(AlbedoSwitchInfo {
            name,
            color,
            min_luminance,
            max_luminance,
        })
    }
}
