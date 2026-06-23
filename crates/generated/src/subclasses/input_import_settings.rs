//! 自动生成。请勿手动修改。

use crate::subclasses::substance_value::SubstanceValue;

#[derive(Debug, Clone)]
pub struct InputImportSettings {
    pub name: String,
    pub value: SubstanceValue,
}

impl crate::Parse for InputImportSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let value = SubstanceValue::parse(r, ver)?;

        Ok(InputImportSettings {
            name,
            value,
        })
    }
}
