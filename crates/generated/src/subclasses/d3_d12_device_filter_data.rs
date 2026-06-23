//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct D3D12DeviceFilterData {
    pub vendor_name: String,
    pub device_name: String,
    pub driver_version_comparator: u32,
    pub driver_version: String,
    pub feature_level_comparator: u32,
    pub feature_level: String,
    pub graphics_memory_comparator: u32,
    pub graphics_memory: String,
    pub processor_count_comparator: u32,
    pub processor_count: String,
    pub device_type: u32,
}

impl crate::Parse for D3D12DeviceFilterData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let vendor_name = r.read_aligned_string()?;
        r.align(4)?;
        let device_name = r.read_aligned_string()?;
        r.align(4)?;
        let driver_version_comparator = r.read_u32()?;
        let driver_version = r.read_aligned_string()?;
        r.align(4)?;
        let feature_level_comparator = r.read_u32()?;
        let feature_level = r.read_aligned_string()?;
        r.align(4)?;
        let graphics_memory_comparator = r.read_u32()?;
        let graphics_memory = r.read_aligned_string()?;
        r.align(4)?;
        let processor_count_comparator = r.read_u32()?;
        let processor_count = r.read_aligned_string()?;
        r.align(4)?;
        let device_type = r.read_u32()?;
        r.align(4)?;

        Ok(D3D12DeviceFilterData {
            vendor_name,
            device_name,
            driver_version_comparator,
            driver_version,
            feature_level_comparator,
            feature_level,
            graphics_memory_comparator,
            graphics_memory,
            processor_count_comparator,
            processor_count,
            device_type,
        })
    }
}
