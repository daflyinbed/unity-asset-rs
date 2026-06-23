//! 自动生成。请勿手动修改。

use crate::subclasses::lod_renderer::LodRenderer;

#[derive(Debug, Clone)]
pub struct Lod {
    pub screen_relative_height: f32,
    pub renderers: Vec<LodRenderer>,
}

impl crate::Parse for Lod {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let screen_relative_height = r.read_f32()?;
        let renderers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LodRenderer::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(Lod {
            screen_relative_height,
            renderers,
        })
    }
}
