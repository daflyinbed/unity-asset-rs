//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct SplashScreenLogo {
    pub logo: PPtr,
    pub duration: f32,
}

impl crate::Parse for SplashScreenLogo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let logo = crate::PPtr::parse(rdr, ver)?;
        let duration = rdr.read_f32()?;
        rdr.align(4)?;

        Ok(SplashScreenLogo { logo, duration })
    }
}

impl crate::ToValue for SplashScreenLogo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("logo".to_string(), self.logo.to_value())),
            Some(("duration".to_string(), self.duration.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "logo" => (self.logo).inspect_value(path, offset, limit),
                "duration" => (self.duration).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.logo).inspect_value(path, offset, limit)).map(|v| ("logo".to_string(), v)),
            ((self.duration).inspect_value(path, offset, limit))
                .map(|v| ("duration".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
