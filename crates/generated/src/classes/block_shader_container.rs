//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct BlockShaderContainer {
    pub m_name: String,
    pub guid: Guid,
    pub dependencies: Vec<PPtr>,
    pub generated_paths: Vec<String>,
    pub blob: Vec<u32>,
    pub state: i32,
}

impl crate::Parse for BlockShaderContainer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let guid = Guid::parse(rdr, ver)?;
        let dependencies = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let generated_paths = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let blob = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let state = rdr.read_i32()?;

        Ok(BlockShaderContainer {
            m_name,
            guid,
            dependencies,
            generated_paths,
            blob,
            state,
        })
    }
}

impl crate::ToValue for BlockShaderContainer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("guid".to_string(), self.guid.to_value())),
            Some(("dependencies".to_string(), self.dependencies.to_value())),
            Some((
                "generatedPaths".to_string(),
                self.generated_paths.to_value(),
            )),
            Some(("blob".to_string(), self.blob.to_value())),
            Some(("state".to_string(), self.state.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "guid" => (self.guid).inspect_value(path, offset, limit),
                "dependencies" => (self.dependencies).inspect_value(path, offset, limit),
                "generatedPaths" => (self.generated_paths).inspect_value(path, offset, limit),
                "blob" => (self.blob).inspect_value(path, offset, limit),
                "state" => (self.state).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.guid).inspect_value(path, offset, limit)).map(|v| ("guid".to_string(), v)),
            ((self.dependencies).inspect_value(path, offset, limit))
                .map(|v| ("dependencies".to_string(), v)),
            ((self.generated_paths).inspect_value(path, offset, limit))
                .map(|v| ("generatedPaths".to_string(), v)),
            ((self.blob).inspect_value(path, offset, limit)).map(|v| ("blob".to_string(), v)),
            ((self.state).inspect_value(path, offset, limit)).map(|v| ("state".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
