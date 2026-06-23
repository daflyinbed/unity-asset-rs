//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgba::ColorRgba;

#[derive(Debug, Clone)]
pub struct AlbedoSwatchInfo {
    pub name: String,
    pub color: ColorRgba,
    pub min_luminance: f32,
    pub max_luminance: f32,
}

impl crate::Parse for AlbedoSwatchInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let color = ColorRgba::parse(r, ver)?;
        let min_luminance = r.read_f32()?;
        let max_luminance = r.read_f32()?;
        r.align(4)?;

        Ok(AlbedoSwatchInfo {
            name,
            color,
            min_luminance,
            max_luminance,
        })
    }
}
