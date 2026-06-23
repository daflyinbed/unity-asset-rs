//! 自动生成。请勿手动修改。

use crate::subclasses::platform_icon::PlatformIcon;

#[derive(Debug, Clone)]
pub struct BuildTargetIconsNew {
    pub m_build_target: String,
    pub m_icons: Vec<PlatformIcon>,
}

impl crate::Parse for BuildTargetIconsNew {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = r.read_aligned_string()?;
        r.align(4)?;
        let m_icons = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { PlatformIcon::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(BuildTargetIconsNew {
            m_build_target,
            m_icons,
        })
    }
}
