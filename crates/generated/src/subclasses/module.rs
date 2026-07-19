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
