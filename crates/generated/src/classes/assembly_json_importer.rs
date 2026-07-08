//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;


/// 版本枚举：AssemblyJsonImporterMNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssemblyJsonImporterMNameVariant {
    Default(String),
    V2017_2(String),
}

impl crate::Parse for AssemblyJsonImporterMNameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_aligned_string()?;
            Ok(Self::V2017_2(v))
        } else {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AssemblyJsonImporter {
    pub m_name: AssemblyJsonImporterMNameVariant,
    pub m_user_data: String,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_asset_bundle_name: String,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for AssemblyJsonImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_user_data = None;
        let mut m_external_objects = None;
        let mut m_asset_bundle_name = None;
        let mut m_used_file_i_ds = None;
        let mut m_asset_bundle_variant = None;
            m_name = Some(AssemblyJsonImporterMNameVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_used_file_i_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
        }
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);

        Ok(AssemblyJsonImporter {
            m_name: m_name.unwrap(),
            m_user_data: m_user_data.unwrap(),
            m_external_objects,
            m_asset_bundle_name: m_asset_bundle_name.unwrap(),
            m_used_file_i_ds,
            m_asset_bundle_variant: m_asset_bundle_variant.unwrap(),
        })
    }
}
