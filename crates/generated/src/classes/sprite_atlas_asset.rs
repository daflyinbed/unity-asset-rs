//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::sprite_atlas_asset_data::SpriteAtlasAssetData;
use crate::subclasses::sprite_atlas_editor_data::SpriteAtlasEditorData;

/// 版本枚举：SpriteAtlasAssetMImporterDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SpriteAtlasAssetMImporterDataVariant {
    Default(Box<SpriteAtlasEditorData>),
    V2022_1(Box<SpriteAtlasAssetData>),
}

impl crate::Parse for SpriteAtlasAssetMImporterDataVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = ({ Ok(Box::new(SpriteAtlasAssetData::parse(rdr, ver)?)) })?;
            Ok(Self::V2022_1(v))
        } else {
            let v = ({ Ok(Box::new(SpriteAtlasEditorData::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SpriteAtlasAssetMImporterDataVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_master_atlas = crate::PPtr::parse(rdr, ver)?;
        let m_importer_data = SpriteAtlasAssetMImporterDataVariant::parse(rdr, ver)?;
        let m_is_variant = rdr.read_bool()?;
        rdr.align(4)?;
        let m_scriptable_packer = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
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

impl crate::ToValue for SpriteAtlasAsset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_MasterAtlas".to_string(), self.m_master_atlas.to_value())),
            Some((
                "m_ImporterData".to_string(),
                self.m_importer_data.to_value(),
            )),
            Some(("m_IsVariant".to_string(), self.m_is_variant.to_value())),
            crate::opt_field("m_ScriptablePacker", &self.m_scriptable_packer),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_MasterAtlas" => (self.m_master_atlas).inspect_value(path, offset, limit),
                "m_ImporterData" => (self.m_importer_data).inspect_value(path, offset, limit),
                "m_IsVariant" => (self.m_is_variant).inspect_value(path, offset, limit),
                "m_ScriptablePacker" => {
                    (self.m_scriptable_packer).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_master_atlas).inspect_value(path, offset, limit))
                .map(|v| ("m_MasterAtlas".to_string(), v)),
            ((self.m_importer_data).inspect_value(path, offset, limit))
                .map(|v| ("m_ImporterData".to_string(), v)),
            ((self.m_is_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_IsVariant".to_string(), v)),
            ((self.m_scriptable_packer).inspect_value(path, offset, limit))
                .map(|v| ("m_ScriptablePacker".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
