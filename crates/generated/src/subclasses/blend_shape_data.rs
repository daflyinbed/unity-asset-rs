//! 自动生成。请勿手动修改。

use crate::subclasses::blend_shape_vertex::BlendShapeVertex;
use crate::subclasses::mesh_blend_shape::MeshBlendShape;
use crate::subclasses::mesh_blend_shape_channel::MeshBlendShapeChannel;

#[derive(Debug, Clone)]
pub struct BlendShapeData {
    pub vertices: Vec<BlendShapeVertex>,
    pub shapes: Vec<MeshBlendShape>,
    pub channels: Vec<MeshBlendShapeChannel>,
    pub full_weights: Vec<f32>,
}

impl crate::Parse for BlendShapeData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let vertices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BlendShapeVertex::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let shapes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MeshBlendShape::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let channels = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { MeshBlendShapeChannel::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let full_weights = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(BlendShapeData {
            vertices,
            shapes,
            channels,
            full_weights,
        })
    }
}
