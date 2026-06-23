//! 自动生成。请勿手动修改。

use crate::subclasses::asset_importer_import_error::AssetImporter_ImportError;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct AssetImporterLog {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_logs: Vec<AssetImporter_ImportError>,
}

impl crate::Parse for AssetImporterLog {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_logs = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AssetImporter_ImportError::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(AssetImporterLog {
            parent,
            m_name,
            m_logs,
        })
    }
}
