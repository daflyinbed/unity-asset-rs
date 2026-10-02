//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;
use crate::subclasses::height_mesh_bv_node::HeightMeshBvNode;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：HeightMeshDataMVerticesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HeightMeshDataMVerticesVariant {
    Default(Vec<Vector3f>),
    V2017_1(Vec<Vector3f>),
}

impl crate::Parse for HeightMeshDataMVerticesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for HeightMeshDataMVerticesVariant {
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

/// 版本枚举：HeightMeshDataMIndicesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HeightMeshDataMIndicesVariant {
    Default(Vec<i32>),
    V2017_1(Vec<i32>),
}

impl crate::Parse for HeightMeshDataMIndicesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_i32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_i32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for HeightMeshDataMIndicesVariant {
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

/// 版本枚举：HeightMeshDataMNodesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum HeightMeshDataMNodesVariant {
    Default(Vec<HeightMeshBvNode>),
    V2017_1(Vec<HeightMeshBvNode>),
}

impl crate::Parse for HeightMeshDataMNodesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| HeightMeshBvNode::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| HeightMeshBvNode::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for HeightMeshDataMNodesVariant {
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
pub struct HeightMeshData {
    pub m_vertices: HeightMeshDataMVerticesVariant,
    pub m_indices: HeightMeshDataMIndicesVariant,
    pub m_bounds: Aabb,
    pub m_nodes: HeightMeshDataMNodesVariant,
}

impl crate::Parse for HeightMeshData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_vertices = HeightMeshDataMVerticesVariant::parse(rdr, ver)?;
        let m_indices = HeightMeshDataMIndicesVariant::parse(rdr, ver)?;
        let m_bounds = Aabb::parse(rdr, ver)?;
        let m_nodes = HeightMeshDataMNodesVariant::parse(rdr, ver)?;

        Ok(HeightMeshData {
            m_vertices,
            m_indices,
            m_bounds,
            m_nodes,
        })
    }
}

impl crate::ToValue for HeightMeshData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Vertices".to_string(), self.m_vertices.to_value())),
            Some(("m_Indices".to_string(), self.m_indices.to_value())),
            Some(("m_Bounds".to_string(), self.m_bounds.to_value())),
            Some(("m_Nodes".to_string(), self.m_nodes.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Vertices" => (self.m_vertices).inspect_value(path, offset, limit),
                "m_Indices" => (self.m_indices).inspect_value(path, offset, limit),
                "m_Bounds" => (self.m_bounds).inspect_value(path, offset, limit),
                "m_Nodes" => (self.m_nodes).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_vertices).inspect_value(path, offset, limit))
                .map(|v| ("m_Vertices".to_string(), v)),
            ((self.m_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_Indices".to_string(), v)),
            ((self.m_bounds).inspect_value(path, offset, limit))
                .map(|v| ("m_Bounds".to_string(), v)),
            ((self.m_nodes).inspect_value(path, offset, limit)).map(|v| ("m_Nodes".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
