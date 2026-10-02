//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct BlobObject {
    pub m_name: String,
    pub m_blob_type_hash: u64,
    pub m_blob_data: Vec<u8>,
    pub m_nested_blob_object_reference_offsets: Vec<u32>,
    pub m_nested_blob_object_references: Vec<PPtr>,
}

impl crate::Parse for BlobObject {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_blob_type_hash = rdr.read_u64()?;
        let m_blob_data = ({
            let n = rdr.read_i32()? as usize;
            (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_nested_blob_object_reference_offsets = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_nested_blob_object_references = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(BlobObject {
            m_name,
            m_blob_type_hash,
            m_blob_data,
            m_nested_blob_object_reference_offsets,
            m_nested_blob_object_references,
        })
    }
}

impl crate::ToValue for BlobObject {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_BlobTypeHash".to_string(),
                self.m_blob_type_hash.to_value(),
            )),
            Some(("m_BlobData".to_string(), self.m_blob_data.to_value())),
            Some((
                "m_NestedBlobObjectReferenceOffsets".to_string(),
                self.m_nested_blob_object_reference_offsets.to_value(),
            )),
            Some((
                "m_NestedBlobObjectReferences".to_string(),
                self.m_nested_blob_object_references.to_value(),
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
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_BlobTypeHash" => (self.m_blob_type_hash).inspect_value(path, offset, limit),
                "m_BlobData" => (self.m_blob_data).inspect_value(path, offset, limit),
                "m_NestedBlobObjectReferenceOffsets" => {
                    (self.m_nested_blob_object_reference_offsets).inspect_value(path, offset, limit)
                }
                "m_NestedBlobObjectReferences" => {
                    (self.m_nested_blob_object_references).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_blob_type_hash).inspect_value(path, offset, limit))
                .map(|v| ("m_BlobTypeHash".to_string(), v)),
            ((self.m_blob_data).inspect_value(path, offset, limit))
                .map(|v| ("m_BlobData".to_string(), v)),
            ((self.m_nested_blob_object_reference_offsets).inspect_value(path, offset, limit))
                .map(|v| ("m_NestedBlobObjectReferenceOffsets".to_string(), v)),
            ((self.m_nested_blob_object_references).inspect_value(path, offset, limit))
                .map(|v| ("m_NestedBlobObjectReferences".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
