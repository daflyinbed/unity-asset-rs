//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;


/// 版本枚举：BillboardAssetImageTexCoordsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardAssetImageTexCoordsVariant {
    Default(Vec<Vector4f>),
    V2017_1(Vec<Vector4f>),
}

impl crate::Parse for BillboardAssetImageTexCoordsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector4f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector4f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BillboardAssetVerticesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardAssetVerticesVariant {
    Default(Vec<Vector2f>),
    V2017_1(Vec<Vector2f>),
}

impl crate::Parse for BillboardAssetVerticesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：BillboardAssetIndicesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardAssetIndicesVariant {
    Default(Vec<u16>),
    V2017_1(Vec<u16>),
}

impl crate::Parse for BillboardAssetIndicesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct BillboardAsset {
    pub m_name: String,
    pub width: f32,
    pub bottom: f32,
    pub height: f32,
    pub image_tex_coords: BillboardAssetImageTexCoordsVariant,
    pub rotated: Option<Vec<u8>>,
    pub vertices: BillboardAssetVerticesVariant,
    pub indices: BillboardAssetIndicesVariant,
    pub material: PPtr,
}

impl crate::Parse for BillboardAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut width = None;
        let mut bottom = None;
        let mut height = None;
        let mut image_tex_coords = None;
        let mut rotated = None;
        let mut vertices = None;
        let mut indices = None;
        let mut material = None;
            m_name = Some(rdr.read_aligned_string()?);
            width = Some(rdr.read_f32()?);
            bottom = Some(rdr.read_f32()?);
            height = Some(rdr.read_f32()?);
            image_tex_coords = Some(BillboardAssetImageTexCoordsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
        } else {
            rotated = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        }
            vertices = Some(BillboardAssetVerticesVariant::parse(rdr, ver)?);
            indices = Some(BillboardAssetIndicesVariant::parse(rdr, ver)?);
            material = Some(crate::PPtr::parse(rdr, ver)?);

        Ok(BillboardAsset {
            m_name: m_name.unwrap(),
            width: width.unwrap(),
            bottom: bottom.unwrap(),
            height: height.unwrap(),
            image_tex_coords: image_tex_coords.unwrap(),
            rotated,
            vertices: vertices.unwrap(),
            indices: indices.unwrap(),
            material: material.unwrap(),
        })
    }
}
