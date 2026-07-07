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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_scripts = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_script_hashes = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = Hash128::parse(r, ver)?; let b = Hash128::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_assembly_names = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_runtime_class_hashes = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = Hash128::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_assembly_types = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(MonoManager {
            m_scripts,
            m_script_hashes,
            m_assembly_names,
            m_runtime_class_hashes,
            m_assembly_types,
        })
    }
}
