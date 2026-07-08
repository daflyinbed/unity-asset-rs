//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;


/// 版本枚举：MovieImporterMLinearTextureVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum MovieImporterMLinearTextureVariant {
    Default(bool),
    V4_0(bool),
}

impl crate::Parse for MovieImporterMLinearTextureVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Ok(Self::V4_0(v))
        } else {
            let v = r.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct MovieImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_quality: f32,
    pub m_external_objects: Option<Vec<(SourceAssetIdentifier, PPtr)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_linear_texture: Option<MovieImporterMLinearTextureVariant>,
    pub m_used_file_i_ds: Option<Vec<i64>>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_user_data: Option<String>,
    pub m_asset_bundle_name: Option<String>,
    pub m_asset_bundle_variant: Option<String>,
}

impl crate::Parse for MovieImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_quality = None;
        let mut m_external_objects = None;
        let mut m_old_hash_identity = None;
        let mut m_linear_texture = None;
        let mut m_used_file_i_ds = None;
        let mut m_new_hash_identity = None;
        let mut m_user_data = None;
        let mut m_asset_bundle_name = None;
        let mut m_asset_bundle_variant = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_used_file_i_ds = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i64() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_external_objects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = SourceAssetIdentifier::parse(r, ver)?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_file_id_to_recycle_name = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_old_hash_identity = Some(MdFour::parse(r, ver)?);
            r.align(4)?;
            m_new_hash_identity = Some(MdFour::parse(r, ver)?);
            r.align(4)?;
        }
            m_quality = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_linear_texture = Some(MovieImporterMLinearTextureVariant::parse(r, ver)?);
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_asset_bundle_variant = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_linear_texture = Some(MovieImporterMLinearTextureVariant::parse(r, ver)?);
            m_user_data = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_linear_texture = Some(MovieImporterMLinearTextureVariant::parse(r, ver)?);
        } else {
        }

        Ok(MovieImporter {
            m_name: m_name.unwrap(),
            m_file_id_to_recycle_name,
            m_quality: m_quality.unwrap(),
            m_external_objects,
            m_old_hash_identity,
            m_linear_texture,
            m_used_file_i_ds,
            m_new_hash_identity,
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}
