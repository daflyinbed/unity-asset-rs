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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let vendor_name = r.read_aligned_string()?;
        r.align(4)?;
        let device_name = r.read_aligned_string()?;
        r.align(4)?;
        let brand_name = r.read_aligned_string()?;
        r.align(4)?;
        let product_name = r.read_aligned_string()?;
        r.align(4)?;
        let android_os_version_string = r.read_aligned_string()?;
        r.align(4)?;
        let vulkan_api_version_string = r.read_aligned_string()?;
        r.align(4)?;
        let driver_version_string = r.read_aligned_string()?;
        r.align(4)?;

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
