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

impl crate::ToValue for ExtensionPropertyValue {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("pluginName".to_string(), self.plugin_name.to_value())),
            Some(("extensionName".to_string(), self.extension_name.to_value())),
            Some(("propertyName".to_string(), self.property_name.to_value())),
            Some(("propertyValue".to_string(), self.property_value.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "pluginName" => (self.plugin_name).inspect_value(path, offset, limit),
                "extensionName" => (self.extension_name).inspect_value(path, offset, limit),
                "propertyName" => (self.property_name).inspect_value(path, offset, limit),
                "propertyValue" => (self.property_value).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.plugin_name).inspect_value(path, offset, limit))
                .map(|v| ("pluginName".to_string(), v)),
            ((self.extension_name).inspect_value(path, offset, limit))
                .map(|v| ("extensionName".to_string(), v)),
            ((self.property_name).inspect_value(path, offset, limit))
                .map(|v| ("propertyName".to_string(), v)),
            ((self.property_value).inspect_value(path, offset, limit))
                .map(|v| ("propertyValue".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
