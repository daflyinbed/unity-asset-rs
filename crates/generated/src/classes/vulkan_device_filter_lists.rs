//! 自动生成。请勿手动修改。

use crate::subclasses::android_device_filter_data::AndroidDeviceFilterData;
use crate::subclasses::vulkan_graphics_jobs_device_filter_data::VulkanGraphicsJobsDeviceFilterData;

#[derive(Debug, Clone)]
pub struct VulkanDeviceFilterLists {
    pub m_name: String,
    pub m_vulkan_allow_filter_list: Vec<AndroidDeviceFilterData>,
    pub m_vulkan_deny_filter_list: Vec<AndroidDeviceFilterData>,
    pub m_gfx_job_filter_list: Vec<VulkanGraphicsJobsDeviceFilterData>,
}

impl crate::Parse for VulkanDeviceFilterLists {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_vulkan_allow_filter_list = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_vulkan_deny_filter_list = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| AndroidDeviceFilterData::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_gfx_job_filter_list = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| VulkanGraphicsJobsDeviceFilterData::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(VulkanDeviceFilterLists {
            m_name,
            m_vulkan_allow_filter_list,
            m_vulkan_deny_filter_list,
            m_gfx_job_filter_list,
        })
    }
}

impl crate::ToValue for VulkanDeviceFilterLists {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_VulkanAllowFilterList".to_string(),
                self.m_vulkan_allow_filter_list.to_value(),
            )),
            Some((
                "m_VulkanDenyFilterList".to_string(),
                self.m_vulkan_deny_filter_list.to_value(),
            )),
            Some((
                "m_GfxJobFilterList".to_string(),
                self.m_gfx_job_filter_list.to_value(),
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
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_VulkanAllowFilterList" => {
                    (self.m_vulkan_allow_filter_list).inspect_value(path, offset, limit)
                }
                "m_VulkanDenyFilterList" => {
                    (self.m_vulkan_deny_filter_list).inspect_value(path, offset, limit)
                }
                "m_GfxJobFilterList" => {
                    (self.m_gfx_job_filter_list).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_vulkan_allow_filter_list).inspect_value(path, offset, limit))
                .map(|v| ("m_VulkanAllowFilterList".to_string(), v)),
            ((self.m_vulkan_deny_filter_list).inspect_value(path, offset, limit))
                .map(|v| ("m_VulkanDenyFilterList".to_string(), v)),
            ((self.m_gfx_job_filter_list).inspect_value(path, offset, limit))
                .map(|v| ("m_GfxJobFilterList".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
