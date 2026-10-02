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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ImportLog_ImportLogEntry::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2022_2(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| AssetImporter_ImportError::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AssetImporterLogMLogsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_2(v) => (v).inspect_value(path, offset, limit),
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

        Ok(AssetImporterLog { m_name, m_logs })
    }
}

impl crate::ToValue for AssetImporterLog {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Logs".to_string(), self.m_logs.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Logs" => (self.m_logs).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_logs).inspect_value(path, offset, limit)).map(|v| ("m_Logs".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
