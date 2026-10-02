//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub dependencies: Vec<String>,
    pub strippable: bool,
    pub controlled_by_builtin_package: Option<bool>,
}

impl crate::Parse for Module {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let dependencies = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let strippable = rdr.read_bool()?;
        let controlled_by_builtin_package = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(rdr.read_bool()?)
        } else {
            None
        };

        Ok(Module {
            name,
            dependencies,
            strippable,
            controlled_by_builtin_package,
        })
    }
}

impl crate::ToValue for Module {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("dependencies".to_string(), self.dependencies.to_value())),
            Some(("strippable".to_string(), self.strippable.to_value())),
            crate::opt_field(
                "controlledByBuiltinPackage",
                &self.controlled_by_builtin_package,
            ),
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
                "dependencies" => (self.dependencies).inspect_value(path, offset, limit),
                "strippable" => (self.strippable).inspect_value(path, offset, limit),
                "controlledByBuiltinPackage" => {
                    (self.controlled_by_builtin_package).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.dependencies).inspect_value(path, offset, limit))
                .map(|v| ("dependencies".to_string(), v)),
            ((self.strippable).inspect_value(path, offset, limit))
                .map(|v| ("strippable".to_string(), v)),
            ((self.controlled_by_builtin_package).inspect_value(path, offset, limit))
                .map(|v| ("controlledByBuiltinPackage".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
