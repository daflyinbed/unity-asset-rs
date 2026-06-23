//! 自动生成。请勿手动修改。

use crate::subclasses::module::Module;

#[derive(Debug, Clone)]
pub struct PlatformModuleSetup {
    pub m_object_hide_flags: u32,
    pub modules: Vec<Module>,
}

impl crate::Parse for PlatformModuleSetup {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let modules = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Module::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(PlatformModuleSetup {
            m_object_hide_flags,
            modules,
        })
    }
}
