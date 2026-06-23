//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct GraphicsSettings {
    pub m_object_hide_flags: u32,
    pub m_always_included_shaders: Vec<PPtr>,
}

impl crate::Parse for GraphicsSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_always_included_shaders = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(GraphicsSettings {
            m_object_hide_flags,
            m_always_included_shaders,
        })
    }
}
