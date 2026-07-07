//! 自动生成。请勿手动修改。

use crate::subclasses::import_log_import_log_entry::ImportLog_ImportLogEntry;

#[derive(Debug, Clone)]
pub struct AssetImporterLog {
    pub m_name: String,
    pub m_logs: Vec<ImportLog_ImportLogEntry>,
}

impl crate::Parse for AssetImporterLog {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_logs = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ImportLog_ImportLogEntry::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(AssetImporterLog {
            m_name,
            m_logs,
        })
    }
}
