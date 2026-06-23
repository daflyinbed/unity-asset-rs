//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub dependencies: Vec<String>,
    pub strippable: bool,
}

impl crate::Parse for Module {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let dependencies = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let strippable = r.read_bool()?;

        Ok(Module {
            name,
            dependencies,
            strippable,
        })
    }
}
