//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;

#[derive(Debug, Clone)]
pub struct Texture3D {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_width: i32,
    pub m_height: i32,
    pub m_depth: i32,
    pub m_format: u32,
    pub m_mip_map: bool,
    pub m_data_size: u32,
    pub m_texture_settings: GlTextureSettings,
    pub image_data: Vec<u8>,
}

impl crate::Parse for Texture3D {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_width = r.read_i32()?;
        let m_height = r.read_i32()?;
        let m_depth = r.read_i32()?;
        let m_format = r.read_u32()?;
        let m_mip_map = r.read_bool()?;
        r.align(4)?;
        let m_data_size = r.read_u32()?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let image_data = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
        r.align(4)?;

        Ok(Texture3D {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_width,
            m_height,
            m_depth,
            m_format,
            m_mip_map,
            m_data_size,
            m_texture_settings,
            image_data,
        })
    }
}
