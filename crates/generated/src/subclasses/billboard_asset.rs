//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct BillboardAsset {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub width: f32,
    pub bottom: f32,
    pub height: f32,
    pub image_tex_coords: Vec<Vector4f>,
    pub rotated: Vec<u8>,
    pub vertices: Vec<Vector2f>,
    pub indices: Vec<u16>,
    pub material: PPtr,
}

impl crate::Parse for BillboardAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let width = r.read_f32()?;
        let bottom = r.read_f32()?;
        let height = r.read_f32()?;
        let image_tex_coords = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector4f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let rotated = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let vertices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector2f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u16() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let material = crate::PPtr::parse(r, ver)?;

        Ok(BillboardAsset {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
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
