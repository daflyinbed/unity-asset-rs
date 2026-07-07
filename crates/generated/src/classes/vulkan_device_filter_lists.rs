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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_vulkan_allow_filter_list = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_vulkan_deny_filter_list = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AndroidDeviceFilterData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_gfx_job_filter_list = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VulkanGraphicsJobsDeviceFilterData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(VulkanDeviceFilterLists {
            m_name,
            m_vulkan_allow_filter_list,
            m_vulkan_deny_filter_list,
            m_gfx_job_filter_list,
        })
    }
}
