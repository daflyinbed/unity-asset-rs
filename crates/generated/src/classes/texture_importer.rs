//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::build_target_settings::BuildTargetSettings;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::platform_settings::PlatformSettings;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::source_texture_information::SourceTextureInformation;
use crate::subclasses::sprite_sheet_meta_data::SpriteSheetMetaData;
use crate::subclasses::texture_import_output::TextureImportOutput;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;
use super::asset_importer::AssetImporter;

#[derive(Debug, Clone)]
pub struct TextureImporter {
    pub parent: AssetImporter,
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_mip_map_mode: i32,
    pub m_internal_id_to_name_table: Option<Vec<((i32, i64), String)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_enable_mip_map: i32,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_linear_texture: Option<i32>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub correct_gamma: Option<i32>,
    pub m_s_rgb_texture: Option<i32>,
    pub m_fade_out: i32,
    pub m_correct_gamma: Option<i32>,
    pub m_border_mip_map: i32,
    pub m_mip_map_fade_distance_start: i32,
    pub m_mip_map_fade_distance_end: i32,
    pub m_mip_maps_preserve_coverage: Option<i32>,
    pub m_convert_to_normal_map: i32,
    pub m_alpha_test_reference_value: Option<f32>,
    pub m_external_normal_map: i32,
    pub m_height_scale: f32,
    pub m_normal_map_filter: i32,
    pub m_is_readable: i32,
    pub m_gray_scale_to_alpha: i32,
    pub m_generate_cubemap: i32,
    pub m_texture_format: i32,
    pub m_seamless_cubemap: Option<i32>,
    pub m_max_texture_size: i32,
    pub m_cubemap_convolution: Option<i32>,
    pub m_texture_settings: GlTextureSettings,
    pub m_cubemap_convolution_steps: Option<i32>,
    pub m_streaming_mipmaps: Option<i32>,
    pub m_flip_green_channel: Option<i32>,
    pub m_npot_scale: i32,
    pub m_cubemap_convolution_exponent: Option<f32>,
    pub m_streaming_mipmaps_priority: Option<i32>,
    pub m_lightmap: i32,
    pub m_compression_quality: Option<i32>,
    pub m_vt_only: Option<i32>,
    pub m_texture_type: i32,
    pub m_ignore_master_texture_limit: Option<i32>,
    pub m_recommended_texture_format: Option<i32>,
    pub m_alpha_is_transparency: Option<i32>,
    pub m_sprite_mode: Option<i32>,
    pub m_ignore_mipmap_limit: Option<i32>,
    pub m_source_texture_information: Option<SourceTextureInformation>,
    pub m_build_target_settings: Option<Vec<BuildTargetSettings>>,
    pub m_sprite_extrude: Option<u32>,
    pub m_output: Option<TextureImportOutput>,
    pub m_sprite_mesh_type: Option<i32>,
    pub m_user_data: Option<String>,
    pub m_alignment: Option<i32>,
    pub m_rgbm: Option<i32>,
    pub m_sprite_pivot: Option<Vector2f>,
    pub m_sprite_pixels_to_units: Option<f32>,
    pub m_sprite_border: Option<Vector4f>,
    pub m_allows_alpha_splitting: Option<i32>,
    pub m_sprite_sheet: Option<SpriteSheetMetaData>,
    pub m_alpha_usage: Option<i32>,
    pub m_sprite_packing_tag: Option<String>,
    pub m_sprite_tessellation_detail: Option<f32>,
    pub m_sprite_generate_fallback_physics_shape: Option<i32>,
    pub m_texture_shape: Option<i32>,
    pub m_max_texture_size_set: Option<i32>,
    pub m_compression_quality_set: Option<i32>,
    pub m_texture_format_set: Option<i32>,
    pub m_platform_settings: Option<Vec<PlatformSettings>>,
    pub m_single_channel_component: Option<i32>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
    pub m_flipbook_rows: Option<i32>,
    pub m_push_pull_dilation: Option<i32>,
    pub m_flipbook_columns: Option<i32>,
    pub m_apply_gamma_decoding: Option<i32>,
    pub m_ignore_png_gamma: Option<bool>,
    pub m_psd_remove_matte: Option<bool>,
    pub m_psd_show_remove_matte_option: Option<bool>,
    pub m_cookie_light_type: Option<i32>,
    pub m_swizzle: Option<u32>,
    pub m_mipmap_limit_group_name: Option<String>,
}

