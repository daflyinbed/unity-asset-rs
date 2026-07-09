//! 自动生成。请勿手动修改。

use crate::subclasses::property_name32::PropertyName32;

#[derive(Debug, Clone)]
pub struct ExtensionPropertyValue {
    pub plugin_name: PropertyName32,
    pub extension_name: PropertyName32,
    pub property_name: PropertyName32,
    pub property_value: f32,
}

impl crate::Parse for ExtensionPropertyValue {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let plugin_name = PropertyName32::parse(rdr, ver)?;
        let extension_name = PropertyName32::parse(rdr, ver)?;
        let property_name = PropertyName32::parse(rdr, ver)?;
        let property_value = rdr.read_f32()?;

        Ok(ExtensionPropertyValue {
            plugin_name,
            extension_name,
            property_name,
            property_value,
        })
    }
}
