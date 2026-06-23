//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct PreloadData {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_assets: Vec<PPtr>,
    pub m_dependencies: Option<Vec<String>>,
    pub m_explicit_data_layout: Option<bool>,
}

impl crate::Parse for PreloadData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_assets = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_dependencies = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_explicit_data_layout = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(PreloadData {
            parent,
            m_name,
            m_assets,
            m_dependencies,
            m_explicit_data_layout,
        })
    }
}
