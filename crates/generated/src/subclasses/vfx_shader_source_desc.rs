//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxShaderSourceDesc {
    pub compute: bool,
    pub name: String,
    pub source: String,
}

impl crate::Parse for VfxShaderSourceDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let compute = rdr.read_bool()?;
        rdr.align(4)?;
        let name = rdr.read_aligned_string()?;
        let source = rdr.read_aligned_string()?;

        Ok(VfxShaderSourceDesc {
            compute,
            name,
            source,
        })
    }
}

impl crate::ToValue for VfxShaderSourceDesc {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("compute".to_string(), self.compute.to_value())),
            Some(("name".to_string(), self.name.to_value())),
            Some(("source".to_string(), self.source.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "compute" => (self.compute).inspect_value(path, offset, limit),
                "name" => (self.name).inspect_value(path, offset, limit),
                "source" => (self.source).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.compute).inspect_value(path, offset, limit)).map(|v| ("compute".to_string(), v)),
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.source).inspect_value(path, offset, limit)).map(|v| ("source".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
