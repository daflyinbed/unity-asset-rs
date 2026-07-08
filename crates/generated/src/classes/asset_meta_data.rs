//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::md_four::MdFour;


/// 版本枚举：AssetMetaDataOriginalDigestVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssetMetaDataOriginalDigestVariant {
    Default(MdFour),
    V5_0(Hash128),
}

impl crate::Parse for AssetMetaDataOriginalDigestVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = Hash128::parse(r, ver)?;
            Ok(Self::V5_0(v))
        } else {
            let v = MdFour::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AssetMetaData {
    pub guid: Guid,
    pub path_name: String,
    pub original_changeset: Option<u32>,
    pub original_name: String,
    pub time_created: Option<u64>,
    pub labels: Vec<String>,
    pub original_parent: Option<Guid>,
    pub asset_store_ref: u64,
    pub original_digest: Option<AssetMetaDataOriginalDigestVariant>,
    pub license_type: Option<i32>,
}

impl crate::Parse for AssetMetaData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut guid = None;
        let mut path_name = None;
        let mut original_changeset = None;
        let mut original_name = None;
        let mut time_created = None;
        let mut labels = None;
        let mut original_parent = None;
        let mut asset_store_ref = None;
        let mut original_digest = None;
        let mut license_type = None;
            guid = Some(Guid::parse(r, ver)?);
            path_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2017, 4, 6) {
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            time_created = Some(r.read_u64()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 3) {
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            time_created = Some(r.read_u64()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            time_created = Some(r.read_u64()?);
            original_changeset = Some(r.read_u32()?);
        } else {
            original_changeset = Some(r.read_u32()?);
        }
            original_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
        } else {
            original_parent = Some(Guid::parse(r, ver)?);
            original_digest = Some(AssetMetaDataOriginalDigestVariant::parse(r, ver)?);
        }
            labels = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            asset_store_ref = Some(r.read_u64()?);
        if *ver >= crate::UnityVersion::new(2017, 4, 6) {
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            license_type = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 3) {
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            license_type = Some(r.read_i32()?);
        } else {
        }

        Ok(AssetMetaData {
            guid: guid.unwrap(),
            path_name: path_name.unwrap(),
            original_changeset,
            original_name: original_name.unwrap(),
            time_created,
            labels: labels.unwrap(),
            original_parent,
            asset_store_ref: asset_store_ref.unwrap(),
            original_digest,
            license_type,
        })
    }
}
