//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct OcclusionScene {
    pub index_renderers: i32,
    pub size_renderers: i32,
    pub index_portals: i32,
    pub size_portals: i32,
    pub scene: Guid,
}

impl crate::Parse for OcclusionScene {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let index_renderers = rdr.read_i32()?;
        let size_renderers = rdr.read_i32()?;
        let index_portals = rdr.read_i32()?;
        let size_portals = rdr.read_i32()?;
        let scene = Guid::parse(rdr, ver)?;

        Ok(OcclusionScene {
            index_renderers,
            size_renderers,
            index_portals,
            size_portals,
            scene,
        })
    }
}

impl crate::ToValue for OcclusionScene {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "indexRenderers".to_string(),
                self.index_renderers.to_value(),
            )),
            Some(("sizeRenderers".to_string(), self.size_renderers.to_value())),
            Some(("indexPortals".to_string(), self.index_portals.to_value())),
            Some(("sizePortals".to_string(), self.size_portals.to_value())),
            Some(("scene".to_string(), self.scene.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "indexRenderers" => (self.index_renderers).inspect_value(path, offset, limit),
                "sizeRenderers" => (self.size_renderers).inspect_value(path, offset, limit),
                "indexPortals" => (self.index_portals).inspect_value(path, offset, limit),
                "sizePortals" => (self.size_portals).inspect_value(path, offset, limit),
                "scene" => (self.scene).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.index_renderers).inspect_value(path, offset, limit))
                .map(|v| ("indexRenderers".to_string(), v)),
            ((self.size_renderers).inspect_value(path, offset, limit))
                .map(|v| ("sizeRenderers".to_string(), v)),
            ((self.index_portals).inspect_value(path, offset, limit))
                .map(|v| ("indexPortals".to_string(), v)),
            ((self.size_portals).inspect_value(path, offset, limit))
                .map(|v| ("sizePortals".to_string(), v)),
            ((self.scene).inspect_value(path, offset, limit)).map(|v| ("scene".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
