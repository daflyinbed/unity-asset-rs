//! 自动生成。请勿手动修改。

use crate::subclasses::error::Error;

#[derive(Debug, Clone)]
pub struct BlockShaderErrors {
    pub m_name: String,
    pub errors: Vec<Error>,
    pub file_path: String,
}

impl crate::Parse for BlockShaderErrors {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let errors = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Error::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let file_path = rdr.read_aligned_string()?;

        Ok(BlockShaderErrors {
            m_name,
            errors,
            file_path,
        })
    }
}

impl crate::ToValue for BlockShaderErrors {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("errors".to_string(), self.errors.to_value())),
            Some(("filePath".to_string(), self.file_path.to_value())),
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
                "errors" => (self.errors).inspect_value(path, offset, limit),
                "filePath" => (self.file_path).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.errors).inspect_value(path, offset, limit)).map(|v| ("errors".to_string(), v)),
            ((self.file_path).inspect_value(path, offset, limit))
                .map(|v| ("filePath".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
