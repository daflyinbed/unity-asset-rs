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
        let m_icons = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| PlatformIcon::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(BuildTargetIconsNew {
            m_build_target,
            m_icons,
        })
    }
}

impl crate::ToValue for BuildTargetIconsNew {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BuildTarget".to_string(), self.m_build_target.to_value())),
            Some(("m_Icons".to_string(), self.m_icons.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BuildTarget" => (self.m_build_target).inspect_value(path, offset, limit),
                "m_Icons" => (self.m_icons).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTarget".to_string(), v)),
            ((self.m_icons).inspect_value(path, offset, limit)).map(|v| ("m_Icons".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
