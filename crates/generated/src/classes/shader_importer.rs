//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

/// 版本枚举：ShaderImporterMNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ShaderImporterMNameVariant {
    Default(String),
    V4_0(String),
    V4_2(String),
}

impl crate::Parse for ShaderImporterMNameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
            let v = rdr.read_aligned_string()?;
            Ok(Self::V4_2(v))
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_aligned_string()?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ShaderImporterMExternalObjectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ShaderImporterMExternalObjectsVariant {
    Default(Vec<(SourceAssetIdentifier, PPtr)>),
    V2019_1(Vec<(SourceAssetIdentifier, PPtr)>),
}

impl crate::Parse for ShaderImporterMExternalObjectsVariant {
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
#[derive(Debug, Clone)]
pub struct ShaderImporter {
    pub m_name: ShaderImporterMNameVariant,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_user_data: Option<String>,
    pub m_default_textures: Option<Vec<(String, PPtr)>>,
    pub m_external_objects: Option<ShaderImporterMExternalObjectsVariant>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_asset_bundle_name: Option<String>,
    pub m_non_modifiable_textures: Option<Vec<(String, PPtr)>>,
    pub m_asset_bundle_variant: Option<String>,
    pub m_preprocessor_override: Option<i32>,
}

impl crate::Parse for ShaderImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_user_data = None;
        let mut m_default_textures = None;
        let mut m_external_objects = None;
        let mut m_old_hash_identity = None;
        let mut m_used_file_i_ds = None;
        let mut m_new_hash_identity = None;
        let mut m_asset_bundle_name = None;
        let mut m_non_modifiable_textures = None;
        let mut m_asset_bundle_variant = None;
        let mut m_preprocessor_override = None;
        m_name = Some(ShaderImporterMNameVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_external_objects = Some(ShaderImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_default_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_non_modifiable_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_external_objects = Some(ShaderImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_default_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_non_modifiable_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_preprocessor_override = Some(rdr.read_i32()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(ShaderImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_used_file_i_ds = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_i64())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_default_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_non_modifiable_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_external_objects = Some(ShaderImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_default_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_non_modifiable_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(ShaderImporterMExternalObjectsVariant::parse(rdr, ver)?);
            m_default_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_default_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 2, 0) {
            m_default_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_user_data = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_file_id_to_recycle_name = Some(
                ({
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
                })?,
            );
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
        }

        Ok(ShaderImporter {
            m_name: m_name.unwrap(),
            m_file_id_to_recycle_name,
            m_user_data,
            m_default_textures,
            m_external_objects,
            m_old_hash_identity,
            m_used_file_i_ds,
            m_new_hash_identity,
            m_asset_bundle_name,
            m_non_modifiable_textures,
            m_asset_bundle_variant,
            m_preprocessor_override,
        })
    }
}
