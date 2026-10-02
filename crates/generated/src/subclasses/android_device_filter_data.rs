//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AndroidDeviceFilterData {
    pub vendor_name: String,
    pub device_name: String,
    pub brand_name: String,
    pub product_name: String,
    pub android_os_version_string: String,
    pub vulkan_api_version_string: String,
    pub driver_version_string: String,
}

impl crate::Parse for AndroidDeviceFilterData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let vendor_name = rdr.read_aligned_string()?;
        let device_name = rdr.read_aligned_string()?;
        let brand_name = rdr.read_aligned_string()?;
        let product_name = rdr.read_aligned_string()?;
        let android_os_version_string = rdr.read_aligned_string()?;
        let vulkan_api_version_string = rdr.read_aligned_string()?;
        let driver_version_string = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(AndroidDeviceFilterData {
            vendor_name,
            device_name,
            brand_name,
            product_name,
            android_os_version_string,
            vulkan_api_version_string,
            driver_version_string,
        })
    }
}

impl crate::ToValue for AndroidDeviceFilterData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("vendorName".to_string(), self.vendor_name.to_value())),
            Some(("deviceName".to_string(), self.device_name.to_value())),
            Some(("brandName".to_string(), self.brand_name.to_value())),
            Some(("productName".to_string(), self.product_name.to_value())),
            Some((
                "androidOsVersionString".to_string(),
                self.android_os_version_string.to_value(),
            )),
            Some((
                "vulkanApiVersionString".to_string(),
                self.vulkan_api_version_string.to_value(),
            )),
            Some((
                "driverVersionString".to_string(),
                self.driver_version_string.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "vendorName" => (self.vendor_name).inspect_value(path, offset, limit),
                "deviceName" => (self.device_name).inspect_value(path, offset, limit),
                "brandName" => (self.brand_name).inspect_value(path, offset, limit),
                "productName" => (self.product_name).inspect_value(path, offset, limit),
                "androidOsVersionString" => {
                    (self.android_os_version_string).inspect_value(path, offset, limit)
                }
                "vulkanApiVersionString" => {
                    (self.vulkan_api_version_string).inspect_value(path, offset, limit)
                }
                "driverVersionString" => {
                    (self.driver_version_string).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.vendor_name).inspect_value(path, offset, limit))
                .map(|v| ("vendorName".to_string(), v)),
            ((self.device_name).inspect_value(path, offset, limit))
                .map(|v| ("deviceName".to_string(), v)),
            ((self.brand_name).inspect_value(path, offset, limit))
                .map(|v| ("brandName".to_string(), v)),
            ((self.product_name).inspect_value(path, offset, limit))
                .map(|v| ("productName".to_string(), v)),
            ((self.android_os_version_string).inspect_value(path, offset, limit))
                .map(|v| ("androidOsVersionString".to_string(), v)),
            ((self.vulkan_api_version_string).inspect_value(path, offset, limit))
                .map(|v| ("vulkanApiVersionString".to_string(), v)),
            ((self.driver_version_string).inspect_value(path, offset, limit))
                .map(|v| ("driverVersionString".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
