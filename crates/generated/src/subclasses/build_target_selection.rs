//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetSelection {
    pub platform: i32,
    pub sub_target: i32,
    pub extended_platform: i32,
}

impl crate::Parse for BuildTargetSelection {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let platform = rdr.read_i32()?;
        let sub_target = rdr.read_i32()?;
        let extended_platform = rdr.read_i32()?;

        Ok(BuildTargetSelection {
            platform,
            sub_target,
            extended_platform,
        })
    }
}

impl crate::ToValue for BuildTargetSelection {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("platform".to_string(), self.platform.to_value())),
            Some(("subTarget".to_string(), self.sub_target.to_value())),
            Some((
                "extendedPlatform".to_string(),
                self.extended_platform.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "platform" => (self.platform).inspect_value(path, offset, limit),
                "subTarget" => (self.sub_target).inspect_value(path, offset, limit),
                "extendedPlatform" => (self.extended_platform).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.platform).inspect_value(path, offset, limit))
                .map(|v| ("platform".to_string(), v)),
            ((self.sub_target).inspect_value(path, offset, limit))
                .map(|v| ("subTarget".to_string(), v)),
            ((self.extended_platform).inspect_value(path, offset, limit))
                .map(|v| ("extendedPlatform".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
