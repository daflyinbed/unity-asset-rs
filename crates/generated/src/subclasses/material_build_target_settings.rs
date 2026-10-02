//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MaterialBuildTargetSettings {
    pub m_build_target: String,
    pub m_texture_width: i32,
    pub m_texture_height: i32,
    pub m_format: i32,
    pub m_loading_behavior: Option<i32>,
}

impl crate::Parse for MaterialBuildTargetSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_texture_width = rdr.read_i32()?;
        let m_texture_height = rdr.read_i32()?;
        let m_format = rdr.read_i32()?;
        let m_loading_behavior = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(MaterialBuildTargetSettings {
            m_build_target,
            m_texture_width,
            m_texture_height,
            m_format,
            m_loading_behavior,
        })
    }
}

impl crate::ToValue for MaterialBuildTargetSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BuildTarget".to_string(), self.m_build_target.to_value())),
            Some((
                "m_TextureWidth".to_string(),
                self.m_texture_width.to_value(),
            )),
            Some((
                "m_TextureHeight".to_string(),
                self.m_texture_height.to_value(),
            )),
            Some(("m_Format".to_string(), self.m_format.to_value())),
            crate::opt_field("m_LoadingBehavior", &self.m_loading_behavior),
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
                "m_TextureWidth" => (self.m_texture_width).inspect_value(path, offset, limit),
                "m_TextureHeight" => (self.m_texture_height).inspect_value(path, offset, limit),
                "m_Format" => (self.m_format).inspect_value(path, offset, limit),
                "m_LoadingBehavior" => (self.m_loading_behavior).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTarget".to_string(), v)),
            ((self.m_texture_width).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureWidth".to_string(), v)),
            ((self.m_texture_height).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureHeight".to_string(), v)),
            ((self.m_format).inspect_value(path, offset, limit))
                .map(|v| ("m_Format".to_string(), v)),
            ((self.m_loading_behavior).inspect_value(path, offset, limit))
                .map(|v| ("m_LoadingBehavior".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
