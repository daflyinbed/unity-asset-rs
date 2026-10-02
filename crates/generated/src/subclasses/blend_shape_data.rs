//! 自动生成。请勿手动修改。

use crate::subclasses::blend_shape_vertex::BlendShapeVertex;
use crate::subclasses::mesh_blend_shape::MeshBlendShape;
use crate::subclasses::mesh_blend_shape_channel::MeshBlendShapeChannel;

/// 版本枚举：BlendShapeDataVerticesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BlendShapeDataVerticesVariant {
    Default(Vec<BlendShapeVertex>),
    V2017_1(Vec<BlendShapeVertex>),
}

impl crate::Parse for BlendShapeDataVerticesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BlendShapeVertex::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| BlendShapeVertex::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BlendShapeDataVerticesVariant {
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

/// 版本枚举：BlendShapeDataShapesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BlendShapeDataShapesVariant {
    Default(Vec<MeshBlendShape>),
    V2017_1(Vec<MeshBlendShape>),
}

impl crate::Parse for BlendShapeDataShapesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| MeshBlendShape::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| MeshBlendShape::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BlendShapeDataShapesVariant {
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

/// 版本枚举：BlendShapeDataChannelsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BlendShapeDataChannelsVariant {
    Default(Vec<MeshBlendShapeChannel>),
    V2017_1(Vec<MeshBlendShapeChannel>),
}

impl crate::Parse for BlendShapeDataChannelsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| MeshBlendShapeChannel::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| MeshBlendShapeChannel::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BlendShapeDataChannelsVariant {
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

/// 版本枚举：BlendShapeDataFullWeightsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BlendShapeDataFullWeightsVariant {
    Default(Vec<f32>),
    V2017_1(Vec<f32>),
}

impl crate::Parse for BlendShapeDataFullWeightsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BlendShapeDataFullWeightsVariant {
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
pub struct BlendShapeData {
    pub vertices: BlendShapeDataVerticesVariant,
    pub shapes: BlendShapeDataShapesVariant,
    pub channels: BlendShapeDataChannelsVariant,
    pub full_weights: BlendShapeDataFullWeightsVariant,
}

impl crate::Parse for BlendShapeData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let vertices = BlendShapeDataVerticesVariant::parse(rdr, ver)?;
        let shapes = BlendShapeDataShapesVariant::parse(rdr, ver)?;
        let channels = BlendShapeDataChannelsVariant::parse(rdr, ver)?;
        let full_weights = BlendShapeDataFullWeightsVariant::parse(rdr, ver)?;

        Ok(BlendShapeData {
            vertices,
            shapes,
            channels,
            full_weights,
        })
    }
}

impl crate::ToValue for BlendShapeData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("vertices".to_string(), self.vertices.to_value())),
            Some(("shapes".to_string(), self.shapes.to_value())),
            Some(("channels".to_string(), self.channels.to_value())),
            Some(("fullWeights".to_string(), self.full_weights.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "vertices" => (self.vertices).inspect_value(path, offset, limit),
                "shapes" => (self.shapes).inspect_value(path, offset, limit),
                "channels" => (self.channels).inspect_value(path, offset, limit),
                "fullWeights" => (self.full_weights).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.vertices).inspect_value(path, offset, limit))
                .map(|v| ("vertices".to_string(), v)),
            ((self.shapes).inspect_value(path, offset, limit)).map(|v| ("shapes".to_string(), v)),
            ((self.channels).inspect_value(path, offset, limit))
                .map(|v| ("channels".to_string(), v)),
            ((self.full_weights).inspect_value(path, offset, limit))
                .map(|v| ("fullWeights".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
