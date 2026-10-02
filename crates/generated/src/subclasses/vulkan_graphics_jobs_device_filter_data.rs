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

impl crate::ToValue for VulkanGraphicsJobsDeviceFilterData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("preferredMode".to_string(), self.preferred_mode.to_value())),
            Some(("filter".to_string(), self.filter.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "preferredMode" => (self.preferred_mode).inspect_value(path, offset, limit),
                "filter" => (self.filter).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.preferred_mode).inspect_value(path, offset, limit))
                .map(|v| ("preferredMode".to_string(), v)),
            ((self.filter).inspect_value(path, offset, limit)).map(|v| ("filter".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
