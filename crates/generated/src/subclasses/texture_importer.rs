//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::build_target_settings::BuildTargetSettings;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_texture_information::SourceTextureInformation;

#[derive(Debug, Clone)]
pub struct TextureImporter {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_file_id_to_recycle_name: Vec<(i32, String)>,
    pub m_old_hash_identity: MdFour,
    pub m_new_hash_identity: MdFour,
    pub m_mip_map_mode: i32,
    pub m_enable_mip_map: i32,
    pub m_correct_gamma: i32,
    pub m_fade_out: i32,
    pub m_border_mip_map: i32,
    pub m_mip_map_fade_distance_start: i32,
    pub m_mip_map_fade_distance_end: i32,
    pub m_convert_to_normal_map: i32,
    pub m_external_normal_map: i32,
    pub m_is_readable: i32,
    pub m_height_scale: f32,
    pub m_normal_map_filter: i32,
    pub m_gray_scale_to_alpha: i32,
    pub m_generate_cubemap: i32,
    pub m_texture_format: i32,
    pub m_max_texture_size: i32,
    pub m_texture_settings: GlTextureSettings,
    pub m_npot_scale: i32,
    pub m_lightmap: i32,
    pub m_texture_type: i32,
    pub m_recommended_texture_format: i32,
    pub m_source_texture_information: SourceTextureInformation,
    pub m_build_target_settings: Vec<BuildTargetSettings>,
}

impl crate::Parse for TextureImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_file_id_to_recycle_name = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_old_hash_identity = MdFour::parse(r, ver)?;
        r.align(4)?;
        let m_new_hash_identity = MdFour::parse(r, ver)?;
        r.align(4)?;
        let m_mip_map_mode = r.read_i32()?;
        let m_enable_mip_map = r.read_i32()?;
        let m_correct_gamma = r.read_i32()?;
        let m_fade_out = r.read_i32()?;
        let m_border_mip_map = r.read_i32()?;
        let m_mip_map_fade_distance_start = r.read_i32()?;
        let m_mip_map_fade_distance_end = r.read_i32()?;
        let m_convert_to_normal_map = r.read_i32()?;
        let m_external_normal_map = r.read_i32()?;
        let m_is_readable = r.read_i32()?;
        r.align(4)?;
        let m_height_scale = r.read_f32()?;
        let m_normal_map_filter = r.read_i32()?;
        let m_gray_scale_to_alpha = r.read_i32()?;
        r.align(4)?;
        let m_generate_cubemap = r.read_i32()?;
        let m_texture_format = r.read_i32()?;
        r.align(4)?;
        let m_max_texture_size = r.read_i32()?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_npot_scale = r.read_i32()?;
        let m_lightmap = r.read_i32()?;
        let m_texture_type = r.read_i32()?;
        let m_recommended_texture_format = r.read_i32()?;
        let m_source_texture_information = SourceTextureInformation::parse(r, ver)?;
        r.align(4)?;
        let m_build_target_settings = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildTargetSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(TextureImporter {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_file_id_to_recycle_name,
            m_old_hash_identity,
            m_new_hash_identity,
            m_mip_map_mode,
            m_enable_mip_map,
            m_correct_gamma,
            m_fade_out,
            m_border_mip_map,
            m_mip_map_fade_distance_start,
            m_mip_map_fade_distance_end,
            m_convert_to_normal_map,
            m_external_normal_map,
            m_is_readable,
            m_height_scale,
            m_normal_map_filter,
            m_gray_scale_to_alpha,
            m_generate_cubemap,
            m_texture_format,
            m_max_texture_size,
            m_texture_settings,
            m_npot_scale,
            m_lightmap,
            m_texture_type,
            m_recommended_texture_format,
            m_source_texture_information,
            m_build_target_settings,
        })
    }
}
