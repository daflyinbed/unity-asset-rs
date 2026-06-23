//! 自动生成。请勿手动修改。

use super::named_object::NamedObject;
use super::text_asset::TextAsset;

#[derive(Debug, Clone)]
pub enum CgProgramParent {
    TextAsset(TextAsset),
    NamedObject(NamedObject),
}

#[derive(Debug, Clone)]
pub struct CgProgram {
    pub parent: CgProgramParent,
    pub m_name: String,
    pub m_script: Option<String>,
    pub m_path_name: Option<String>,
}

impl crate::Parse for CgProgram {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            CgProgramParent::TextAsset(TextAsset::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            CgProgramParent::NamedObject(NamedObject::parse(r, ver)?)
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            CgProgramParent::TextAsset(TextAsset::parse(r, ver)?)
        } else {
            panic!("无匹配的父类分支：version = {:?}", ver)
        };
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_script = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_path_name = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(CgProgram {
            parent,
            m_name,
            m_script,
            m_path_name,
        })
    }
}
