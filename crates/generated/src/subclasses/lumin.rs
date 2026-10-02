//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Lumin {
    pub depth_format: i32,
    pub frame_timing: i32,
    pub enable_gl_cache: bool,
    pub gl_cache_max_blob_size: u32,
    pub gl_cache_max_file_size: u32,
}

impl crate::Parse for Lumin {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let depth_format = rdr.read_i32()?;
        let frame_timing = rdr.read_i32()?;
        let enable_gl_cache = rdr.read_bool()?;
        rdr.align(4)?;
        let gl_cache_max_blob_size = rdr.read_u32()?;
        let gl_cache_max_file_size = rdr.read_u32()?;
        rdr.align(4)?;

        Ok(Lumin {
            depth_format,
            frame_timing,
            enable_gl_cache,
            gl_cache_max_blob_size,
            gl_cache_max_file_size,
        })
    }
}

impl crate::ToValue for Lumin {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("depthFormat".to_string(), self.depth_format.to_value())),
            Some(("frameTiming".to_string(), self.frame_timing.to_value())),
            Some(("enableGLCache".to_string(), self.enable_gl_cache.to_value())),
            Some((
                "glCacheMaxBlobSize".to_string(),
                self.gl_cache_max_blob_size.to_value(),
            )),
            Some((
                "glCacheMaxFileSize".to_string(),
                self.gl_cache_max_file_size.to_value(),
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
                "depthFormat" => (self.depth_format).inspect_value(path, offset, limit),
                "frameTiming" => (self.frame_timing).inspect_value(path, offset, limit),
                "enableGLCache" => (self.enable_gl_cache).inspect_value(path, offset, limit),
                "glCacheMaxBlobSize" => {
                    (self.gl_cache_max_blob_size).inspect_value(path, offset, limit)
                }
                "glCacheMaxFileSize" => {
                    (self.gl_cache_max_file_size).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.depth_format).inspect_value(path, offset, limit))
                .map(|v| ("depthFormat".to_string(), v)),
            ((self.frame_timing).inspect_value(path, offset, limit))
                .map(|v| ("frameTiming".to_string(), v)),
            ((self.enable_gl_cache).inspect_value(path, offset, limit))
                .map(|v| ("enableGLCache".to_string(), v)),
            ((self.gl_cache_max_blob_size).inspect_value(path, offset, limit))
                .map(|v| ("glCacheMaxBlobSize".to_string(), v)),
            ((self.gl_cache_max_file_size).inspect_value(path, offset, limit))
                .map(|v| ("glCacheMaxFileSize".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
