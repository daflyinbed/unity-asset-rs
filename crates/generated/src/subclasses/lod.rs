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

impl crate::ToValue for LodRenderersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for Lod {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "screenRelativeHeight".to_string(),
                self.screen_relative_height.to_value(),
            )),
            Some(("renderers".to_string(), self.renderers.to_value())),
            crate::opt_field("fadeMode", &self.fade_mode),
            crate::opt_field("fadeTransitionWidth", &self.fade_transition_width),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "screenRelativeHeight" => {
                    (self.screen_relative_height).inspect_value(path, offset, limit)
                }
                "renderers" => (self.renderers).inspect_value(path, offset, limit),
                "fadeMode" => (self.fade_mode).inspect_value(path, offset, limit),
                "fadeTransitionWidth" => {
                    (self.fade_transition_width).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.screen_relative_height).inspect_value(path, offset, limit))
                .map(|v| ("screenRelativeHeight".to_string(), v)),
            ((self.renderers).inspect_value(path, offset, limit))
                .map(|v| ("renderers".to_string(), v)),
            ((self.fade_mode).inspect_value(path, offset, limit))
                .map(|v| ("fadeMode".to_string(), v)),
            ((self.fade_transition_width).inspect_value(path, offset, limit))
                .map(|v| ("fadeTransitionWidth".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
