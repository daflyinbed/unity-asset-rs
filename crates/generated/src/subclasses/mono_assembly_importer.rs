//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;

#[derive(Debug, Clone)]
pub struct MonoAssemblyImporter {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_file_id_to_recycle_name: Vec<(i32, String)>,
    pub m_old_hash_identity: MdFour,
    pub m_new_hash_identity: MdFour,
    pub m_execution_order: Vec<(String, i32)>,
    pub m_icon_map: Vec<(String, PPtr)>,
}

impl crate::Parse for MonoAssemblyImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_file_id_to_recycle_name = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_old_hash_identity = MdFour::parse(r, ver)?;
        r.align(4)?;
        let m_new_hash_identity = MdFour::parse(r, ver)?;
        r.align(4)?;
        let m_execution_order = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_icon_map = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(MonoAssemblyImporter {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_file_id_to_recycle_name,
            m_old_hash_identity,
            m_new_hash_identity,
            m_execution_order,
            m_icon_map,
        })
    }
}
