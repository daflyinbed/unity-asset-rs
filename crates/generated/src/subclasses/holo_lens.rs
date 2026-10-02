//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct HoloLens {
    pub depth_format: i32,
    pub depth_buffer_sharing_enabled: Option<bool>,
}

impl crate::Parse for HoloLens {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let depth_format = rdr.read_i32()?;
        let depth_buffer_sharing_enabled = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };

        Ok(HoloLens {
            depth_format,
            depth_buffer_sharing_enabled,
        })
    }
}

impl crate::ToValue for HoloLens {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("depthFormat".to_string(), self.depth_format.to_value())),
            crate::opt_field(
                "depthBufferSharingEnabled",
                &self.depth_buffer_sharing_enabled,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "depthFormat" => (self.depth_format).inspect_value(path, offset, limit),
                "depthBufferSharingEnabled" => {
                    (self.depth_buffer_sharing_enabled).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.depth_format).inspect_value(path, offset, limit))
                .map(|v| ("depthFormat".to_string(), v)),
            ((self.depth_buffer_sharing_enabled).inspect_value(path, offset, limit))
                .map(|v| ("depthBufferSharingEnabled".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
