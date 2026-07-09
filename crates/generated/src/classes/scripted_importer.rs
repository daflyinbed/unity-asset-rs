//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;


/// 版本枚举：ScriptedImporterMNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ScriptedImporterMNameVariant {
    Default(String),
    V2017_3(String),
}

impl crate::Parse for ScriptedImporterMNameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = rdr.read_aligned_string()?;
            Ok(Self::V2017_3(v))
        } else {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ScriptedImporter {
    pub m_name: ScriptedImporterMNameVariant,
    pub m_user_data: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i64, String)>>,
    pub m_internal_id_to_name_table: Option<Vec<((i32, i64), String)>>,
    pub m_asset_bundle_name: String,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_asset_bundle_variant: String,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_script: PPtr,
}

impl crate::Parse for ScriptedImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_user_data = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_internal_id_to_name_table = None;
        let mut m_asset_bundle_name = None;
        let mut m_external_objects = None;
        let mut m_asset_bundle_variant = None;
        let mut m_used_file_i_ds = None;
        let mut m_script = None;
            m_name = Some(ScriptedImporterMNameVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_internal_id_to_name_table = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = rdr.read_i32()?; let b = rdr.read_i64()?; Ok((a, b)) } )?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_used_file_i_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_file_id_to_recycle_name = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i64()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_external_objects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(rdr, ver)?; let b = crate::PPtr::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }
            m_user_data = Some(rdr.read_aligned_string()?);
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            m_script = Some(crate::PPtr::parse(rdr, ver)?);

        Ok(ScriptedImporter {
            m_name: m_name.unwrap(),
            m_user_data: m_user_data.unwrap(),
            m_file_id_to_recycle_name,
            m_internal_id_to_name_table,
            m_asset_bundle_name: m_asset_bundle_name.unwrap(),
            m_external_objects,
            m_asset_bundle_variant: m_asset_bundle_variant.unwrap(),
            m_used_file_i_ds,
            m_script: m_script.unwrap(),
        })
    }
}
