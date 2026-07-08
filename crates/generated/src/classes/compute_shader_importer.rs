//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;


/// 版本枚举：ComputeShaderImporterMCurrentApiMaskVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderImporterMCurrentApiMaskVariant {
    Default(u32),
    V2020_2(u32),
}

impl crate::Parse for ComputeShaderImporterMCurrentApiMaskVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            let v = r.read_u32()?;
            Ok(Self::V2020_2(v))
        } else {
            let v = r.read_u32()?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ComputeShaderImporter {
    pub m_name: String,
    pub m_user_data: String,
    pub m_current_build_target: Option<i32>,
    pub m_current_api_mask: Option<ComputeShaderImporterMCurrentApiMaskVariant>,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_asset_bundle_name: Option<String>,
    pub m_preprocessor_override: Option<i32>,
    pub m_asset_bundle_variant: Option<String>,
}

impl crate::Parse for ComputeShaderImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
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
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_preprocessor_override = Some(r.read_i32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_current_api_mask = Some(ComputeShaderImporterMCurrentApiMaskVariant::parse(r, ver)?);
            m_preprocessor_override = Some(r.read_i32()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_current_api_mask = Some(ComputeShaderImporterMCurrentApiMaskVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_current_api_mask = Some(ComputeShaderImporterMCurrentApiMaskVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 2) {
            m_current_api_mask = Some(ComputeShaderImporterMCurrentApiMaskVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_current_build_target = Some(r.read_i32()?);
            r.align(4)?;
        } else {
        }
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
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
