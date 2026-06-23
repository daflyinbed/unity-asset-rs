//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SceneRoots {
    pub m_object_hide_flags: u32,
    pub m_roots: Vec<PPtr>,
}

impl crate::Parse for SceneRoots {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_roots = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(SceneRoots {
            m_object_hide_flags,
            m_roots,
        })
    }
}
