//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;


/// 版本枚举：MonoImporterMDefaultReferencesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoImporterMDefaultReferencesVariant {
    Default(Vec<(String, PPtr)>),
    V2017_1(Vec<(String, PPtr)>),
}

impl crate::Parse for MonoImporterMDefaultReferencesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：MonoImporterIconVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MonoImporterIconVariant {
    Default(PPtr),
    V4_0(PPtr),
}

impl crate::Parse for MonoImporterIconVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::V4_0(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct MonoImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_default_references: MonoImporterMDefaultReferencesVariant,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub execution_order: i16,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub icon: MonoImporterIconVariant,
    pub m_user_data: Option<String>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
}

impl crate::Parse for MonoImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_default_references = None;
        let mut m_external_objects = None;
        let mut m_old_hash_identity = None;
        let mut execution_order = None;
        let mut m_used_file_i_ds = None;
        let mut m_new_hash_identity = None;
        let mut icon = None;
        let mut m_user_data = None;
        let mut m_asset_bundle_name = None;
        let mut m_asset_bundle_variant = None;
            m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_used_file_i_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_file_id_to_recycle_name = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i32()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
        }
            m_default_references = Some(MonoImporterMDefaultReferencesVariant::parse(rdr, ver)?);
            execution_order = Some(rdr.read_i16()?);
            rdr.align(4)?;
            icon = Some(MonoImporterIconVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_user_data = Some(rdr.read_aligned_string()?);
        } else {
        }

        Ok(MonoImporter {
            m_name: m_name.unwrap(),
            m_file_id_to_recycle_name,
            m_default_references: m_default_references.unwrap(),
            m_external_objects,
            m_old_hash_identity,
            execution_order: execution_order.unwrap(),
            m_used_file_i_ds,
            m_new_hash_identity,
            icon: icon.unwrap(),
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}
