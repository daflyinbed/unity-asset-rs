//! 自动生成。请勿手动修改。

use crate::subclasses::lod_renderer::LodRenderer;

/// 版本枚举：LodRenderersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LodRenderersVariant {
    Default(Vec<LodRenderer>),
    V2017_1(Vec<LodRenderer>),
}

impl crate::Parse for LodRenderersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| LodRenderer::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| LodRenderer::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Lod {
    pub screen_relative_height: f32,
    pub renderers: LodRenderersVariant,
    pub fade_mode: Option<i32>,
    pub fade_transition_width: Option<f32>,
}

impl crate::Parse for Lod {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut screen_relative_height = None;
        let mut renderers = None;
        let mut fade_mode = None;
        let mut fade_transition_width = None;
        screen_relative_height = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            fade_transition_width = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            fade_mode = Some(rdr.read_i32()?);
            fade_transition_width = Some(rdr.read_f32()?);
        } else {
        }
        renderers = Some(LodRenderersVariant::parse(rdr, ver)?);

        Ok(Lod {
            screen_relative_height: screen_relative_height.unwrap(),
            renderers: renderers.unwrap(),
            fade_mode,
            fade_transition_width,
        })
    }
}
