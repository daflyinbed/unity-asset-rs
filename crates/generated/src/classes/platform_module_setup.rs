//! 自动生成。请勿手动修改。

use crate::subclasses::module::Module;
use super::object::Object;

#[derive(Debug, Clone)]
pub struct PlatformModuleSetup {
    pub parent: Object,
    pub modules: Vec<Module>,
}

impl crate::Parse for PlatformModuleSetup {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Object::parse(r, ver)?;
        let modules = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Module::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(PlatformModuleSetup {
            parent,
            modules,
        })
    }
}
