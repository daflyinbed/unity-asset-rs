//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxRendererSettings {
    pub motion_vector_generation_mode: i32,
    pub shadow_casting_mode: i32,
    pub receive_shadows: Option<bool>,
    pub ray_tracing_mode: Option<i32>,
    pub reflection_probe_usage: Option<i32>,
    pub light_probe_usage: Option<i32>,
}

impl crate::Parse for VfxRendererSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut motion_vector_generation_mode = None;
        let mut shadow_casting_mode = None;
        let mut receive_shadows = None;
        let mut ray_tracing_mode = None;
        let mut reflection_probe_usage = None;
        let mut light_probe_usage = None;
        motion_vector_generation_mode = Some(rdr.read_i32()?);
        shadow_casting_mode = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            ray_tracing_mode = Some(rdr.read_i32()?);
            receive_shadows = Some(rdr.read_bool()?);
            rdr.align(4)?;
            reflection_probe_usage = Some(rdr.read_i32()?);
            light_probe_usage = Some(rdr.read_i32()?);
        } else {
            receive_shadows = Some(rdr.read_bool()?);
            rdr.align(4)?;
            reflection_probe_usage = Some(rdr.read_i32()?);
            light_probe_usage = Some(rdr.read_i32()?);
        }

        Ok(VfxRendererSettings {
            motion_vector_generation_mode: motion_vector_generation_mode.unwrap(),
            shadow_casting_mode: shadow_casting_mode.unwrap(),
            receive_shadows,
            ray_tracing_mode,
            reflection_probe_usage,
            light_probe_usage,
        })
    }
}

impl crate::ToValue for VfxRendererSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "motionVectorGenerationMode".to_string(),
                self.motion_vector_generation_mode.to_value(),
            )),
            Some((
                "shadowCastingMode".to_string(),
                self.shadow_casting_mode.to_value(),
            )),
            crate::opt_field("receiveShadows", &self.receive_shadows),
            crate::opt_field("rayTracingMode", &self.ray_tracing_mode),
            crate::opt_field("reflectionProbeUsage", &self.reflection_probe_usage),
            crate::opt_field("lightProbeUsage", &self.light_probe_usage),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "motionVectorGenerationMode" => {
                    (self.motion_vector_generation_mode).inspect_value(path, offset, limit)
                }
                "shadowCastingMode" => {
                    (self.shadow_casting_mode).inspect_value(path, offset, limit)
                }
                "receiveShadows" => (self.receive_shadows).inspect_value(path, offset, limit),
                "rayTracingMode" => (self.ray_tracing_mode).inspect_value(path, offset, limit),
                "reflectionProbeUsage" => {
                    (self.reflection_probe_usage).inspect_value(path, offset, limit)
                }
                "lightProbeUsage" => (self.light_probe_usage).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.motion_vector_generation_mode).inspect_value(path, offset, limit))
                .map(|v| ("motionVectorGenerationMode".to_string(), v)),
            ((self.shadow_casting_mode).inspect_value(path, offset, limit))
                .map(|v| ("shadowCastingMode".to_string(), v)),
            ((self.receive_shadows).inspect_value(path, offset, limit))
                .map(|v| ("receiveShadows".to_string(), v)),
            ((self.ray_tracing_mode).inspect_value(path, offset, limit))
                .map(|v| ("rayTracingMode".to_string(), v)),
            ((self.reflection_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("reflectionProbeUsage".to_string(), v)),
            ((self.light_probe_usage).inspect_value(path, offset, limit))
                .map(|v| ("lightProbeUsage".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
