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
        let m_items = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = Guid::parse(rdr, ver)?;
                        let b = Item::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_deleted_items = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = Guid::parse(rdr, ver)?;
                        let b = DeletedItem::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_last_commit_message = rdr.read_aligned_string()?;
        let m_commit_item_selection = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Guid::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_working_item_meta_data = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = Guid::parse(rdr, ver)?;
                        let b = CachedAssetMetaData::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_latest_server_changeset = rdr.read_i32()?;
        let m_caches_initialized = rdr.read_i32()?;
        let m_modified_items = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = Guid::parse(rdr, ver)?;
                        let b = Item::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

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

impl crate::ToValue for AssetServerCache {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Items".to_string(), self.m_items.to_value())),
            Some((
                "m_DeletedItems".to_string(),
                self.m_deleted_items.to_value(),
            )),
            Some((
                "m_LastCommitMessage".to_string(),
                self.m_last_commit_message.to_value(),
            )),
            Some((
                "m_CommitItemSelection".to_string(),
                self.m_commit_item_selection.to_value(),
            )),
            Some((
                "m_WorkingItemMetaData".to_string(),
                self.m_working_item_meta_data.to_value(),
            )),
            Some((
                "m_LatestServerChangeset".to_string(),
                self.m_latest_server_changeset.to_value(),
            )),
            Some((
                "m_CachesInitialized".to_string(),
                self.m_caches_initialized.to_value(),
            )),
            Some((
                "m_ModifiedItems".to_string(),
                self.m_modified_items.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Items" => (self.m_items).inspect_value(path, offset, limit),
                "m_DeletedItems" => (self.m_deleted_items).inspect_value(path, offset, limit),
                "m_LastCommitMessage" => {
                    (self.m_last_commit_message).inspect_value(path, offset, limit)
                }
                "m_CommitItemSelection" => {
                    (self.m_commit_item_selection).inspect_value(path, offset, limit)
                }
                "m_WorkingItemMetaData" => {
                    (self.m_working_item_meta_data).inspect_value(path, offset, limit)
                }
                "m_LatestServerChangeset" => {
                    (self.m_latest_server_changeset).inspect_value(path, offset, limit)
                }
                "m_CachesInitialized" => {
                    (self.m_caches_initialized).inspect_value(path, offset, limit)
                }
                "m_ModifiedItems" => (self.m_modified_items).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_items).inspect_value(path, offset, limit)).map(|v| ("m_Items".to_string(), v)),
            ((self.m_deleted_items).inspect_value(path, offset, limit))
                .map(|v| ("m_DeletedItems".to_string(), v)),
            ((self.m_last_commit_message).inspect_value(path, offset, limit))
                .map(|v| ("m_LastCommitMessage".to_string(), v)),
            ((self.m_commit_item_selection).inspect_value(path, offset, limit))
                .map(|v| ("m_CommitItemSelection".to_string(), v)),
            ((self.m_working_item_meta_data).inspect_value(path, offset, limit))
                .map(|v| ("m_WorkingItemMetaData".to_string(), v)),
            ((self.m_latest_server_changeset).inspect_value(path, offset, limit))
                .map(|v| ("m_LatestServerChangeset".to_string(), v)),
            ((self.m_caches_initialized).inspect_value(path, offset, limit))
                .map(|v| ("m_CachesInitialized".to_string(), v)),
            ((self.m_modified_items).inspect_value(path, offset, limit))
                .map(|v| ("m_ModifiedItems".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
