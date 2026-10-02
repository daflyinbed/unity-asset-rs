//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct RendererData {
    pub uv_mesh: PPtr,
    pub terrain_dynamic_uvst: Vector4f,
    pub terrain_chunk_dynamic_uvst: Vector4f,
    pub lightmap_index: u16,
    pub lightmap_index_dynamic: u16,
    pub lightmap_st: Vector4f,
    pub lightmap_st_dynamic: Vector4f,
    pub explicit_probe_set_hash: Option<Hash128>,
}

impl crate::Parse for RendererData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let uv_mesh = crate::PPtr::parse(rdr, ver)?;
        let terrain_dynamic_uvst = Vector4f::parse(rdr, ver)?;
        let terrain_chunk_dynamic_uvst = Vector4f::parse(rdr, ver)?;
        let lightmap_index = rdr.read_u16()?;
        let lightmap_index_dynamic = rdr.read_u16()?;
        let lightmap_st = Vector4f::parse(rdr, ver)?;
        let lightmap_st_dynamic = Vector4f::parse(rdr, ver)?;
        let explicit_probe_set_hash = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(Hash128::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(RendererData {
            uv_mesh,
            terrain_dynamic_uvst,
            terrain_chunk_dynamic_uvst,
            lightmap_index,
            lightmap_index_dynamic,
            lightmap_st,
            lightmap_st_dynamic,
            explicit_probe_set_hash,
        })
    }
}

impl crate::ToValue for RendererData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("uvMesh".to_string(), self.uv_mesh.to_value())),
            Some((
                "terrainDynamicUVST".to_string(),
                self.terrain_dynamic_uvst.to_value(),
            )),
            Some((
                "terrainChunkDynamicUVST".to_string(),
                self.terrain_chunk_dynamic_uvst.to_value(),
            )),
            Some(("lightmapIndex".to_string(), self.lightmap_index.to_value())),
            Some((
                "lightmapIndexDynamic".to_string(),
                self.lightmap_index_dynamic.to_value(),
            )),
            Some(("lightmapST".to_string(), self.lightmap_st.to_value())),
            Some((
                "lightmapSTDynamic".to_string(),
                self.lightmap_st_dynamic.to_value(),
            )),
            crate::opt_field("explicitProbeSetHash", &self.explicit_probe_set_hash),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "uvMesh" => (self.uv_mesh).inspect_value(path, offset, limit),
                "terrainDynamicUVST" => {
                    (self.terrain_dynamic_uvst).inspect_value(path, offset, limit)
                }
                "terrainChunkDynamicUVST" => {
                    (self.terrain_chunk_dynamic_uvst).inspect_value(path, offset, limit)
                }
                "lightmapIndex" => (self.lightmap_index).inspect_value(path, offset, limit),
                "lightmapIndexDynamic" => {
                    (self.lightmap_index_dynamic).inspect_value(path, offset, limit)
                }
                "lightmapST" => (self.lightmap_st).inspect_value(path, offset, limit),
                "lightmapSTDynamic" => {
                    (self.lightmap_st_dynamic).inspect_value(path, offset, limit)
                }
                "explicitProbeSetHash" => {
                    (self.explicit_probe_set_hash).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.uv_mesh).inspect_value(path, offset, limit)).map(|v| ("uvMesh".to_string(), v)),
            ((self.terrain_dynamic_uvst).inspect_value(path, offset, limit))
                .map(|v| ("terrainDynamicUVST".to_string(), v)),
            ((self.terrain_chunk_dynamic_uvst).inspect_value(path, offset, limit))
                .map(|v| ("terrainChunkDynamicUVST".to_string(), v)),
            ((self.lightmap_index).inspect_value(path, offset, limit))
                .map(|v| ("lightmapIndex".to_string(), v)),
            ((self.lightmap_index_dynamic).inspect_value(path, offset, limit))
                .map(|v| ("lightmapIndexDynamic".to_string(), v)),
            ((self.lightmap_st).inspect_value(path, offset, limit))
                .map(|v| ("lightmapST".to_string(), v)),
            ((self.lightmap_st_dynamic).inspect_value(path, offset, limit))
                .map(|v| ("lightmapSTDynamic".to_string(), v)),
            ((self.explicit_probe_set_hash).inspect_value(path, offset, limit))
                .map(|v| ("explicitProbeSetHash".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
