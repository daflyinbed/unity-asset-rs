//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PlatformSettingsData_Editor {
    pub enabled: Option<bool>,
    pub settings: Vec<(String, String)>,
}

impl crate::Parse for PlatformSettingsData_Editor {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut settings = None;
        if *ver >= crate::UnityVersion::new(2021, 1, 5) {
        } else if *ver >= crate::UnityVersion::new(2021, 1, 1) {
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 4, 38) {
        } else if *ver >= crate::UnityVersion::new(2019, 4, 27) {
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(2019, 1, 4) {
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
        } else if *ver >= crate::UnityVersion::new(2018, 3, 10) {
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
        } else {
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        settings = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = rdr.read_aligned_string()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );

        Ok(PlatformSettingsData_Editor {
            enabled,
            settings: settings.unwrap(),
        })
    }
}

impl crate::ToValue for PlatformSettingsData_Editor {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("enabled", &self.enabled),
            Some(("settings".to_string(), self.settings.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "settings" => (self.settings).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.settings).inspect_value(path, offset, limit))
                .map(|v| ("settings".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
