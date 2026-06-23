//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::d3_d12_device_filter_data::D3D12DeviceFilterData;
use crate::subclasses::d3_d12_graphics_jobs_device_filter_data::D3D12GraphicsJobsDeviceFilterData;

#[derive(Debug, Clone)]
pub struct D3D12DeviceFilterLists {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_name: String,
    pub m_allow_filter_list: Vec<D3D12DeviceFilterData>,
    pub m_deny_filter_list: Vec<D3D12DeviceFilterData>,
    pub m_graphics_jobs_filter_list: Vec<D3D12GraphicsJobsDeviceFilterData>,
}

impl crate::Parse for D3D12DeviceFilterLists {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_allow_filter_list = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { D3D12DeviceFilterData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_deny_filter_list = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { D3D12DeviceFilterData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_graphics_jobs_filter_list = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { D3D12GraphicsJobsDeviceFilterData::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(D3D12DeviceFilterLists {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_name,
            m_allow_filter_list,
            m_deny_filter_list,
            m_graphics_jobs_filter_list,
        })
    }
}
