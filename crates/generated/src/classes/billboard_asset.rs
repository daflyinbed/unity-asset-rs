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
    V5_3(Vec<Vector2f>),
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
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::V5_3(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
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
    pub indices: Vec<u16>,
    pub material: PPtr,
}

impl crate::Parse for BillboardAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let width = rdr.read_f32()?;
        let bottom = rdr.read_f32()?;
        let height = rdr.read_f32()?;
        let image_tex_coords = BillboardAssetImageTexCoordsVariant::parse(rdr, ver)?;
        let rotated = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let vertices = BillboardAssetVerticesVariant::parse(rdr, ver)?;
        let indices = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let material = crate::PPtr::parse(rdr, ver)?;

        Ok(BillboardAsset {
            m_name,
            width,
            bottom,
            height,
            image_tex_coords,
            rotated,
            vertices,
            indices,
            material,
        })
    }
}
