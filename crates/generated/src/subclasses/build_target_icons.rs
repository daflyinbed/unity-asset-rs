//! 自动生成。请勿手动修改。

use crate::subclasses::icon_with_size::IconWithSize;

#[derive(Debug, Clone)]
pub struct BuildTargetIcons {
    pub m_build_target: String,
    pub m_icons: Vec<IconWithSize>,
}

impl crate::Parse for BuildTargetIcons {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_icons = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { IconWithSize::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(BuildTargetIcons {
            m_build_target,
            m_icons,
        })
    }
}
