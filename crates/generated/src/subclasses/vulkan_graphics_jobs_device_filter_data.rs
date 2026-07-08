//! 自动生成。请勿手动修改。

use crate::subclasses::android_device_filter_data::AndroidDeviceFilterData;

#[derive(Debug, Clone)]
pub struct VulkanGraphicsJobsDeviceFilterData {
    pub preferred_mode: u32,
    pub filter: AndroidDeviceFilterData,
}

impl crate::Parse for VulkanGraphicsJobsDeviceFilterData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let preferred_mode = rdr.read_u32()?;
        let filter = AndroidDeviceFilterData::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(VulkanGraphicsJobsDeviceFilterData {
            preferred_mode,
            filter,
        })
    }
}
