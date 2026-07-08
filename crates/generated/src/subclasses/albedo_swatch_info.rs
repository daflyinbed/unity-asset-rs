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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let color = ColorRgba::parse(rdr, ver)?;
        let min_luminance = rdr.read_f32()?;
        let max_luminance = rdr.read_f32()?;
        rdr.align(4)?;

        Ok(AlbedoSwatchInfo {
            name,
            color,
            min_luminance,
            max_luminance,
        })
    }
}
