//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::update_zone_info::UpdateZoneInfo;

#[derive(Debug, Clone)]
pub struct CustomRenderTexture {
    pub m_object_hide_flags: u32,
    pub m_prefab_parent_object: PPtr,
    pub m_prefab_internal: PPtr,
    pub m_name: String,
    pub m_image_contents_hash: Hash128,
    pub m_width: i32,
    pub m_height: i32,
    pub m_anti_aliasing: i32,
    pub m_depth_format: i32,
    pub m_color_format: i32,
    pub m_mip_map: bool,
    pub m_generate_mips: bool,
    pub m_srgb: bool,
    pub m_texture_settings: GlTextureSettings,
    pub m_dimension: i32,
    pub m_volume_depth: i32,
    pub m_material: PPtr,
    pub m_init_material: PPtr,
    pub m_init_color: ColorRgba,
    pub m_init_texture: PPtr,
    pub m_update_mode: i32,
    pub m_initialization_mode: i32,
    pub m_update_zone_space: i32,
    pub m_current_update_zone_space: i32,
    pub m_update_zones: Vec<UpdateZoneInfo>,
    pub m_update_period: f32,
    pub m_shader_pass: u32,
    pub m_cubemap_face_mask: u32,
    pub m_double_buffered: bool,
    pub m_wrap_update_zones: bool,
}

impl crate::Parse for CustomRenderTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_prefab_parent_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_image_contents_hash = Hash128::parse(r, ver)?;
        let m_width = r.read_i32()?;
        let m_height = r.read_i32()?;
        let m_anti_aliasing = r.read_i32()?;
        let m_depth_format = r.read_i32()?;
        let m_color_format = r.read_i32()?;
        let m_mip_map = r.read_bool()?;
        let m_generate_mips = r.read_bool()?;
        let m_srgb = r.read_bool()?;
        r.align(4)?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_dimension = r.read_i32()?;
        let m_volume_depth = r.read_i32()?;
        let m_material = crate::PPtr::parse(r, ver)?;
        let m_init_material = crate::PPtr::parse(r, ver)?;
        let m_init_color = ColorRgba::parse(r, ver)?;
        let m_init_texture = crate::PPtr::parse(r, ver)?;
        let m_update_mode = r.read_i32()?;
        let m_initialization_mode = r.read_i32()?;
        let m_update_zone_space = r.read_i32()?;
        let m_current_update_zone_space = r.read_i32()?;
        let m_update_zones = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { UpdateZoneInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_update_period = r.read_f32()?;
        r.align(4)?;
        let m_shader_pass = r.read_u32()?;
        let m_cubemap_face_mask = r.read_u32()?;
        let m_double_buffered = r.read_bool()?;
        r.align(4)?;
        let m_wrap_update_zones = r.read_bool()?;
        r.align(4)?;

        Ok(CustomRenderTexture {
            m_object_hide_flags,
            m_prefab_parent_object,
            m_prefab_internal,
            m_name,
            m_image_contents_hash,
            m_width,
            m_height,
            m_anti_aliasing,
            m_depth_format,
            m_color_format,
            m_mip_map,
            m_generate_mips,
            m_srgb,
            m_texture_settings,
            m_dimension,
            m_volume_depth,
            m_material,
            m_init_material,
            m_init_color,
            m_init_texture,
            m_update_mode,
            m_initialization_mode,
            m_update_zone_space,
            m_current_update_zone_space,
            m_update_zones,
            m_update_period,
            m_shader_pass,
            m_cubemap_face_mask,
            m_double_buffered,
            m_wrap_update_zones,
        })
    }
}
