//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::blend_shape_data::BlendShapeData;
use crate::subclasses::bone_weights4::BoneWeights4;
use crate::subclasses::matrix4x4f::Matrix4x4f;
use crate::subclasses::rectf::Rectf;
use crate::subclasses::secondary_sprite_texture::SecondarySpriteTexture;
use crate::subclasses::sprite_vertex::SpriteVertex;
use crate::subclasses::sub_mesh::SubMesh;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;
use crate::subclasses::vertex_data::VertexData;

/// 版本枚举：SpriteRenderDataMSubMeshesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRenderDataMSubMeshesVariant {
    Default(Vec<SubMesh>),
    V2017_1(Vec<SubMesh>),
}

impl crate::Parse for SpriteRenderDataMSubMeshesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubMesh::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubMesh::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SpriteRenderDataMVertexDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteRenderDataMVertexDataVariant {
    Default(Box<VertexData>),
    V2018_1(Box<VertexData>),
}

impl crate::Parse for SpriteRenderDataMVertexDataVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = ({ Ok(Box::new(VertexData::parse(rdr, ver)?)) })?;
            Ok(Self::V2018_1(v))
        } else {
            let v = ({ Ok(Box::new(VertexData::parse(rdr, ver)?)) })?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SpriteRenderData {
    pub texture: PPtr,
    pub vertices: Option<Vec<SpriteVertex>>,
    pub alpha_texture: Option<PPtr>,
    pub indices: Option<Vec<u16>>,
    pub m_sub_meshes: Option<SpriteRenderDataMSubMeshesVariant>,
    pub secondary_textures: Option<Vec<SecondarySpriteTexture>>,
    pub texture_rect: Rectf,
    pub m_index_buffer: Option<Vec<u8>>,
    pub texture_rect_offset: Vector2f,
    pub m_vertex_data: Option<SpriteRenderDataMVertexDataVariant>,
    pub settings_raw: u32,
    pub m_bindpose: Option<Vec<Matrix4x4f>>,
    pub uv_transform: Option<Vector4f>,
    pub atlas_rect_offset: Option<Vector2f>,
    pub m_source_skin: Option<Vec<BoneWeights4>>,
    pub m_blend_shapes: Option<BlendShapeData>,
    pub downscale_multiplier: Option<f32>,
}

impl crate::Parse for SpriteRenderData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut texture = None;
        let mut vertices = None;
        let mut alpha_texture = None;
        let mut indices = None;
        let mut m_sub_meshes = None;
        let mut secondary_textures = None;
        let mut texture_rect = None;
        let mut m_index_buffer = None;
        let mut texture_rect_offset = None;
        let mut m_vertex_data = None;
        let mut settings_raw = None;
        let mut m_bindpose = None;
        let mut uv_transform = None;
        let mut atlas_rect_offset = None;
        let mut m_source_skin = None;
        let mut m_blend_shapes = None;
        let mut downscale_multiplier = None;
        texture = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            alpha_texture = Some(crate::PPtr::parse(rdr, ver)?);
            secondary_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SecondarySpriteTexture::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_sub_meshes = Some(SpriteRenderDataMSubMeshesVariant::parse(rdr, ver)?);
            m_index_buffer = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_vertex_data = Some(SpriteRenderDataMVertexDataVariant::parse(rdr, ver)?);
            m_bindpose = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Matrix4x4f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_blend_shapes = Some(BlendShapeData::parse(rdr, ver)?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            alpha_texture = Some(crate::PPtr::parse(rdr, ver)?);
            secondary_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SecondarySpriteTexture::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_sub_meshes = Some(SpriteRenderDataMSubMeshesVariant::parse(rdr, ver)?);
            m_index_buffer = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_vertex_data = Some(SpriteRenderDataMVertexDataVariant::parse(rdr, ver)?);
            m_bindpose = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Matrix4x4f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            alpha_texture = Some(crate::PPtr::parse(rdr, ver)?);
            m_sub_meshes = Some(SpriteRenderDataMSubMeshesVariant::parse(rdr, ver)?);
            m_index_buffer = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_vertex_data = Some(SpriteRenderDataMVertexDataVariant::parse(rdr, ver)?);
            m_bindpose = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Matrix4x4f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            alpha_texture = Some(crate::PPtr::parse(rdr, ver)?);
            m_sub_meshes = Some(SpriteRenderDataMSubMeshesVariant::parse(rdr, ver)?);
            m_index_buffer = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_vertex_data = Some(SpriteRenderDataMVertexDataVariant::parse(rdr, ver)?);
            m_bindpose = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| Matrix4x4f::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_source_skin = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| BoneWeights4::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            alpha_texture = Some(crate::PPtr::parse(rdr, ver)?);
            m_sub_meshes = Some(SpriteRenderDataMSubMeshesVariant::parse(rdr, ver)?);
            m_index_buffer = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_vertex_data = Some(SpriteRenderDataMVertexDataVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            alpha_texture = Some(crate::PPtr::parse(rdr, ver)?);
            vertices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SpriteVertex::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            indices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
            vertices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| SpriteVertex::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            indices = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u16())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        }
        texture_rect = Some(Rectf::parse(rdr, ver)?);
        texture_rect_offset = Some(Vector2f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 3) {
            atlas_rect_offset = Some(Vector2f::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 4, 6) {
            atlas_rect_offset = Some(Vector2f::parse(rdr, ver)?);
        } else {
        }
        settings_raw = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            uv_transform = Some(Vector4f::parse(rdr, ver)?);
            downscale_multiplier = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            uv_transform = Some(Vector4f::parse(rdr, ver)?);
        } else {
        }

        Ok(SpriteRenderData {
            texture: texture.unwrap(),
            vertices,
            alpha_texture,
            indices,
            m_sub_meshes,
            secondary_textures,
            texture_rect: texture_rect.unwrap(),
            m_index_buffer,
            texture_rect_offset: texture_rect_offset.unwrap(),
            m_vertex_data,
            settings_raw: settings_raw.unwrap(),
            m_bindpose,
            uv_transform,
            atlas_rect_offset,
            m_source_skin,
            m_blend_shapes,
            downscale_multiplier,
        })
    }
}
