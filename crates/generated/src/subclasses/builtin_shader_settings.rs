//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct BuiltinShaderSettings {
    pub m_mode: i32,
    pub m_shader: PPtr,
}

impl crate::Parse for BuiltinShaderSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_mode = rdr.read_i32()?;
        let m_shader = crate::PPtr::parse(rdr, ver)?;

        Ok(BuiltinShaderSettings {
            m_mode,
            m_shader,
        })
    }
}
