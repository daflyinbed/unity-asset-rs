//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct EnlightenRendererInformation {
    pub renderer: PPtr,
    pub dynamic_lightmap_st_in_system: Vector4f,
    pub system_id: i32,
    pub instance_hash: Hash128,
}

impl crate::Parse for EnlightenRendererInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let renderer = crate::PPtr::parse(rdr, ver)?;
        let dynamic_lightmap_st_in_system = Vector4f::parse(rdr, ver)?;
        let system_id = rdr.read_i32()?;
        let instance_hash = Hash128::parse(rdr, ver)?;

        Ok(EnlightenRendererInformation {
            renderer,
            dynamic_lightmap_st_in_system,
            system_id,
            instance_hash,
        })
    }
}

impl crate::ToValue for EnlightenRendererInformation {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("renderer".to_string(), self.renderer.to_value())),
            Some((
                "dynamicLightmapSTInSystem".to_string(),
                self.dynamic_lightmap_st_in_system.to_value(),
            )),
            Some(("systemId".to_string(), self.system_id.to_value())),
            Some(("instanceHash".to_string(), self.instance_hash.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "renderer" => (self.renderer).inspect_value(path, offset, limit),
                "dynamicLightmapSTInSystem" => {
                    (self.dynamic_lightmap_st_in_system).inspect_value(path, offset, limit)
                }
                "systemId" => (self.system_id).inspect_value(path, offset, limit),
                "instanceHash" => (self.instance_hash).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.renderer).inspect_value(path, offset, limit))
                .map(|v| ("renderer".to_string(), v)),
            ((self.dynamic_lightmap_st_in_system).inspect_value(path, offset, limit))
                .map(|v| ("dynamicLightmapSTInSystem".to_string(), v)),
            ((self.system_id).inspect_value(path, offset, limit))
                .map(|v| ("systemId".to_string(), v)),
            ((self.instance_hash).inspect_value(path, offset, limit))
                .map(|v| ("instanceHash".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
