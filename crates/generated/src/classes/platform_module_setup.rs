//! 自动生成。请勿手动修改。

use crate::subclasses::module::Module;

#[derive(Debug, Clone)]
pub struct PlatformModuleSetup {
    pub modules: Vec<Module>,
}

impl crate::Parse for PlatformModuleSetup {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let modules = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Module::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(PlatformModuleSetup {
            modules,
        })
    }
}
