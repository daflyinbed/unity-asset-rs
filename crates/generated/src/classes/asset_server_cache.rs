//! 自动生成。请勿手动修改。

use crate::subclasses::cached_asset_meta_data::CachedAssetMetaData;
use crate::subclasses::deleted_item::DeletedItem;
use crate::subclasses::guid::Guid;
use crate::subclasses::item::Item;

#[derive(Debug, Clone)]
pub struct AssetServerCache {
    pub m_items: Vec<(Guid, Item)>,
    pub m_deleted_items: Vec<(Guid, DeletedItem)>,
    pub m_last_commit_message: String,
    pub m_commit_item_selection: Vec<Guid>,
    pub m_working_item_meta_data: Vec<(Guid, CachedAssetMetaData)>,
    pub m_latest_server_changeset: i32,
    pub m_caches_initialized: i32,
    pub m_modified_items: Vec<(Guid, Item)>,
}

impl crate::Parse for AssetServerCache {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_items = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(rdr, ver)?; let b = Item::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_deleted_items = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(rdr, ver)?; let b = DeletedItem::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_last_commit_message = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_commit_item_selection = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_working_item_meta_data = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(rdr, ver)?; let b = CachedAssetMetaData::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_latest_server_changeset = rdr.read_i32()?;
        let m_caches_initialized = rdr.read_i32()?;
        let m_modified_items = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(rdr, ver)?; let b = Item::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(AssetServerCache {
            m_items,
            m_deleted_items,
            m_last_commit_message,
            m_commit_item_selection,
            m_working_item_meta_data,
            m_latest_server_changeset,
            m_caches_initialized,
            m_modified_items,
        })
    }
}
