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
