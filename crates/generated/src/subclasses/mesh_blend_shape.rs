//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：MeshBlendShapeHasTangentsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MeshBlendShapeHasTangentsVariant {
    Default(bool),
    V4_3(bool),
}

impl crate::Parse for MeshBlendShapeHasTangentsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V4_3(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for MeshBlendShapeHasTangentsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MeshBlendShape {
    pub name: Option<String>,
    pub first_vertex: u32,
    pub vertex_count: u32,
    pub has_normals: bool,
    pub aabb_min_delta: Option<Vector3f>,
    pub has_tangents: MeshBlendShapeHasTangentsVariant,
    pub aabb_max_delta: Option<Vector3f>,
}

impl crate::Parse for MeshBlendShape {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut first_vertex = None;
        let mut vertex_count = None;
        let mut has_normals = None;
        let mut aabb_min_delta = None;
        let mut has_tangents = None;
        let mut aabb_max_delta = None;
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
        } else {
            name = Some(rdr.read_aligned_string()?);
        }
        first_vertex = Some(rdr.read_u32()?);
        vertex_count = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
        } else {
            aabb_min_delta = Some(Vector3f::parse(rdr, ver)?);
            aabb_max_delta = Some(Vector3f::parse(rdr, ver)?);
        }
        has_normals = Some(rdr.read_bool()?);
        has_tangents = Some(MeshBlendShapeHasTangentsVariant::parse(rdr, ver)?);

        Ok(MeshBlendShape {
            name,
            first_vertex: first_vertex.unwrap(),
            vertex_count: vertex_count.unwrap(),
            has_normals: has_normals.unwrap(),
            aabb_min_delta,
            has_tangents: has_tangents.unwrap(),
            aabb_max_delta,
        })
    }
}

impl crate::ToValue for MeshBlendShape {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("name", &self.name),
            Some(("firstVertex".to_string(), self.first_vertex.to_value())),
            Some(("vertexCount".to_string(), self.vertex_count.to_value())),
            Some(("hasNormals".to_string(), self.has_normals.to_value())),
            crate::opt_field("aabbMinDelta", &self.aabb_min_delta),
            Some(("hasTangents".to_string(), self.has_tangents.to_value())),
            crate::opt_field("aabbMaxDelta", &self.aabb_max_delta),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "firstVertex" => (self.first_vertex).inspect_value(path, offset, limit),
                "vertexCount" => (self.vertex_count).inspect_value(path, offset, limit),
                "hasNormals" => (self.has_normals).inspect_value(path, offset, limit),
                "aabbMinDelta" => (self.aabb_min_delta).inspect_value(path, offset, limit),
                "hasTangents" => (self.has_tangents).inspect_value(path, offset, limit),
                "aabbMaxDelta" => (self.aabb_max_delta).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.first_vertex).inspect_value(path, offset, limit))
                .map(|v| ("firstVertex".to_string(), v)),
            ((self.vertex_count).inspect_value(path, offset, limit))
                .map(|v| ("vertexCount".to_string(), v)),
            ((self.has_normals).inspect_value(path, offset, limit))
                .map(|v| ("hasNormals".to_string(), v)),
            ((self.aabb_min_delta).inspect_value(path, offset, limit))
                .map(|v| ("aabbMinDelta".to_string(), v)),
            ((self.has_tangents).inspect_value(path, offset, limit))
                .map(|v| ("hasTangents".to_string(), v)),
            ((self.aabb_max_delta).inspect_value(path, offset, limit))
                .map(|v| ("aabbMaxDelta".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
