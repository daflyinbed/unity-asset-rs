//! 自动生成。请勿手动修改。

use crate::subclasses::module::Module;

#[derive(Debug, Clone)]
pub struct PlatformModuleSetup {
    pub modules: Vec<Module>,
}

impl crate::Parse for PlatformModuleSetup {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let modules = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Module::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(PlatformModuleSetup { modules })
    }
}

impl crate::ToValue for PlatformModuleSetup {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("modules".to_string(), self.modules.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "modules" => (self.modules).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.modules).inspect_value(path, offset, limit)).map(|v| ("modules".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
