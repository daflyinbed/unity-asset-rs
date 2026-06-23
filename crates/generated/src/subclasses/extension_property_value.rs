//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ExtensionPropertyValue {
    pub plugin_name: String,
    pub extension_name: String,
    pub property_name: String,
    pub property_value: f32,
}

impl crate::Parse for ExtensionPropertyValue {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let plugin_name = r.read_aligned_string()?;
        r.align(4)?;
        let extension_name = r.read_aligned_string()?;
        r.align(4)?;
        let property_name = r.read_aligned_string()?;
        r.align(4)?;
        let property_value = r.read_f32()?;

        Ok(ExtensionPropertyValue {
            plugin_name,
            extension_name,
            property_name,
            property_value,
        })
    }
}
