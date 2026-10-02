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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let vendor_name = rdr.read_aligned_string()?;
        let device_name = rdr.read_aligned_string()?;
        let driver_version_comparator = rdr.read_u32()?;
        let driver_version = rdr.read_aligned_string()?;
        let feature_level_comparator = rdr.read_u32()?;
        let feature_level = rdr.read_aligned_string()?;
        let graphics_memory_comparator = rdr.read_u32()?;
        let graphics_memory = rdr.read_aligned_string()?;
        let processor_count_comparator = rdr.read_u32()?;
        let processor_count = rdr.read_aligned_string()?;
        let device_type = rdr.read_u32()?;
        rdr.align(4)?;

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

impl crate::ToValue for D3D12DeviceFilterData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("vendorName".to_string(), self.vendor_name.to_value())),
            Some(("deviceName".to_string(), self.device_name.to_value())),
            Some((
                "driverVersionComparator".to_string(),
                self.driver_version_comparator.to_value(),
            )),
            Some(("driverVersion".to_string(), self.driver_version.to_value())),
            Some((
                "featureLevelComparator".to_string(),
                self.feature_level_comparator.to_value(),
            )),
            Some(("featureLevel".to_string(), self.feature_level.to_value())),
            Some((
                "graphicsMemoryComparator".to_string(),
                self.graphics_memory_comparator.to_value(),
            )),
            Some((
                "graphicsMemory".to_string(),
                self.graphics_memory.to_value(),
            )),
            Some((
                "processorCountComparator".to_string(),
                self.processor_count_comparator.to_value(),
            )),
            Some((
                "processorCount".to_string(),
                self.processor_count.to_value(),
            )),
            Some(("deviceType".to_string(), self.device_type.to_value())),
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
                "driverVersionComparator" => {
                    (self.driver_version_comparator).inspect_value(path, offset, limit)
                }
                "driverVersion" => (self.driver_version).inspect_value(path, offset, limit),
                "featureLevelComparator" => {
                    (self.feature_level_comparator).inspect_value(path, offset, limit)
                }
                "featureLevel" => (self.feature_level).inspect_value(path, offset, limit),
                "graphicsMemoryComparator" => {
                    (self.graphics_memory_comparator).inspect_value(path, offset, limit)
                }
                "graphicsMemory" => (self.graphics_memory).inspect_value(path, offset, limit),
                "processorCountComparator" => {
                    (self.processor_count_comparator).inspect_value(path, offset, limit)
                }
                "processorCount" => (self.processor_count).inspect_value(path, offset, limit),
                "deviceType" => (self.device_type).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.vendor_name).inspect_value(path, offset, limit))
                .map(|v| ("vendorName".to_string(), v)),
            ((self.device_name).inspect_value(path, offset, limit))
                .map(|v| ("deviceName".to_string(), v)),
            ((self.driver_version_comparator).inspect_value(path, offset, limit))
                .map(|v| ("driverVersionComparator".to_string(), v)),
            ((self.driver_version).inspect_value(path, offset, limit))
                .map(|v| ("driverVersion".to_string(), v)),
            ((self.feature_level_comparator).inspect_value(path, offset, limit))
                .map(|v| ("featureLevelComparator".to_string(), v)),
            ((self.feature_level).inspect_value(path, offset, limit))
                .map(|v| ("featureLevel".to_string(), v)),
            ((self.graphics_memory_comparator).inspect_value(path, offset, limit))
                .map(|v| ("graphicsMemoryComparator".to_string(), v)),
            ((self.graphics_memory).inspect_value(path, offset, limit))
                .map(|v| ("graphicsMemory".to_string(), v)),
            ((self.processor_count_comparator).inspect_value(path, offset, limit))
                .map(|v| ("processorCountComparator".to_string(), v)),
            ((self.processor_count).inspect_value(path, offset, limit))
                .map(|v| ("processorCount".to_string(), v)),
            ((self.device_type).inspect_value(path, offset, limit))
                .map(|v| ("deviceType".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
