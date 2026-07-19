//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::packing_settings::PackingSettings;
use crate::subclasses::secondary_texture_settings::SecondaryTextureSettings;
use crate::subclasses::texture_importer_platform_settings::TextureImporterPlatformSettings;
use crate::subclasses::texture_settings::TextureSettings;

#[derive(Debug, Clone)]
pub struct SpriteAtlasEditorData {
    pub texture_settings: TextureSettings,
    pub platform_settings: Vec<TextureImporterPlatformSettings>,
    pub packing_settings: PackingSettings,
    pub variant_multiplier: f32,
    pub secondary_texture_settings: Option<Vec<(String, SecondaryTextureSettings)>>,
    pub packables: Vec<PPtr>,
    pub total_sprite_surface_area: Option<u32>,
    pub bind_as_default: bool,
    pub is_atlas_v2: bool,
    pub cached_data: PPtr,
}

impl crate::Parse for SpriteAtlasEditorData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut texture_settings = None;
        let mut platform_settings = None;
        let mut packing_settings = None;
        let mut variant_multiplier = None;
        let mut secondary_texture_settings = None;
        let mut packables = None;
        let mut total_sprite_surface_area = None;
        let mut bind_as_default = None;
        let mut is_atlas_v2 = None;
        let mut cached_data = None;
        texture_settings = Some(TextureSettings::parse(rdr, ver)?);
        platform_settings = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TextureImporterPlatformSettings::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        packing_settings = Some(PackingSettings::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            secondary_texture_settings = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = SecondaryTextureSettings::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }
        variant_multiplier = Some(rdr.read_f32()?);
        packables = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2020, 3, 19) {
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            total_sprite_surface_area = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 12) {
        } else {
            total_sprite_surface_area = Some(rdr.read_u32()?);
        }
        bind_as_default = Some(rdr.read_bool()?);
        is_atlas_v2 = Some(rdr.read_bool()?);
        rdr.align(4)?;
        cached_data = Some(crate::PPtr::parse(rdr, ver)?);

        Ok(SpriteAtlasEditorData {
            texture_settings: texture_settings.unwrap(),
            platform_settings: platform_settings.unwrap(),
            packing_settings: packing_settings.unwrap(),
            variant_multiplier: variant_multiplier.unwrap(),
            secondary_texture_settings,
            packables: packables.unwrap(),
            total_sprite_surface_area,
            bind_as_default: bind_as_default.unwrap(),
            is_atlas_v2: is_atlas_v2.unwrap(),
            cached_data: cached_data.unwrap(),
        })
    }
}
