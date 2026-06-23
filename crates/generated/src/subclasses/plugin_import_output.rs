//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PluginImportOutput {
    pub plugin_type: i32,
}

impl crate::Parse for PluginImportOutput {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let plugin_type = r.read_i32()?;

        Ok(PluginImportOutput {
            plugin_type,
        })
    }
}
