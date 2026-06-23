//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::md_four::MdFour;

#[derive(Debug, Clone)]
pub struct AssetMetaData {
    pub m_object_hide_flags: u32,
    pub guid: Guid,
    pub path_name: String,
    pub original_changeset: u32,
    pub original_name: String,
    pub original_parent: Guid,
    pub original_digest: MdFour,
    pub labels: Vec<String>,
    pub asset_store_ref: u64,
}

impl crate::Parse for AssetMetaData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let guid = Guid::parse(r, ver)?;
        let path_name = r.read_aligned_string()?;
        r.align(4)?;
        let original_changeset = r.read_u32()?;
        let original_name = r.read_aligned_string()?;
        r.align(4)?;
        let original_parent = Guid::parse(r, ver)?;
        let original_digest = MdFour::parse(r, ver)?;
        r.align(4)?;
        let labels = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let asset_store_ref = r.read_u64()?;

        Ok(AssetMetaData {
            m_object_hide_flags,
            guid,
            path_name,
            original_changeset,
            original_name,
            original_parent,
            original_digest,
            labels,
            asset_store_ref,
        })
    }
}
