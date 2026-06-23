//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::aabb::Aabb;
use crate::subclasses::vector4f::Vector4f;

#[derive(Debug, Clone)]
pub struct SkinnedMeshRenderer {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: bool,
    pub m_receive_shadows: bool,
    pub m_lightmap_index: u8,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_materials: Vec<PPtr>,
    pub m_subset_indices: Vec<u32>,
    pub m_static_batch_root: PPtr,
    pub m_scale_in_lightmap: f32,
    pub m_quality: i32,
    pub m_update_when_offscreen: bool,
    pub m_mesh: PPtr,
    pub m_bones: Vec<PPtr>,
    pub m_aabb: Aabb,
    pub m_dirty_aabb: bool,
}

impl crate::Parse for SkinnedMeshRenderer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_bool()?;
        let m_cast_shadows = r.read_bool()?;
        let m_receive_shadows = r.read_bool()?;
        let m_lightmap_index = r.read_u8()?;
        let m_lightmap_tiling_offset = Vector4f::parse(r, ver)?;
        let m_materials = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_subset_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_static_batch_root = crate::PPtr::parse(r, ver)?;
        let m_scale_in_lightmap = r.read_f32()?;
        let m_quality = r.read_i32()?;
        let m_update_when_offscreen = r.read_bool()?;
        r.align(4)?;
        let m_mesh = crate::PPtr::parse(r, ver)?;
        let m_bones = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_aabb = Aabb::parse(r, ver)?;
        let m_dirty_aabb = r.read_bool()?;

        Ok(SkinnedMeshRenderer {
            m_object_hide_flags,
            m_extension_ptr,
            m_game_object,
            m_enabled,
            m_cast_shadows,
            m_receive_shadows,
            m_lightmap_index,
            m_lightmap_tiling_offset,
            m_materials,
            m_subset_indices,
            m_static_batch_root,
            m_scale_in_lightmap,
            m_quality,
            m_update_when_offscreen,
            m_mesh,
            m_bones,
            m_aabb,
            m_dirty_aabb,
        })
    }
}
