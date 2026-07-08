//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::sprite_atlas_asset_data::SpriteAtlasAssetData;
use crate::subclasses::sprite_atlas_editor_data::SpriteAtlasEditorData;


/// 版本枚举：SpriteAtlasAssetMImporterDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteAtlasAssetMImporterDataVariant {
    Default(SpriteAtlasEditorData),
    V2022_1(SpriteAtlasAssetData),
}

impl crate::Parse for SpriteAtlasAssetMImporterDataVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = SpriteAtlasAssetData::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V2022_1(v))
        } else {
            let v = SpriteAtlasEditorData::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SpriteAtlasAsset {
    pub m_name: String,
    pub m_master_atlas: PPtr,
    pub m_importer_data: SpriteAtlasAssetMImporterDataVariant,
    pub m_is_variant: bool,
    pub m_scriptable_packer: Option<PPtr>,
}

impl crate::Parse for SpriteAtlasAsset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_master_atlas = crate::PPtr::parse(r, ver)?;
        let m_importer_data = SpriteAtlasAssetMImporterDataVariant::parse(r, ver)?;
        let m_is_variant = r.read_bool()?;
        r.align(4)?;
        let m_scriptable_packer = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SpriteAtlasAsset {
            m_name,
            m_master_atlas,
            m_importer_data,
            m_is_variant,
            m_scriptable_packer,
        })
    }
}