impl crate::Parse for TextureImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = AssetImporter::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_file_id_to_recycle_name = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mip_map_mode = r.read_i32()?;
        let m_internal_id_to_name_table = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = r.read_i32()?; let b = r.read_i64()?; Ok((a, b)) } )?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_old_hash_identity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = MdFour::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enable_mip_map = r.read_i32()?;
        let m_external_objects = if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_new_hash_identity = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = MdFour::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_linear_texture = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_used_file_i_ds = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let correct_gamma = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_s_rgb_texture = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_fade_out = r.read_i32()?;
        let m_correct_gamma = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_border_mip_map = r.read_i32()?;
        let m_mip_map_fade_distance_start = r.read_i32()?;
        let m_mip_map_fade_distance_end = r.read_i32()?;
        let m_mip_maps_preserve_coverage = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_convert_to_normal_map = r.read_i32()?;
        let m_alpha_test_reference_value = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_external_normal_map = r.read_i32()?;
        let m_height_scale = r.read_f32()?;
        let m_normal_map_filter = r.read_i32()?;
        let m_is_readable = r.read_i32()?;
        r.align(4)?;
        let m_gray_scale_to_alpha = r.read_i32()?;
        r.align(4)?;
        let m_generate_cubemap = r.read_i32()?;
        let m_texture_format = r.read_i32()?;
        r.align(4)?;
        let m_seamless_cubemap = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_max_texture_size = r.read_i32()?;
        let m_cubemap_convolution = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_cubemap_convolution_steps = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_streaming_mipmaps = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_flip_green_channel = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_npot_scale = r.read_i32()?;
        let m_cubemap_convolution_exponent = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_streaming_mipmaps_priority = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_lightmap = r.read_i32()?;
        let m_compression_quality = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_vt_only = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_texture_type = r.read_i32()?;
        let m_ignore_master_texture_limit = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_recommended_texture_format = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_alpha_is_transparency = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_sprite_mode = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_ignore_mipmap_limit = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_source_texture_information = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = SourceTextureInformation::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_build_target_settings = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { BuildTargetSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sprite_extrude = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_output = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = TextureImportOutput::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sprite_mesh_type = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_user_data = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_alignment = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_rgbm = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_sprite_pivot = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(Vector2f::parse(r, ver)?)
        } else {
            None
        };
        let m_sprite_pixels_to_units = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_sprite_border = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(Vector4f::parse(r, ver)?)
        } else {
            None
        };
        let m_allows_alpha_splitting = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sprite_sheet = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = SpriteSheetMetaData::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_alpha_usage = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_sprite_packing_tag = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sprite_tessellation_detail = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_sprite_generate_fallback_physics_shape = if *ver >= crate::UnityVersion::new(2017, 3, 1) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_texture_shape = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_max_texture_size_set = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_compression_quality_set = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_texture_format_set = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_platform_settings = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { PlatformSettings::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_single_channel_component = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_asset_bundle_name = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_bundle_variant = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_flipbook_rows = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_push_pull_dilation = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_flipbook_columns = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_apply_gamma_decoding = if *ver >= crate::UnityVersion::new(2019, 3, 6) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_ignore_png_gamma = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_psd_remove_matte = if *ver >= crate::UnityVersion::new(2018, 2, 1) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_psd_show_remove_matte_option = if *ver >= crate::UnityVersion::new(2018, 2, 1) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cookie_light_type = if *ver >= crate::UnityVersion::new(2020, 3, 38) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_swizzle = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = r.read_u32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_mipmap_limit_group_name = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(TextureImporter {
            parent,
            m_name,
            m_file_id_to_recycle_name,
            m_mip_map_mode,
            m_internal_id_to_name_table,
            m_old_hash_identity,
            m_enable_mip_map,
            m_external_objects,
            m_new_hash_identity,
            m_linear_texture,
            m_used_file_i_ds,
            correct_gamma,
            m_s_rgb_texture,
            m_fade_out,
            m_correct_gamma,
            m_border_mip_map,
            m_mip_map_fade_distance_start,
            m_mip_map_fade_distance_end,
            m_mip_maps_preserve_coverage,
            m_convert_to_normal_map,
            m_alpha_test_reference_value,
            m_external_normal_map,
            m_height_scale,
            m_normal_map_filter,
            m_is_readable,
            m_gray_scale_to_alpha,
            m_generate_cubemap,
            m_texture_format,
            m_seamless_cubemap,
            m_max_texture_size,
            m_cubemap_convolution,
            m_texture_settings,
            m_cubemap_convolution_steps,
            m_streaming_mipmaps,
            m_flip_green_channel,
            m_npot_scale,
            m_cubemap_convolution_exponent,
            m_streaming_mipmaps_priority,
            m_lightmap,
            m_compression_quality,
            m_vt_only,
            m_texture_type,
            m_ignore_master_texture_limit,
            m_recommended_texture_format,
            m_alpha_is_transparency,
            m_sprite_mode,
            m_ignore_mipmap_limit,
            m_source_texture_information,
            m_build_target_settings,
            m_sprite_extrude,
            m_output,
            m_sprite_mesh_type,
            m_user_data,
            m_alignment,
            m_rgbm,
            m_sprite_pivot,
            m_sprite_pixels_to_units,
            m_sprite_border,
            m_allows_alpha_splitting,
            m_sprite_sheet,
            m_alpha_usage,
            m_sprite_packing_tag,
            m_sprite_tessellation_detail,
            m_sprite_generate_fallback_physics_shape,
            m_texture_shape,
            m_max_texture_size_set,
            m_compression_quality_set,
            m_texture_format_set,
            m_platform_settings,
            m_single_channel_component,
            m_asset_bundle_name,
            m_asset_bundle_variant,
            m_flipbook_rows,
            m_push_pull_dilation,
            m_flipbook_columns,
            m_apply_gamma_decoding,
            m_ignore_png_gamma,
            m_psd_remove_matte,
            m_psd_show_remove_matte_option,
            m_cookie_light_type,
            m_swizzle,
            m_mipmap_limit_group_name,
        })
    }
}
