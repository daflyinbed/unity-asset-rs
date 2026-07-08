//! 自动生成。请勿手动修改。

use crate::subclasses::asset_importer_import_error::AssetImporter_ImportError;
use crate::subclasses::import_log_import_log_entry::ImportLog_ImportLogEntry;


/// 版本枚举：AssetImporterLogMLogsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssetImporterLogMLogsVariant {
    Default(Vec<AssetImporter_ImportError>),
    V2022_2(Vec<ImportLog_ImportLogEntry>),
}

impl crate::Parse for AssetImporterLogMLogsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ImportLog_ImportLogEntry::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2022_2(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AssetImporter_ImportError::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AssetImporterLog {
    pub m_name: String,
    pub m_logs: AssetImporterLogMLogsVariant,
}

impl crate::Parse for AssetImporterLog {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_logs = AssetImporterLogMLogsVariant::parse(rdr, ver)?;

        Ok(AssetImporterLog {
            m_name,
            m_logs,
        })
    }
}
