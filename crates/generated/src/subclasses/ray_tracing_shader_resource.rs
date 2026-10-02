//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct RayTracingShaderResource {
    pub name: String,
    pub bind_point: i32,
    pub sampler_bind_point: i32,
    pub tex_dimension: i32,
    pub ray_gen_mask: u64,
    pub array_size: Option<i32>,
    pub multisampled: Option<bool>,
}

impl crate::Parse for RayTracingShaderResource {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let bind_point = rdr.read_i32()?;
        let sampler_bind_point = rdr.read_i32()?;
        let tex_dimension = rdr.read_i32()?;
        let ray_gen_mask = rdr.read_u64()?;
        let array_size = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let multisampled = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };

        Ok(RayTracingShaderResource {
            name,
            bind_point,
            sampler_bind_point,
            tex_dimension,
            ray_gen_mask,
            array_size,
            multisampled,
        })
    }
}

impl crate::ToValue for RayTracingShaderResource {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("bindPoint".to_string(), self.bind_point.to_value())),
            Some((
                "samplerBindPoint".to_string(),
                self.sampler_bind_point.to_value(),
            )),
            Some(("texDimension".to_string(), self.tex_dimension.to_value())),
            Some(("rayGenMask".to_string(), self.ray_gen_mask.to_value())),
            crate::opt_field("arraySize", &self.array_size),
            crate::opt_field("multisampled", &self.multisampled),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "bindPoint" => (self.bind_point).inspect_value(path, offset, limit),
                "samplerBindPoint" => (self.sampler_bind_point).inspect_value(path, offset, limit),
                "texDimension" => (self.tex_dimension).inspect_value(path, offset, limit),
                "rayGenMask" => (self.ray_gen_mask).inspect_value(path, offset, limit),
                "arraySize" => (self.array_size).inspect_value(path, offset, limit),
                "multisampled" => (self.multisampled).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.bind_point).inspect_value(path, offset, limit))
                .map(|v| ("bindPoint".to_string(), v)),
            ((self.sampler_bind_point).inspect_value(path, offset, limit))
                .map(|v| ("samplerBindPoint".to_string(), v)),
            ((self.tex_dimension).inspect_value(path, offset, limit))
                .map(|v| ("texDimension".to_string(), v)),
            ((self.ray_gen_mask).inspect_value(path, offset, limit))
                .map(|v| ("rayGenMask".to_string(), v)),
            ((self.array_size).inspect_value(path, offset, limit))
                .map(|v| ("arraySize".to_string(), v)),
            ((self.multisampled).inspect_value(path, offset, limit))
                .map(|v| ("multisampled".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
