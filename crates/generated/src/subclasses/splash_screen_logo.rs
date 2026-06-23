//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SplashScreenLogo {
    pub logo: PPtr,
    pub duration: f32,
}

impl crate::Parse for SplashScreenLogo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let logo = crate::PPtr::parse(r, ver)?;
        let duration = r.read_f32()?;
        r.align(4)?;

        Ok(SplashScreenLogo {
            logo,
            duration,
        })
    }
}
