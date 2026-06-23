//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct BlobObject {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_blob_type_hash: u64,
    pub m_blob_data: Vec<u8>,
    pub m_nested_blob_object_reference_offsets: Vec<u32>,
    pub m_nested_blob_object_references: Vec<PPtr>,
}

impl crate::Parse for BlobObject {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_blob_type_hash = r.read_u64()?;
        let m_blob_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_nested_blob_object_reference_offsets = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_nested_blob_object_references = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(BlobObject {
            parent,
            m_name,
            m_blob_type_hash,
            m_blob_data,
            m_nested_blob_object_reference_offsets,
            m_nested_blob_object_references,
        })
    }
}
