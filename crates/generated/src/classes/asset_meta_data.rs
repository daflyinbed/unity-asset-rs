//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::md_four::MdFour;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct AssetMetaData {
    pub parent: Object,
    pub guid: Guid,
    pub path_name: String,
    pub original_changeset: Option<u32>,
    pub original_name: String,
    pub time_created: Option<u64>,
    pub labels: Vec<String>,
    pub original_parent: Option<Guid>,
    pub asset_store_ref: u64,
    pub original_digest: Option<MdFour>,
    pub license_type: Option<i32>,
}

impl crate::Parse for AssetMetaData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let guid = Guid::parse(r, ver)?;
        let path_name = r.read_aligned_string()?;
        r.align(4)?;
        let original_changeset = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let original_name = r.read_aligned_string()?;
        r.align(4)?;
        let time_created = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_u64()?)
        } else {
            None
        };
        let labels = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let original_parent = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(Guid::parse(r, ver)?)
        } else {
            None
        };
        let asset_store_ref = r.read_u64()?;
        let original_digest = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = MdFour::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let license_type = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(AssetMetaData {
            parent,
            guid,
            path_name,
            original_changeset,
            original_name,
            time_created,
            labels,
            original_parent,
            asset_store_ref,
            original_digest,
            license_type,
        })
    }
}
