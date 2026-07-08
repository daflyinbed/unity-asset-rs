//! 自动生成。请勿手动修改。

use crate::subclasses::platform_icon::PlatformIcon;

#[derive(Debug, Clone)]
pub struct BuildTargetIconsNew {
    pub m_build_target: String,
    pub m_icons: Vec<PlatformIcon>,
}

impl crate::Parse for BuildTargetIconsNew {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_icons = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { PlatformIcon::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(BuildTargetIconsNew {
            m_build_target,
            m_icons,
        })
    }
}
