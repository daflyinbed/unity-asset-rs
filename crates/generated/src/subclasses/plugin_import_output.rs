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
        let plugin_type = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let dll_type = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let scripting_runtime_version = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(PluginImportOutput {
            plugin_type,
            dll_type,
            scripting_runtime_version,
        })
    }
}
