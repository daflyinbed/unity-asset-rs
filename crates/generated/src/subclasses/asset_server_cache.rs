//! 自动生成。请勿手动修改。

use crate::subclasses::cached_asset_meta_data::CachedAssetMetaData;
use crate::subclasses::deleted_item::DeletedItem;
use crate::subclasses::guid::Guid;
use crate::subclasses::item::Item;

#[derive(Debug, Clone)]
pub struct AssetServerCache {
    pub m_object_hide_flags: u32,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_items = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(r, ver)?; let b = Item::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_deleted_items = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(r, ver)?; let b = DeletedItem::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_last_commit_message = r.read_aligned_string()?;
        r.align(4)?;
        let m_commit_item_selection = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_working_item_meta_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(r, ver)?; let b = CachedAssetMetaData::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_latest_server_changeset = r.read_i32()?;
        let m_caches_initialized = r.read_i32()?;
        let m_modified_items = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Guid::parse(r, ver)?; let b = Item::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(AssetServerCache {
            m_object_hide_flags,
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
