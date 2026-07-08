//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct MonoManager {
    pub m_scripts: Vec<PPtr>,
    pub m_script_hashes: Option<Vec<(Hash128, Hash128)>>,
    pub m_assembly_names: Option<Vec<String>>,
    pub m_runtime_class_hashes: Option<Vec<(i32, Hash128)>>,
    pub m_assembly_types: Option<Vec<i32>>,
}

impl crate::Parse for MonoManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_scripts = None;
        let mut m_script_hashes = None;
        let mut m_assembly_names = None;
        let mut m_runtime_class_hashes = None;
        let mut m_assembly_types = None;
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            m_script_hashes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = Hash128::parse(rdr, ver)?; let b = Hash128::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_runtime_class_hashes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i32()?; let b = Hash128::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
        } else {
        }
            m_scripts = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_assembly_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_assembly_types = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
            m_assembly_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        }

        Ok(MonoManager {
            m_scripts: m_scripts.unwrap(),
            m_script_hashes,
            m_assembly_names,
            m_runtime_class_hashes,
            m_assembly_types,
        })
    }
}
