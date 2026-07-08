//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct InputImportSettings {
    pub name: String,
    pub alpha_source: i32,
    pub filter_mode: i32,
    pub aniso: i32,
    pub wrap_mode: i32,
}

impl crate::Parse for InputImportSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let alpha_source = rdr.read_i32()?;
        let filter_mode = rdr.read_i32()?;
        let aniso = rdr.read_i32()?;
        let wrap_mode = rdr.read_i32()?;

        Ok(InputImportSettings {
            name,
            alpha_source,
            filter_mode,
            aniso,
            wrap_mode,
        })
    }
}
