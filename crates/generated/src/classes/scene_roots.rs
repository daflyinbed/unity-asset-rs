//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SceneRoots {
    pub m_roots: Vec<PPtr>,
}

impl crate::Parse for SceneRoots {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_roots = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(SceneRoots { m_roots })
    }
}
