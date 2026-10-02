//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGraphicsJobMode {
    pub m_build_target: String,
    pub m_graphics_job_mode: i32,
}

impl crate::Parse for BuildTargetGraphicsJobMode {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_graphics_job_mode = rdr.read_i32()?;

        Ok(BuildTargetGraphicsJobMode {
            m_build_target,
            m_graphics_job_mode,
        })
    }
}

impl crate::ToValue for BuildTargetGraphicsJobMode {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BuildTarget".to_string(), self.m_build_target.to_value())),
            Some((
                "m_GraphicsJobMode".to_string(),
                self.m_graphics_job_mode.to_value(),
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
                "m_BuildTarget" => (self.m_build_target).inspect_value(path, offset, limit),
                "m_GraphicsJobMode" => {
                    (self.m_graphics_job_mode).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTarget".to_string(), v)),
            ((self.m_graphics_job_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_GraphicsJobMode".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
