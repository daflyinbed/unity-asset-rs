//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct BillboardAsset {
    pub parent: NamedObject,
    pub m_name: String,
    pub width: f32,
    pub bottom: f32,
    pub height: f32,
    pub image_tex_coords: Vec<Vector4f>,
    pub rotated: Option<Vec<u8>>,
    pub vertices: Vec<Vector2f>,
    pub indices: Vec<u16>,
    pub material: PPtr,
}

impl crate::Parse for BillboardAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let width = r.read_f32()?;
        let bottom = r.read_f32()?;
        let height = r.read_f32()?;
        let image_tex_coords = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector4f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let rotated = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let vertices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let material = crate::PPtr::parse(r, ver)?;

        Ok(BillboardAsset {
            parent,
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
