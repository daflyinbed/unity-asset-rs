//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;
use crate::subclasses::vfx_template::VfxTemplate;

/// 版本枚举：VisualEffectImporterMTemplateVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VisualEffectImporterMTemplateVariant {
    Default(Box<VfxTemplate>),
    V6000_3(Box<VfxTemplate>),
}

impl crate::Parse for VisualEffectImporterMTemplateVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = ({ Ok(Box::new(VfxTemplate::parse(rdr, ver)?)) })?;
            Ok(Self::V6000_3(v))
        } else {
            let v = ({ Ok(Box::new(VfxTemplate::parse(rdr, ver)?)) })?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VisualEffectImporterMTemplateVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct VisualEffectImporter {
    pub m_name: String,
    pub m_external_objects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_user_data: String,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_asset_bundle_name: String,
    pub m_template: Option<VisualEffectImporterMTemplateVariant>,
    pub m_asset_bundle_variant: String,
    pub m_use_as_template: Option<bool>,
}

impl crate::Parse for VisualEffectImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_external_objects = None;
        let mut m_user_data = None;
        let mut m_used_file_i_ds = None;
        let mut m_asset_bundle_name = None;
        let mut m_template = None;
        let mut m_asset_bundle_variant = None;
        let mut m_use_as_template = None;
        m_name = Some(rdr.read_aligned_string()?);
        m_external_objects = Some(
            ({
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
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_template = Some(VisualEffectImporterMTemplateVariant::parse(rdr, ver)?);
            m_use_as_template = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2023, 2, 0) {
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_template = Some(VisualEffectImporterMTemplateVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }
        m_user_data = Some(rdr.read_aligned_string()?);
        m_asset_bundle_name = Some(rdr.read_aligned_string()?);
        m_asset_bundle_variant = Some(rdr.read_aligned_string()?);

        Ok(VisualEffectImporter {
            m_name: m_name.unwrap(),
            m_external_objects: m_external_objects.unwrap(),
            m_user_data: m_user_data.unwrap(),
            m_used_file_i_ds,
            m_asset_bundle_name: m_asset_bundle_name.unwrap(),
            m_template,
            m_asset_bundle_variant: m_asset_bundle_variant.unwrap(),
            m_use_as_template,
        })
    }
}

impl crate::ToValue for VisualEffectImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_ExternalObjects".to_string(),
                self.m_external_objects.to_value(),
            )),
            Some(("m_UserData".to_string(), self.m_user_data.to_value())),
            crate::opt_field("m_UsedFileIDs", &self.m_used_file_i_ds),
            Some((
                "m_AssetBundleName".to_string(),
                self.m_asset_bundle_name.to_value(),
            )),
            crate::opt_field("m_Template", &self.m_template),
            Some((
                "m_AssetBundleVariant".to_string(),
                self.m_asset_bundle_variant.to_value(),
            )),
            crate::opt_field("m_UseAsTemplate", &self.m_use_as_template),
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
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_Template" => (self.m_template).inspect_value(path, offset, limit),
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                "m_UseAsTemplate" => (self.m_use_as_template).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_template).inspect_value(path, offset, limit))
                .map(|v| ("m_Template".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
            ((self.m_use_as_template).inspect_value(path, offset, limit))
                .map(|v| ("m_UseAsTemplate".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
