//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PluginImportOutput {
    pub plugin_type: Option<i32>,
    pub dll_type: Option<i32>,
    pub scripting_runtime_version: Option<i32>,
}

impl crate::Parse for PluginImportOutput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut plugin_type = None;
        let mut dll_type = None;
        let mut scripting_runtime_version = None;
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            dll_type = Some(rdr.read_i32()?);
            scripting_runtime_version = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
            dll_type = Some(rdr.read_i32()?);
        } else {
            plugin_type = Some(rdr.read_i32()?);
        }

        Ok(PluginImportOutput {
            plugin_type,
            dll_type,
            scripting_runtime_version,
        })
    }
}
