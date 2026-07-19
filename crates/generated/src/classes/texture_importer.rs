//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::source_texture_information::SourceTextureInformation;
use crate::subclasses::sprite_sheet_meta_data::SpriteSheetMetaData;
use crate::subclasses::texture_import_output::TextureImportOutput;
use crate::subclasses::texture_importer_platform_settings::TextureImporterPlatformSettings;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

/// 版本枚举：TextureImporterMFileIdToRecycleNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMFileIdToRecycleNameVariant {
    Default(Vec<(i32, String)>),
    V5_0(Vec<(i64, String)>),
}

impl crate::Parse for TextureImporterMFileIdToRecycleNameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_i64()?;
                            let b = rdr.read_aligned_string()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_0(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_i32()?;
                            let b = rdr.read_aligned_string()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for TextureImporterMExternalObjectsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = SourceAssetIdentifier::parse(rdr, ver)?;
                            let b = crate::PPtr::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = SourceAssetIdentifier::parse(rdr, ver)?;
                            let b = crate::PPtr::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImporterMIsReadableVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMIsReadableVariant {
    Default(i32),
    V3_5(i32),
}

impl crate::Parse for TextureImporterMIsReadableVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V3_5(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImporterMGrayScaleToAlphaVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMGrayScaleToAlphaVariant {
    Default(i32),
    V4_0(i32),
}

impl crate::Parse for TextureImporterMGrayScaleToAlphaVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImporterMFormatVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMFormatVariant {
    Default(i32),
    V4_0(i32),
}

impl crate::Parse for TextureImporterMFormatVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImporterMTextureTypeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMTextureTypeVariant {
    Default(i32),
    V4_0(i32),
    V5_5(i32),
}

impl crate::Parse for TextureImporterMTextureTypeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_5(v))
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImporterMPlatformSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMPlatformSettingsVariant {
    Default(Vec<TextureImporterPlatformSettings>),
    V2017_1(Vec<TextureImporterPlatformSettings>),
}

impl crate::Parse for TextureImporterMPlatformSettingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TextureImporterPlatformSettings::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TextureImporterPlatformSettings::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImporterMOutputVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMOutputVariant {
    Default(Box<TextureImportOutput>),
    V2018_2_1(Box<TextureImportOutput>),
}

impl crate::Parse for TextureImporterMOutputVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 2, 1) {
            let v = ({ Ok(Box::new(TextureImportOutput::parse(rdr, ver)?)) })?;
            Ok(Self::V2018_2_1(v))
        } else {
            let v = ({ Ok(Box::new(TextureImportOutput::parse(rdr, ver)?)) })?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImporterMTextureFormatSetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMTextureFormatSetVariant {
    Default(i32),
    V2019_3_6(i32),
}

impl crate::Parse for TextureImporterMTextureFormatSetVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 6) {
            let v = rdr.read_i32()?;
            Ok(Self::V2019_3_6(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImporterMApplyGammaDecodingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMApplyGammaDecodingVariant {
    Default(i32),
    V2020_3_38(i32),
    V2021_1(i32),
    V2021_3_9(i32),
}

impl crate::Parse for TextureImporterMApplyGammaDecodingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 3, 9) {
            let v = rdr.read_i32()?;
            Ok(Self::V2021_3_9(v))
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V2021_1(v))
        } else if *ver >= crate::UnityVersion::new(2020, 3, 38) {
            let v = rdr.read_i32()?;
            Ok(Self::V2020_3_38(v))
        } else {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImporterMPsdRemoveMatteVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMPsdRemoveMatteVariant {
    Default(bool),
    V2022_1(bool),
}

impl crate::Parse for TextureImporterMPsdRemoveMatteVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2022_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TextureImporterMSwizzleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TextureImporterMSwizzleVariant {
    Default(u32),
    V2022_1_13(u32),
}

impl crate::Parse for TextureImporterMSwizzleVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 13) {
            let v = rdr.read_u32()?;
            Ok(Self::V2022_1_13(v))
        } else {
            let v = rdr.read_u32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct TextureImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<TextureImporterMFileIdToRecycleNameVariant>,
    pub m_mip_map_mode: i32,
    pub m_internal_id_to_name_table: Option<Vec<((i32, i64), String)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_enable_mip_map: i32,
    pub m_external_objects: Option<TextureImporterMExternalObjectsVariant>,
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
    pub m_is_readable: TextureImporterMIsReadableVariant,
    pub m_gray_scale_to_alpha: TextureImporterMGrayScaleToAlphaVariant,
    pub m_generate_cubemap: i32,
    pub m_format: TextureImporterMFormatVariant,
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
    pub m_texture_type: TextureImporterMTextureTypeVariant,
    pub m_ignore_master_texture_limit: Option<i32>,
    pub m_recommended_texture_format: Option<i32>,
    pub m_alpha_is_transparency: Option<i32>,
    pub m_sprite_mode: Option<i32>,
    pub m_ignore_mipmap_limit: Option<i32>,
    pub m_source_texture_information: Option<SourceTextureInformation>,
    pub m_platform_settings: TextureImporterMPlatformSettingsVariant,
    pub m_sprite_extrude: Option<u32>,
    pub m_output: Option<TextureImporterMOutputVariant>,
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
    pub m_texture_format_set: Option<TextureImporterMTextureFormatSetVariant>,
    pub m_single_channel_component: Option<i32>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
    pub m_flipbook_rows: Option<i32>,
    pub m_flipbook_columns: Option<i32>,
    pub m_apply_gamma_decoding: Option<TextureImporterMApplyGammaDecodingVariant>,
    pub m_ignore_png_gamma: Option<i32>,
    pub m_psd_remove_matte: Option<TextureImporterMPsdRemoveMatteVariant>,
    pub m_psd_show_remove_matte_option: Option<bool>,
    pub m_cookie_light_type: Option<i32>,
    pub m_swizzle: Option<TextureImporterMSwizzleVariant>,
    pub m_mipmap_limit_group_name: Option<String>,
}

impl crate::Parse for TextureImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_mip_map_mode = None;
        let mut m_internal_id_to_name_table = None;
        let mut m_old_hash_identity = None;
        let mut m_enable_mip_map = None;
        let mut m_external_objects = None;
        let mut m_new_hash_identity = None;
        let mut m_linear_texture = None;
        let mut m_used_file_i_ds = None;
        let mut correct_gamma = None;
        let mut m_s_rgb_texture = None;
        let mut m_fade_out = None;
        let mut m_correct_gamma = None;
        let mut m_border_mip_map = None;
        let mut m_mip_map_fade_distance_start = None;
        let mut m_mip_map_fade_distance_end = None;
        let mut m_mip_maps_preserve_coverage = None;
        let mut m_convert_to_normal_map = None;
        let mut m_alpha_test_reference_value = None;
        let mut m_external_normal_map = None;
        let mut m_height_scale = None;
        let mut m_normal_map_filter = None;
        let mut m_is_readable = None;
        let mut m_gray_scale_to_alpha = None;
        let mut m_generate_cubemap = None;
        let mut m_format = None;
        let mut m_seamless_cubemap = None;
        let mut m_max_texture_size = None;
        let mut m_cubemap_convolution = None;
        let mut m_texture_settings = None;
        let mut m_cubemap_convolution_steps = None;
        let mut m_streaming_mipmaps = None;
        let mut m_flip_green_channel = None;
        let mut m_npot_scale = None;
        let mut m_cubemap_convolution_exponent = None;
        let mut m_streaming_mipmaps_priority = None;
        let mut m_lightmap = None;
        let mut m_compression_quality = None;
        let mut m_vt_only = None;
        let mut m_texture_type = None;
        let mut m_ignore_master_texture_limit = None;
        let mut m_recommended_texture_format = None;
        let mut m_alpha_is_transparency = None;
        let mut m_sprite_mode = None;
        let mut m_ignore_mipmap_limit = None;
        let mut m_source_texture_information = None;
        let mut m_platform_settings = None;
        let mut m_sprite_extrude = None;
        let mut m_output = None;
        let mut m_sprite_mesh_type = None;
        let mut m_user_data = None;
        let mut m_alignment = None;
        let mut m_rgbm = None;
        let mut m_sprite_pivot = None;
        let mut m_sprite_pixels_to_units = None;
        let mut m_sprite_border = None;
        let mut m_allows_alpha_splitting = None;
        let mut m_sprite_sheet = None;
        let mut m_alpha_usage = None;
        let mut m_sprite_packing_tag = None;
        let mut m_sprite_tessellation_detail = None;
        let mut m_sprite_generate_fallback_physics_shape = None;
        let mut m_texture_shape = None;
        let mut m_max_texture_size_set = None;
        let mut m_compression_quality_set = None;
        let mut m_texture_format_set = None;
        let mut m_single_channel_component = None;
        let mut m_asset_bundle_name = None;
        let mut m_asset_bundle_variant = None;
        let mut m_flipbook_rows = None;
        let mut m_flipbook_columns = None;
        let mut m_apply_gamma_decoding = None;
        let mut m_ignore_png_gamma = None;
        let mut m_psd_remove_matte = None;
        let mut m_psd_show_remove_matte_option = None;
        let mut m_cookie_light_type = None;
        let mut m_swizzle = None;
        let mut m_mipmap_limit_group_name = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_internal_id_to_name_table = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = ({
                                    let a = rdr.read_i32()?;
                                    let b = rdr.read_i64()?;
                                    Ok((a, b))
                                })?;
                                let b = rdr.read_aligned_string()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_external_objects = Some(TextureImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_file_id_to_recycle_name =
                Some(TextureImporterMFileIdToRecycleNameVariant::parse(rdr, ver)?);
            m_external_objects = Some(TextureImporterMExternalObjectsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 2) {
            m_file_id_to_recycle_name =
                Some(TextureImporterMFileIdToRecycleNameVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_file_id_to_recycle_name =
                Some(TextureImporterMFileIdToRecycleNameVariant::parse(rdr, ver)?);
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
        }
        m_mip_map_mode = Some(rdr.read_i32()?);
        m_enable_mip_map = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_s_rgb_texture = Some(rdr.read_i32()?);
            m_linear_texture = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_linear_texture = Some(rdr.read_i32()?);
            correct_gamma = Some(rdr.read_i32()?);
        } else {
            m_correct_gamma = Some(rdr.read_i32()?);
        }
        m_fade_out = Some(rdr.read_i32()?);
        m_border_mip_map = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_mip_maps_preserve_coverage = Some(rdr.read_i32()?);
            m_alpha_test_reference_value = Some(rdr.read_f32()?);
        } else {
        }
        m_mip_map_fade_distance_start = Some(rdr.read_i32()?);
        m_mip_map_fade_distance_end = Some(rdr.read_i32()?);
        m_convert_to_normal_map = Some(rdr.read_i32()?);
        m_external_normal_map = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_is_readable = Some(TextureImporterMIsReadableVariant::parse(rdr, ver)?);
        }
        m_height_scale = Some(rdr.read_f32()?);
        m_normal_map_filter = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_flip_green_channel = Some(rdr.read_i32()?);
            m_is_readable = Some(TextureImporterMIsReadableVariant::parse(rdr, ver)?);
            m_streaming_mipmaps = Some(rdr.read_i32()?);
            m_streaming_mipmaps_priority = Some(rdr.read_i32()?);
            m_vt_only = Some(rdr.read_i32()?);
            m_ignore_mipmap_limit = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_flip_green_channel = Some(rdr.read_i32()?);
            m_is_readable = Some(TextureImporterMIsReadableVariant::parse(rdr, ver)?);
            m_streaming_mipmaps = Some(rdr.read_i32()?);
            m_streaming_mipmaps_priority = Some(rdr.read_i32()?);
            m_vt_only = Some(rdr.read_i32()?);
            m_ignore_master_texture_limit = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_is_readable = Some(TextureImporterMIsReadableVariant::parse(rdr, ver)?);
            m_streaming_mipmaps = Some(rdr.read_i32()?);
            m_streaming_mipmaps_priority = Some(rdr.read_i32()?);
            m_vt_only = Some(rdr.read_i32()?);
            m_ignore_master_texture_limit = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_is_readable = Some(TextureImporterMIsReadableVariant::parse(rdr, ver)?);
            m_streaming_mipmaps = Some(rdr.read_i32()?);
            m_streaming_mipmaps_priority = Some(rdr.read_i32()?);
            m_vt_only = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_is_readable = Some(TextureImporterMIsReadableVariant::parse(rdr, ver)?);
            m_streaming_mipmaps = Some(rdr.read_i32()?);
            m_streaming_mipmaps_priority = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_is_readable = Some(TextureImporterMIsReadableVariant::parse(rdr, ver)?);
        } else {
        }
        m_gray_scale_to_alpha = Some(TextureImporterMGrayScaleToAlphaVariant::parse(rdr, ver)?);
        m_generate_cubemap = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_cubemap_convolution = Some(rdr.read_i32()?);
            m_seamless_cubemap = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_cubemap_convolution = Some(rdr.read_i32()?);
            m_cubemap_convolution_steps = Some(rdr.read_i32()?);
            m_cubemap_convolution_exponent = Some(rdr.read_f32()?);
            m_seamless_cubemap = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_seamless_cubemap = Some(rdr.read_i32()?);
        } else {
        }
        m_format = Some(TextureImporterMFormatVariant::parse(rdr, ver)?);
        m_max_texture_size = Some(rdr.read_i32()?);
        m_texture_settings = Some(GlTextureSettings::parse(rdr, ver)?);
        m_npot_scale = Some(rdr.read_i32()?);
        m_lightmap = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2017, 3, 1) {
            m_compression_quality = Some(rdr.read_i32()?);
            m_sprite_mode = Some(rdr.read_i32()?);
            m_sprite_extrude = Some(rdr.read_u32()?);
            m_sprite_mesh_type = Some(rdr.read_i32()?);
            m_alignment = Some(rdr.read_i32()?);
            m_sprite_pivot = Some(Vector2f::parse(rdr, ver)?);
            m_sprite_pixels_to_units = Some(rdr.read_f32()?);
            m_sprite_border = Some(Vector4f::parse(rdr, ver)?);
            m_sprite_generate_fallback_physics_shape = Some(rdr.read_i32()?);
            m_alpha_usage = Some(rdr.read_i32()?);
            m_alpha_is_transparency = Some(rdr.read_i32()?);
            m_sprite_tessellation_detail = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_compression_quality = Some(rdr.read_i32()?);
            m_sprite_mode = Some(rdr.read_i32()?);
            m_sprite_extrude = Some(rdr.read_u32()?);
            m_sprite_mesh_type = Some(rdr.read_i32()?);
            m_alignment = Some(rdr.read_i32()?);
            m_sprite_pivot = Some(Vector2f::parse(rdr, ver)?);
            m_sprite_border = Some(Vector4f::parse(rdr, ver)?);
            m_sprite_pixels_to_units = Some(rdr.read_f32()?);
            m_alpha_usage = Some(rdr.read_i32()?);
            m_alpha_is_transparency = Some(rdr.read_i32()?);
            m_sprite_tessellation_detail = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 4) {
            m_rgbm = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_sprite_mode = Some(rdr.read_i32()?);
            m_sprite_extrude = Some(rdr.read_u32()?);
            m_sprite_mesh_type = Some(rdr.read_i32()?);
            m_alignment = Some(rdr.read_i32()?);
            m_sprite_pivot = Some(Vector2f::parse(rdr, ver)?);
            m_sprite_border = Some(Vector4f::parse(rdr, ver)?);
            m_sprite_pixels_to_units = Some(rdr.read_f32()?);
            m_alpha_is_transparency = Some(rdr.read_i32()?);
            m_sprite_tessellation_detail = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_rgbm = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_allows_alpha_splitting = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_sprite_mode = Some(rdr.read_i32()?);
            m_sprite_extrude = Some(rdr.read_u32()?);
            m_sprite_mesh_type = Some(rdr.read_i32()?);
            m_alignment = Some(rdr.read_i32()?);
            m_sprite_pivot = Some(Vector2f::parse(rdr, ver)?);
            m_sprite_border = Some(Vector4f::parse(rdr, ver)?);
            m_sprite_pixels_to_units = Some(rdr.read_f32()?);
            m_alpha_is_transparency = Some(rdr.read_i32()?);
            m_sprite_tessellation_detail = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 8) {
            m_rgbm = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_sprite_mode = Some(rdr.read_i32()?);
            m_sprite_extrude = Some(rdr.read_u32()?);
            m_sprite_mesh_type = Some(rdr.read_i32()?);
            m_alignment = Some(rdr.read_i32()?);
            m_sprite_pivot = Some(Vector2f::parse(rdr, ver)?);
            m_sprite_border = Some(Vector4f::parse(rdr, ver)?);
            m_sprite_pixels_to_units = Some(rdr.read_f32()?);
            m_alpha_is_transparency = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_rgbm = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_allows_alpha_splitting = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_sprite_mode = Some(rdr.read_i32()?);
            m_sprite_extrude = Some(rdr.read_u32()?);
            m_sprite_mesh_type = Some(rdr.read_i32()?);
            m_alignment = Some(rdr.read_i32()?);
            m_sprite_pivot = Some(Vector2f::parse(rdr, ver)?);
            m_sprite_border = Some(Vector4f::parse(rdr, ver)?);
            m_sprite_pixels_to_units = Some(rdr.read_f32()?);
            m_alpha_is_transparency = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_rgbm = Some(rdr.read_i32()?);
            m_compression_quality = Some(rdr.read_i32()?);
            m_sprite_mode = Some(rdr.read_i32()?);
            m_sprite_extrude = Some(rdr.read_u32()?);
            m_sprite_mesh_type = Some(rdr.read_i32()?);
            m_alignment = Some(rdr.read_i32()?);
            m_sprite_pivot = Some(Vector2f::parse(rdr, ver)?);
            m_sprite_border = Some(Vector4f::parse(rdr, ver)?);
            m_sprite_pixels_to_units = Some(rdr.read_f32()?);
            m_alpha_is_transparency = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_compression_quality = Some(rdr.read_i32()?);
            m_sprite_mode = Some(rdr.read_i32()?);
            m_sprite_extrude = Some(rdr.read_u32()?);
            m_sprite_mesh_type = Some(rdr.read_i32()?);
            m_alignment = Some(rdr.read_i32()?);
            m_sprite_pivot = Some(Vector2f::parse(rdr, ver)?);
            m_sprite_border = Some(Vector4f::parse(rdr, ver)?);
            m_sprite_pixels_to_units = Some(rdr.read_f32()?);
            m_alpha_is_transparency = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_compression_quality = Some(rdr.read_i32()?);
            m_sprite_mode = Some(rdr.read_i32()?);
            m_sprite_extrude = Some(rdr.read_u32()?);
            m_sprite_mesh_type = Some(rdr.read_i32()?);
            m_alignment = Some(rdr.read_i32()?);
            m_sprite_pivot = Some(Vector2f::parse(rdr, ver)?);
            m_sprite_pixels_to_units = Some(rdr.read_f32()?);
            m_alpha_is_transparency = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_compression_quality = Some(rdr.read_i32()?);
            m_alpha_is_transparency = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_compression_quality = Some(rdr.read_i32()?);
        } else {
        }
        m_texture_type = Some(TextureImporterMTextureTypeVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 1, 13) {
            m_texture_shape = Some(rdr.read_i32()?);
            m_single_channel_component = Some(rdr.read_i32()?);
            m_flipbook_rows = Some(rdr.read_i32()?);
            m_flipbook_columns = Some(rdr.read_i32()?);
            m_max_texture_size_set = Some(rdr.read_i32()?);
            m_compression_quality_set = Some(rdr.read_i32()?);
            m_texture_format_set = Some(TextureImporterMTextureFormatSetVariant::parse(rdr, ver)?);
            m_ignore_png_gamma = Some(rdr.read_i32()?);
            m_apply_gamma_decoding =
                Some(TextureImporterMApplyGammaDecodingVariant::parse(rdr, ver)?);
            m_swizzle = Some(TextureImporterMSwizzleVariant::parse(rdr, ver)?);
            m_cookie_light_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_texture_shape = Some(rdr.read_i32()?);
            m_single_channel_component = Some(rdr.read_i32()?);
            m_flipbook_rows = Some(rdr.read_i32()?);
            m_flipbook_columns = Some(rdr.read_i32()?);
            m_max_texture_size_set = Some(rdr.read_i32()?);
            m_compression_quality_set = Some(rdr.read_i32()?);
            m_texture_format_set = Some(TextureImporterMTextureFormatSetVariant::parse(rdr, ver)?);
            m_ignore_png_gamma = Some(rdr.read_i32()?);
            m_apply_gamma_decoding =
                Some(TextureImporterMApplyGammaDecodingVariant::parse(rdr, ver)?);
            m_swizzle = Some(TextureImporterMSwizzleVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 9) {
            m_texture_shape = Some(rdr.read_i32()?);
            m_single_channel_component = Some(rdr.read_i32()?);
            m_flipbook_rows = Some(rdr.read_i32()?);
            m_flipbook_columns = Some(rdr.read_i32()?);
            m_max_texture_size_set = Some(rdr.read_i32()?);
            m_compression_quality_set = Some(rdr.read_i32()?);
            m_texture_format_set = Some(TextureImporterMTextureFormatSetVariant::parse(rdr, ver)?);
            m_ignore_png_gamma = Some(rdr.read_i32()?);
            m_apply_gamma_decoding =
                Some(TextureImporterMApplyGammaDecodingVariant::parse(rdr, ver)?);
            m_cookie_light_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_texture_shape = Some(rdr.read_i32()?);
            m_single_channel_component = Some(rdr.read_i32()?);
            m_flipbook_rows = Some(rdr.read_i32()?);
            m_flipbook_columns = Some(rdr.read_i32()?);
            m_max_texture_size_set = Some(rdr.read_i32()?);
            m_compression_quality_set = Some(rdr.read_i32()?);
            m_texture_format_set = Some(TextureImporterMTextureFormatSetVariant::parse(rdr, ver)?);
            m_ignore_png_gamma = Some(rdr.read_i32()?);
            m_apply_gamma_decoding =
                Some(TextureImporterMApplyGammaDecodingVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 38) {
            m_texture_shape = Some(rdr.read_i32()?);
            m_single_channel_component = Some(rdr.read_i32()?);
            m_flipbook_rows = Some(rdr.read_i32()?);
            m_flipbook_columns = Some(rdr.read_i32()?);
            m_max_texture_size_set = Some(rdr.read_i32()?);
            m_compression_quality_set = Some(rdr.read_i32()?);
            m_texture_format_set = Some(TextureImporterMTextureFormatSetVariant::parse(rdr, ver)?);
            m_ignore_png_gamma = Some(rdr.read_i32()?);
            m_apply_gamma_decoding =
                Some(TextureImporterMApplyGammaDecodingVariant::parse(rdr, ver)?);
            m_cookie_light_type = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_texture_shape = Some(rdr.read_i32()?);
            m_single_channel_component = Some(rdr.read_i32()?);
            m_flipbook_rows = Some(rdr.read_i32()?);
            m_flipbook_columns = Some(rdr.read_i32()?);
            m_max_texture_size_set = Some(rdr.read_i32()?);
            m_compression_quality_set = Some(rdr.read_i32()?);
            m_texture_format_set = Some(TextureImporterMTextureFormatSetVariant::parse(rdr, ver)?);
            m_ignore_png_gamma = Some(rdr.read_i32()?);
            m_apply_gamma_decoding =
                Some(TextureImporterMApplyGammaDecodingVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_texture_shape = Some(rdr.read_i32()?);
            m_single_channel_component = Some(rdr.read_i32()?);
            m_max_texture_size_set = Some(rdr.read_i32()?);
            m_compression_quality_set = Some(rdr.read_i32()?);
            m_texture_format_set = Some(TextureImporterMTextureFormatSetVariant::parse(rdr, ver)?);
            m_ignore_png_gamma = Some(rdr.read_i32()?);
            m_apply_gamma_decoding =
                Some(TextureImporterMApplyGammaDecodingVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 6) {
            m_texture_shape = Some(rdr.read_i32()?);
            m_single_channel_component = Some(rdr.read_i32()?);
            m_max_texture_size_set = Some(rdr.read_i32()?);
            m_compression_quality_set = Some(rdr.read_i32()?);
            m_texture_format_set = Some(TextureImporterMTextureFormatSetVariant::parse(rdr, ver)?);
            m_apply_gamma_decoding =
                Some(TextureImporterMApplyGammaDecodingVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_texture_shape = Some(rdr.read_i32()?);
            m_single_channel_component = Some(rdr.read_i32()?);
            m_max_texture_size_set = Some(rdr.read_i32()?);
            m_compression_quality_set = Some(rdr.read_i32()?);
            m_texture_format_set = Some(TextureImporterMTextureFormatSetVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_texture_shape = Some(rdr.read_i32()?);
            m_max_texture_size_set = Some(rdr.read_i32()?);
            m_compression_quality_set = Some(rdr.read_i32()?);
            m_texture_format_set = Some(TextureImporterMTextureFormatSetVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
        } else {
            m_recommended_texture_format = Some(rdr.read_i32()?);
            m_source_texture_information = Some(SourceTextureInformation::parse(rdr, ver)?);
            rdr.align(4)?;
        }
        m_platform_settings = Some(TextureImporterMPlatformSettingsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_sprite_sheet = Some(SpriteSheetMetaData::parse(rdr, ver)?);
            m_mipmap_limit_group_name = Some(rdr.read_aligned_string()?);
            m_output = Some(TextureImporterMOutputVariant::parse(rdr, ver)?);
            m_psd_remove_matte = Some(TextureImporterMPsdRemoveMatteVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_sprite_sheet = Some(SpriteSheetMetaData::parse(rdr, ver)?);
            m_sprite_packing_tag = Some(rdr.read_aligned_string()?);
            m_output = Some(TextureImporterMOutputVariant::parse(rdr, ver)?);
            m_psd_remove_matte = Some(TextureImporterMPsdRemoveMatteVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2018, 2, 1) {
            m_sprite_sheet = Some(SpriteSheetMetaData::parse(rdr, ver)?);
            m_sprite_packing_tag = Some(rdr.read_aligned_string()?);
            m_output = Some(TextureImporterMOutputVariant::parse(rdr, ver)?);
            m_psd_remove_matte = Some(TextureImporterMPsdRemoveMatteVariant::parse(rdr, ver)?);
            m_psd_show_remove_matte_option = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_sprite_sheet = Some(SpriteSheetMetaData::parse(rdr, ver)?);
            m_sprite_packing_tag = Some(rdr.read_aligned_string()?);
            m_output = Some(TextureImporterMOutputVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_sprite_sheet = Some(SpriteSheetMetaData::parse(rdr, ver)?);
            m_sprite_packing_tag = Some(rdr.read_aligned_string()?);
            m_output = Some(TextureImporterMOutputVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_output = Some(TextureImporterMOutputVariant::parse(rdr, ver)?);
            m_user_data = Some(rdr.read_aligned_string()?);
        } else {
        }

        Ok(TextureImporter {
            m_name: m_name.unwrap(),
            m_file_id_to_recycle_name,
            m_mip_map_mode: m_mip_map_mode.unwrap(),
            m_internal_id_to_name_table,
            m_old_hash_identity,
            m_enable_mip_map: m_enable_mip_map.unwrap(),
            m_external_objects,
            m_new_hash_identity,
            m_linear_texture,
            m_used_file_i_ds,
            correct_gamma,
            m_s_rgb_texture,
            m_fade_out: m_fade_out.unwrap(),
            m_correct_gamma,
            m_border_mip_map: m_border_mip_map.unwrap(),
            m_mip_map_fade_distance_start: m_mip_map_fade_distance_start.unwrap(),
            m_mip_map_fade_distance_end: m_mip_map_fade_distance_end.unwrap(),
            m_mip_maps_preserve_coverage,
            m_convert_to_normal_map: m_convert_to_normal_map.unwrap(),
            m_alpha_test_reference_value,
            m_external_normal_map: m_external_normal_map.unwrap(),
            m_height_scale: m_height_scale.unwrap(),
            m_normal_map_filter: m_normal_map_filter.unwrap(),
            m_is_readable: m_is_readable.unwrap(),
            m_gray_scale_to_alpha: m_gray_scale_to_alpha.unwrap(),
            m_generate_cubemap: m_generate_cubemap.unwrap(),
            m_format: m_format.unwrap(),
            m_seamless_cubemap,
            m_max_texture_size: m_max_texture_size.unwrap(),
            m_cubemap_convolution,
            m_texture_settings: m_texture_settings.unwrap(),
            m_cubemap_convolution_steps,
            m_streaming_mipmaps,
            m_flip_green_channel,
            m_npot_scale: m_npot_scale.unwrap(),
            m_cubemap_convolution_exponent,
            m_streaming_mipmaps_priority,
            m_lightmap: m_lightmap.unwrap(),
            m_compression_quality,
            m_vt_only,
            m_texture_type: m_texture_type.unwrap(),
            m_ignore_master_texture_limit,
            m_recommended_texture_format,
            m_alpha_is_transparency,
            m_sprite_mode,
            m_ignore_mipmap_limit,
            m_source_texture_information,
            m_platform_settings: m_platform_settings.unwrap(),
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
            m_single_channel_component,
            m_asset_bundle_name,
            m_asset_bundle_variant,
            m_flipbook_rows,
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
