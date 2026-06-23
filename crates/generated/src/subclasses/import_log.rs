//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::import_log_import_log_entry::ImportLog_ImportLogEntry;

#[derive(Debug, Clone)]
pub struct ImportLog {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_name: String,
    pub m_logs: Vec<ImportLog_ImportLogEntry>,
}

impl crate::Parse for ImportLog {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_logs = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ImportLog_ImportLogEntry::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ImportLog {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_name,
            m_logs,
        })
    }
}
