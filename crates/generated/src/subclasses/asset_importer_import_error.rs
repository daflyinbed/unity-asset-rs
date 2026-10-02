//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AssetImporter_ImportError {
    pub error: String,
    pub mode: i32,
    pub line: i32,
    pub file: String,
    pub object: PPtr,
}

impl crate::Parse for AssetImporter_ImportError {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let error = rdr.read_aligned_string()?;
        let mode = rdr.read_i32()?;
        let line = rdr.read_i32()?;
        let file = rdr.read_aligned_string()?;
        let object = crate::PPtr::parse(rdr, ver)?;

        Ok(AssetImporter_ImportError {
            error,
            mode,
            line,
            file,
            object,
        })
    }
}

impl crate::ToValue for AssetImporter_ImportError {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("error".to_string(), self.error.to_value())),
            Some(("mode".to_string(), self.mode.to_value())),
            Some(("line".to_string(), self.line.to_value())),
            Some(("file".to_string(), self.file.to_value())),
            Some(("object".to_string(), self.object.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "error" => (self.error).inspect_value(path, offset, limit),
                "mode" => (self.mode).inspect_value(path, offset, limit),
                "line" => (self.line).inspect_value(path, offset, limit),
                "file" => (self.file).inspect_value(path, offset, limit),
                "object" => (self.object).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.error).inspect_value(path, offset, limit)).map(|v| ("error".to_string(), v)),
            ((self.mode).inspect_value(path, offset, limit)).map(|v| ("mode".to_string(), v)),
            ((self.line).inspect_value(path, offset, limit)).map(|v| ("line".to_string(), v)),
            ((self.file).inspect_value(path, offset, limit)).map(|v| ("file".to_string(), v)),
            ((self.object).inspect_value(path, offset, limit)).map(|v| ("object".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
