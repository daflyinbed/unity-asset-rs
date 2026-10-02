//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AndroidBanner {
    pub width: i32,
    pub height: i32,
    pub banner: PPtr,
}

impl crate::Parse for AndroidBanner {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let width = rdr.read_i32()?;
        let height = rdr.read_i32()?;
        let banner = crate::PPtr::parse(rdr, ver)?;

        Ok(AndroidBanner {
            width,
            height,
            banner,
        })
    }
}

impl crate::ToValue for AndroidBanner {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("width".to_string(), self.width.to_value())),
            Some(("height".to_string(), self.height.to_value())),
            Some(("banner".to_string(), self.banner.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "width" => (self.width).inspect_value(path, offset, limit),
                "height" => (self.height).inspect_value(path, offset, limit),
                "banner" => (self.banner).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.width).inspect_value(path, offset, limit)).map(|v| ("width".to_string(), v)),
            ((self.height).inspect_value(path, offset, limit)).map(|v| ("height".to_string(), v)),
            ((self.banner).inspect_value(path, offset, limit)).map(|v| ("banner".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
