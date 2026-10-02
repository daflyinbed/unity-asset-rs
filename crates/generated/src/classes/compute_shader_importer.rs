//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

/// 版本枚举：ComputeShaderImporterMNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderImporterMNameVariant {
    Default(String),
    V5_0(String),
}

impl crate::Parse for ComputeShaderImporterMNameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_aligned_string()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderImporterMNameVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ComputeShaderImporterMCurrentApiMaskVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderImporterMCurrentApiMaskVariant {
    Default(u32),
    V2020_2(u32),
}

impl crate::Parse for ComputeShaderImporterMCurrentApiMaskVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2020_2(v))
        } else {
            let v = rdr.read_u32()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ComputeShaderImporterMCurrentApiMaskVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ComputeShaderImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for ComputeShaderImporterMExternalObjectsVariant {
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

impl crate::ToValue for ComputeShaderImporterMExternalObjectsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ComputeShaderImporter {
    pub m_name: ComputeShaderImporterMNameVariant,
    pub m_user_data: String,
    pub m_current_build_target: Option<i32>,
    pub m_current_api_mask: Option<ComputeShaderImporterMCurrentApiMaskVariant>,
    pub m_external_objects: Option<ComputeShaderImporterMExternalObjectsVariant>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_asset_bundle_name: Option<String>,
    pub m_preprocessor_override: Option<i32>,
    pub m_asset_bundle_variant: Option<String>,
}

impl crate::Parse for ComputeShaderImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_user_data = None;
        let mut m_current_build_target = None;
        let mut m_current_api_mask = None;
        let mut m_external_objects = None;
        let mut m_used_file_i_ds = None;
        let mut m_asset_bundle_name = None;
        let mut m_preprocessor_override = None;
        let mut m_asset_bundle_variant = None;
        m_name = Some(ComputeShaderImporterMNameVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_external_objects = Some(ComputeShaderImporterMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_external_objects = Some(ComputeShaderImporterMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_preprocessor_override = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_external_objects = Some(ComputeShaderImporterMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_current_api_mask = Some(ComputeShaderImporterMCurrentApiMaskVariant::parse(
                rdr, ver,
            )?);
            m_preprocessor_override = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(ComputeShaderImporterMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_current_api_mask = Some(ComputeShaderImporterMCurrentApiMaskVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(ComputeShaderImporterMExternalObjectsVariant::parse(
                rdr, ver,
            )?);
            m_current_api_mask = Some(ComputeShaderImporterMCurrentApiMaskVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 2) {
            m_current_api_mask = Some(ComputeShaderImporterMCurrentApiMaskVariant::parse(
                rdr, ver,
            )?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_current_build_target = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else {
        }
        m_user_data = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else {
        }

        Ok(ComputeShaderImporter {
            m_name: m_name.unwrap(),
            m_user_data: m_user_data.unwrap(),
            m_current_build_target,
            m_current_api_mask,
            m_external_objects,
            m_used_file_i_ds,
            m_asset_bundle_name,
            m_preprocessor_override,
            m_asset_bundle_variant,
        })
    }
}

impl crate::ToValue for ComputeShaderImporter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_UserData".to_string(), self.m_user_data.to_value())),
            crate::opt_field("m_CurrentBuildTarget", &self.m_current_build_target),
            crate::opt_field("m_CurrentAPIMask", &self.m_current_api_mask),
            crate::opt_field("m_ExternalObjects", &self.m_external_objects),
            crate::opt_field("m_UsedFileIDs", &self.m_used_file_i_ds),
            crate::opt_field("m_AssetBundleName", &self.m_asset_bundle_name),
            crate::opt_field("m_PreprocessorOverride", &self.m_preprocessor_override),
            crate::opt_field("m_AssetBundleVariant", &self.m_asset_bundle_variant),
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
                "m_UserData" => (self.m_user_data).inspect_value(path, offset, limit),
                "m_CurrentBuildTarget" => {
                    (self.m_current_build_target).inspect_value(path, offset, limit)
                }
                "m_CurrentAPIMask" => (self.m_current_api_mask).inspect_value(path, offset, limit),
                "m_ExternalObjects" => (self.m_external_objects).inspect_value(path, offset, limit),
                "m_UsedFileIDs" => (self.m_used_file_i_ds).inspect_value(path, offset, limit),
                "m_AssetBundleName" => {
                    (self.m_asset_bundle_name).inspect_value(path, offset, limit)
                }
                "m_PreprocessorOverride" => {
                    (self.m_preprocessor_override).inspect_value(path, offset, limit)
                }
                "m_AssetBundleVariant" => {
                    (self.m_asset_bundle_variant).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_user_data).inspect_value(path, offset, limit))
                .map(|v| ("m_UserData".to_string(), v)),
            ((self.m_current_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_CurrentBuildTarget".to_string(), v)),
            ((self.m_current_api_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_CurrentAPIMask".to_string(), v)),
            ((self.m_external_objects).inspect_value(path, offset, limit))
                .map(|v| ("m_ExternalObjects".to_string(), v)),
            ((self.m_used_file_i_ds).inspect_value(path, offset, limit))
                .map(|v| ("m_UsedFileIDs".to_string(), v)),
            ((self.m_asset_bundle_name).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleName".to_string(), v)),
            ((self.m_preprocessor_override).inspect_value(path, offset, limit))
                .map(|v| ("m_PreprocessorOverride".to_string(), v)),
            ((self.m_asset_bundle_variant).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetBundleVariant".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
